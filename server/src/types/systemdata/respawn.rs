use specs::*;
use types::*;

use protocol::server::PlayerRespawn;
use protocol::Upgrades as ProtocolUpgrades;
use protocol::{to_bytes, ServerPacket};
use OwnedMessage;

#[derive(SystemData)]
pub struct PlayerRespawner<'a> {
	pub team: ReadStorage<'a, Team>,
	pub pos: WriteStorage<'a, Position>,
	pub vel: WriteStorage<'a, Velocity>,
	pub rot: WriteStorage<'a, Rotation>,
	pub health: WriteStorage<'a, Health>,
	pub energy: WriteStorage<'a, Energy>,

	pub is_dead: WriteStorage<'a, IsDead>,
	pub is_spec: ReadStorage<'a, IsSpectating>,
	pub is_player: ReadStorage<'a, IsPlayer>,

	pub gamemode: GameModeWriter<'a, GameMode>,

	pub conns: Read<'a, Connections>,
}

impl<'a> PlayerRespawner<'a> {
	/// Respawn mutiple players, sending a PlayerRespawn
	/// packet for each player if they are not also
	/// spectating.
	///
	/// This function sets the position, velocity, rotation
	/// health, and energy to the appropriate values for
	/// respawning each player. It also removes the [`IsDead`]
	/// flag from the player entity
	pub fn respawn_players<'b, I>(&mut self, players: I)
	where
		I: IntoIterator<Item = &'b Entity>,
	{
		let gamemode = self.gamemode.get_mut();

		for &player in players {
			let team = *self.team.get(player).unwrap();
			let pos = gamemode.spawn_pos(player, team);

			*self.pos.get_mut(player).unwrap() = pos;
			*self.vel.get_mut(player).unwrap() = Velocity::default();
			*self.rot.get_mut(player).unwrap() = Rotation::default();
			*self.health.get_mut(player).unwrap() = Health::new(1.0);
			*self.energy.get_mut(player).unwrap() = Energy::new(1.0);

			self.is_dead.remove(player);

			if self.is_spec.get(player).is_none() {
				self.conns.send_to_visible(
					player,
					OwnedMessage::Binary(
						to_bytes(&ServerPacket::PlayerRespawn(PlayerRespawn {
							id: player,
							pos: *self.pos.get(player).unwrap(),
							rot: *self.rot.get(player).unwrap(),
							upgrades: ProtocolUpgrades::default(),
						})).unwrap(),
					),
				)
			}
		}
	}

	///
	pub fn respawn_player(&mut self, player: Entity) {
		self.respawn_players([player].iter())
	}
}
