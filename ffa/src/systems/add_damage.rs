use specs::*;

use components::TotalDamage;

use airmash_server::component::channel::*;
use airmash_server::systems::handlers::game::timer::LoginHandler;
use airmash_server::*;

#[derive(Default)]
pub struct AddDamage {
    reader: Option<OnPlayerJoinReader>,
}

#[derive(SystemData)]
pub struct AddDamageData<'a> {
    channel: Read<'a, OnPlayerJoin>,

    damage: WriteStorage<'a, TotalDamage>,
}

impl<'a> System<'a> for AddDamage {
    type SystemData = AddDamageData<'a>;

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.reader = Some(res.fetch_mut::<OnPlayerJoin>().register_reader());
    }

    fn run(&mut self, mut data: Self::SystemData) {
        for evt in data.channel.read(self.reader.as_mut().unwrap()) {
            data.damage
                .insert(evt.id, TotalDamage(Health::new(0.0)))
                .unwrap();
        }
    }
}

impl SystemInfo for AddDamage {
    type Dependencies = LoginHandler;

    fn name() -> &'static str {
        concat!(module_path!(), "::", line!())
    }

    fn new() -> Self {
        Self::default()
    }
}
