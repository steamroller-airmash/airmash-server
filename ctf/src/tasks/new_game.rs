use specs::{Entities, Join, Read, ReadExpect, ReadStorage, Write, WriteStorage};

use crate::server::component::counter::{Earnings, PlayersGame, TotalDeaths, TotalKills};
use crate::server::component::flag::IsPlayer;
use crate::server::protocol::server::ScoreUpdate;
use crate::server::protocol::Score;
use crate::server::task::TaskData;
use crate::server::types::systemdata::SendToPlayer;
use crate::server::types::{Config, Health, Team, Upgrades};

use crate::component::{GameActive, GameStartEvent, OnGameStart};

use std::time::Duration;

/// Carries out the following tasks:
///  - Award 1000 bounty to all members of the winning team
///  - Set missile damage for all planes to 0
///  - Display the win messages
///  - Display countdown to new game messages
///  - Reset flags to their proper places
///  - Unset GameActive for the duration of the pause
///  - Reshuffle the teams for the next match
///  - Respawn all players
pub async fn new_game(mut data: TaskData, winner: Team) {
	// Initial tasks to be done right when the game ends
	award_bounty(&mut data, winner);
	mark_game_inactive(&mut data);
	reset_flags(&mut data);
	let saved = make_planes_invulnerable(&mut data);
	display_win_banner(&mut data, winner);

	display_message(
		&mut data,
		"New game starting in 1 minute",
		Duration::from_secs(12),
	);

	data.sleep_for(Duration::from_secs(30)).await;

	shuffle(&mut data);
	display_message(
		&mut data,
		"Game starting in 30 seconds",
		Duration::from_secs(7),
	);

	data.sleep_for(Duration::from_secs(20)).await;
	display_message(
		&mut data,
		"Game starting in 10 seconds",
		Duration::from_secs(7),
	);

	data.sleep_for(Duration::from_secs(5)).await;
	display_message(
		&mut data,
		"Game starting in 5 seconds",
		Duration::from_secs(2),
	);

	data.sleep_for(Duration::from_secs(1)).await;
	display_message(
		&mut data,
		"Game starting in 4 seconds",
		Duration::from_secs(2),
	);

	data.sleep_for(Duration::from_secs(1)).await;
	display_message(
		&mut data,
		"Game starting in 3 seconds",
		Duration::from_secs(2),
	);

	data.sleep_for(Duration::from_secs(1)).await;
	display_message(
		&mut data,
		"Game starting in 2 seconds",
		Duration::from_secs(2),
	);

	data.sleep_for(Duration::from_secs(1)).await;
	display_message(
		&mut data,
		"Game starting in a second",
		Duration::from_secs(2),
	);

	data.sleep_for(Duration::from_secs(1)).await;
	display_message(&mut data, "Game starting!", Duration::from_secs(3));

	// Final tasks to be done as the new game starts
	make_planes_vulnerable(&mut data, saved);
	mark_game_active(&mut data);
	reset_scores(&mut data);
	respawn_all_players(&mut data);

	data.write_resource::<OnGameStart, _, _>(|mut channel| {
		channel.single_write(GameStartEvent);
	});
}

fn award_bounty(data: &mut TaskData, winner: Team) {
	#[derive(SystemData)]
	struct AwardBountyData<'a> {
		entities: Entities<'a>,

		team: ReadStorage<'a, Team>,
		score: WriteStorage<'a, Score>,
		earnings: WriteStorage<'a, Earnings>,
		kills: ReadStorage<'a, TotalKills>,
		deaths: ReadStorage<'a, TotalDeaths>,
		player_mask: ReadStorage<'a, IsPlayer>,
		upgrades: ReadStorage<'a, Upgrades>,

		conns: SendToPlayer<'a>,
	}

	use crate::config::GAME_WIN_BOUNTY_BASE;

	let bounty = data.read_resource::<PlayersGame, _, _>(|players_game| {
		players_game.0.min(10) * GAME_WIN_BOUNTY_BASE.0
	});

	data.world(|world| {
		let mut data = world.system_data::<AwardBountyData>();
		let ref conns = data.conns;

		(
			&data.team,
			&mut data.score,
			&mut data.earnings,
			&*data.entities,
			&data.kills,
			&data.deaths,
			&data.upgrades,
			data.player_mask.mask(),
		)
			.join()
			.filter(|(player_team, ..)| winner == **player_team)
			.for_each(
				|(_, score, earnings, player, kills, deaths, upgrades, ..)| {
					score.0 += bounty;
					(earnings.0).0 += bounty;

					// Tell the player their score was updated
					let packet = ScoreUpdate {
						id: player.into(),
						score: *score,
						earnings: earnings.0,
						total_deaths: deaths.0,
						total_kills: kills.0,
						upgrades: upgrades.unused,
					};

					conns.send_to_player(player, packet)
				},
			);
	});
}

fn mark_game_inactive(data: &mut TaskData) {
	data.write_resource::<GameActive, _, _>(|mut active| active.0 = false);
}
fn mark_game_active(data: &mut TaskData) {
	data.write_resource::<GameActive, _, _>(|mut active| {
		active.0 = true;
	});
}

struct SavedVulnData {
	mobs: crate::server::types::config::MobInfos,
}

fn make_planes_invulnerable(data: &mut TaskData) -> SavedVulnData {
	data.write_resource::<Config, _, _>(|mut config| {
		let saved = SavedVulnData {
			mobs: config.mobs.clone(),
		};

		let iter = config
			.mobs
			.0
			.iter_mut()
			.filter_map(|x| x.1.missile.as_mut());

		for missile in iter {
			missile.damage = Health::new(0.0);
		}

		saved
	})
}
fn make_planes_vulnerable(data: &mut TaskData, saved: SavedVulnData) {
	use std::mem;

	data.write_resource::<Config, _, _>(|mut config| {
		let _ = mem::replace(&mut config.mobs, saved.mobs);
	});
}

fn reset_flags(data: &mut TaskData) {
	use crate::component::{FlagEvent, FlagEventType, Flags, OnFlag};

	#[derive(SystemData)]
	struct ResetFlagsData<'a> {
		channel: Write<'a, OnFlag>,
		flags: ReadExpect<'a, Flags>,
	}

	data.world(|world| {
		let mut data = world.system_data::<ResetFlagsData>();

		data.channel.single_write(FlagEvent {
			flag: data.flags.blue,
			player: None,
			ty: FlagEventType::Return,
		});
		data.channel.single_write(FlagEvent {
			flag: data.flags.red,
			player: None,
			ty: FlagEventType::Return,
		});
	});
}

fn shuffle(data: &mut TaskData) {
	use crate::component::Captures;
	use crate::config::{BLUE_TEAM, RED_TEAM};
	use crate::server::protocol::server::{PlayerReteam, PlayerReteamPlayer};
	use crate::server::types::systemdata::SendToAll;
	use crate::server::types::GameModeWriter;
	use crate::shuffle::{PlayerShuffleInfo, ShuffleProvider};
	use crate::CTFGameMode;

	#[derive(SystemData)]
	struct ShuffleData<'a> {
		shuffler: ReadExpect<'a, Box<dyn ShuffleProvider + Sync + Send>>,
		conns: SendToAll<'a>,
		entities: Entities<'a>,
		gamemode: GameModeWriter<'a, CTFGameMode>,

		is_player: ReadStorage<'a, IsPlayer>,
		captures: ReadStorage<'a, Captures>,
		score: ReadStorage<'a, Score>,
		teams: WriteStorage<'a, Team>,
		kills: ReadStorage<'a, TotalKills>,
		deaths: ReadStorage<'a, TotalDeaths>,
	}

	data.world(|world| {
		let mut data = world.system_data::<ShuffleData>();

		let player_info = (
			&*data.entities,
			&data.teams,
			&data.score,
			&data.captures,
			&data.kills,
			&data.deaths,
			data.is_player.mask(),
		)
			.join()
			.map(
				|(id, team, score, captures, kills, deaths, ..)| PlayerShuffleInfo {
					player: id,
					team: *team,
					score: *score,
					captures: captures.0,
					kills: kills.0,
					deaths: deaths.0,
				},
			)
			.collect::<Vec<_>>();

		let swaps = data.shuffler.shuffle(player_info);

		for swap in swaps.iter() {
			data.teams.insert(swap.player, swap.new_team).unwrap();
		}

		let (red, blue) = swaps
			.iter()
			.map(|x| {
				if x.new_team == RED_TEAM {
					(1, 0)
				} else if x.new_team == BLUE_TEAM {
					(0, 1)
				} else {
					unimplemented!()
				}
			})
			.fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

		let gamemode: &mut CTFGameMode = &mut *data.gamemode;

		gamemode.redteam = red;
		gamemode.blueteam = blue;

		let swaps = swaps
			.into_iter()
			.map(|swap| PlayerReteamPlayer {
				id: swap.player.into(),
				team: swap.new_team,
			})
			.collect::<Vec<_>>();

		let packet = PlayerReteam { players: swaps };

		data.conns.send_to_all(packet);
	});
}

fn display_win_banner(data: &mut TaskData, winner: Team) {
	use crate::config::GAME_WIN_BOUNTY_BASE;
	use crate::server::protocol::server::ServerCustom;
	use crate::server::protocol::ServerCustomType;
	use crate::server::types::systemdata::SendToAll;

	#[derive(SystemData)]
	struct DisplayWinData<'a> {
		conns: SendToAll<'a>,
		players_game: Read<'a, PlayersGame>,
	}

	data.world(|world| {
		let data = world.system_data::<DisplayWinData>();

		let text = format!(
			"{{\"w\":{},\"b\":{},\"t\":{}}}",
			winner.0,
			data.players_game.0.min(10) * GAME_WIN_BOUNTY_BASE.0,
			13 // display time in seconds
		);

		let packet = ServerCustom {
			ty: ServerCustomType::CTFWin,
			data: text,
		};
		data.conns.send_to_all(packet);
	});
}

fn display_message(data: &mut TaskData, msg: &str, duration: Duration) {
	use crate::server::protocol::server::ServerMessage;
	use crate::server::protocol::ServerMessageType;
	use crate::server::types::systemdata::SendToAll;

	data.world(|world| {
		let data = world.system_data::<SendToAll>();

		let packet = ServerMessage {
			ty: ServerMessageType::TimeToGameStart,
			duration: duration.as_millis() as u32,
			text: msg.to_owned(),
		};

		data.send_to_all(packet);
	});
}

fn respawn_all_players(data: &mut TaskData) {
	use crate::server::component::channel::OnPlayerRespawn;
	use crate::server::component::event::{PlayerRespawn, PlayerRespawnPrevStatus::*};
	use crate::server::component::flag::{IsDead, IsSpectating};

	#[derive(SystemData)]
	struct RespawnAllData<'a> {
		channel: Write<'a, OnPlayerRespawn>,

		entities: Entities<'a>,
		is_player: ReadStorage<'a, IsPlayer>,
		is_spec: WriteStorage<'a, IsSpectating>,
		is_dead: ReadStorage<'a, IsDead>,
	}

	data.world(|world| {
		let mut data = world.system_data::<RespawnAllData>();

		let ref mut is_spec = data.is_spec;
		let ref is_dead = data.is_dead;

		let players = (&*data.entities, data.is_player.mask())
			.join()
			.map(|(ent, ..)| {
				let spec = is_spec.remove(ent).is_some();
				let dead = is_dead.get(ent).is_some();

				PlayerRespawn {
					player: ent,
					prev_status: if !(dead || spec) { Alive } else { Dead },
				}
			})
			.collect::<Vec<_>>();

		data.channel.iter_write(players.into_iter());
	})
}

fn reset_scores(data: &mut TaskData) {
	use crate::component::{FlagEvent, FlagEventType, Flags, GameScores, OnFlag};

	#[derive(SystemData)]
	struct ResetScoresData<'a> {
		scores: Write<'a, GameScores>,

		flags: ReadExpect<'a, Flags>,
		channel: Write<'a, OnFlag>,
	}

	data.world(|world| {
		let mut data = world.system_data::<ResetScoresData>();

		*data.scores = GameScores {
			blueteam: 0,
			redteam: 0,
		};

		// TODO: Establish what the official server does
		data.channel.single_write(FlagEvent {
			ty: FlagEventType::Return,
			flag: data.flags.red,
			player: None,
		});

		data.channel.single_write(FlagEvent {
			ty: FlagEventType::Return,
			flag: data.flags.blue,
			player: None,
		});
	})
}
