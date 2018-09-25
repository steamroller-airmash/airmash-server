use specs::*;

use components::TotalDamage;

use airmash_server::component::channel::*;
use airmash_server::component::counter::*;
use airmash_server::*;

use airmash_server::protocol::server::{ScoreDetailedFFA, ScoreDetailedFFAEntry};

#[derive(Default)]
pub struct SendScoreDetailed {
    reader: Option<OnScoreDetailedReader>,
}

#[derive(SystemData)]
pub struct SendScoreDetailedData<'a> {
    entities: Entities<'a>,
    channel: Read<'a, OnScoreDetailed>,
    conns: Read<'a, Connections>,

    damage: ReadStorage<'a, TotalDamage>,
    level: ReadStorage<'a, Level>,
    score: ReadStorage<'a, Score>,
    kills: ReadStorage<'a, TotalKills>,
    deaths: ReadStorage<'a, TotalDeaths>,
    ping: ReadStorage<'a, PlayerPing>,
}

impl<'a> System<'a> for SendScoreDetailed {
    type SystemData = SendScoreDetailedData<'a>;

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.reader = Some(res.fetch_mut::<OnScoreDetailed>().register_reader());
    }

    fn run(&mut self, data: Self::SystemData) {
        for evt in data.channel.read(self.reader.as_mut().unwrap()) {
            let conn = evt.0;

            // Check if the packet actually came from a player
            if data.conns.associated_player(conn).is_none() {
                continue;
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
                ).collect::<Vec<_>>();

            entries.sort_by(|a, b| a.score.cmp(&b.score));
            // Avoid going over the capacity of Array
            entries.truncate(0xFFFF);

            data.conns
                .send_to(conn, ScoreDetailedFFA { scores: entries });
        }
    }
}

impl SystemInfo for SendScoreDetailed {
    type Dependencies = super::AddDamage;

    fn name() -> &'static str {
        concat!(module_path!(), "::", line!())
    }

    fn new() -> Self {
        Self::default()
    }
}
