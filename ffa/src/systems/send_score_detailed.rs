use specs::*;

use crate::components::TotalDamage;

use airmash_server::component::counter::*;
use airmash_server::component::event::ScoreDetailedEvent;
use airmash_server::system_info;
use airmash_server::utils::{EventHandler, EventHandlerTypeProvider};
use airmash_server::*;

use airmash_server::protocol::server::{ScoreDetailedFFA, ScoreDetailedFFAEntry};

#[derive(Default)]
pub struct SendScoreDetailed;

#[derive(SystemData)]
pub struct SendScoreDetailedData<'a> {
    entities: Entities<'a>,
    conns: Read<'a, Connections>,

    damage: ReadStorage<'a, TotalDamage>,
    level: ReadStorage<'a, Level>,
    score: ReadStorage<'a, Score>,
    kills: ReadStorage<'a, TotalKills>,
    deaths: ReadStorage<'a, TotalDeaths>,
    ping: ReadStorage<'a, PlayerPing>,
}

impl EventHandlerTypeProvider for SendScoreDetailed {
    type Event = ScoreDetailedEvent;
}

impl<'a> EventHandler<'a> for SendScoreDetailed {
    type SystemData = SendScoreDetailedData<'a>;

    fn on_event(&mut self, evt: &ScoreDetailedEvent, data: &mut Self::SystemData) {
        let conn = evt.0;

        // Check if the packet actually came from a player
        if data.conns.associated_player(conn).is_none() {
            return;
        }

        let mut entries = (
            &*data.entities,
            &data.level,
            &data.score,
            &data.kills,
            &data.deaths,
            &data.damage,
            &data.ping,
        )
            .join()
            .map(
                |(ent, level, score, kills, deaths, damage, ping)| ScoreDetailedFFAEntry {
                    id: ent.into(),
                    level: *level,
                    score: *score,
                    kills: kills.0 as u16,
                    deaths: deaths.0 as u16,
                    damage: damage.0.inner(),
                    ping: ping.0 as u16,
                },
            )
            .collect::<Vec<_>>();

        entries.sort_by(|a, b| a.score.cmp(&b.score));
        // Avoid going over the capacity of Array
        entries.truncate(0xFFFF);

        data.conns
            .send_to(conn, ScoreDetailedFFA { scores: entries });
    }
}

system_info! {
    impl SystemInfo for SendScoreDetailed {
        type Dependencies = super::AddDamage;
    }
}
