use specs::*;

use component::*;

use server::component::channel::*;
use server::*;

pub struct DropOnRespawn {
    pub reader: Option<OnCommandReader>,
}

#[derive(SystemData)]
pub struct DropOnRespawnData<'a> {
    pub channel: Write<'a, OnFlag>,
    pub commands: Read<'a, OnCommand>,
    pub conns: Read<'a, Connections>,
    pub entities: Entities<'a>,

    pub carrier: WriteStorage<'a, FlagCarrier>,

    pub isflag: ReadStorage<'a, IsFlag>,
}

impl<'a> System<'a> for DropOnRespawn {
    type SystemData = DropOnRespawnData<'a>;

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.reader = Some(res.fetch_mut::<OnCommand>().register_reader());
    }

    fn run(&mut self, mut data: Self::SystemData) {
        let mut channel = data.channel;

        for (id, packet) in data.commands.read(self.reader.as_mut().unwrap()) {
            let player = match data.conns.associated_player(*id) {
                Some(p) => p,
                None => continue,
            };

            if packet.com != "respawn" {
                continue;
            }

            (&*data.entities, &mut data.carrier, &data.isflag)
                .join()
                .filter(|(_, carrier, _)| carrier.0.is_some())
                .filter(|(_, carrier, _)| carrier.0.unwrap() == player)
                .for_each(|(ent, carrier, _)| {
                    channel.single_write(FlagEvent {
                        ty: FlagEventType::Drop,
                        player: Some(player),
                        flag: ent,
                    });

                    carrier.0 = None;
                });
        }
    }
}

impl SystemInfo for DropOnRespawn {
    type Dependencies = ();

    fn name() -> &'static str {
        concat!(module_path!(), "::", line!())
    }

    fn new() -> Self {
        Self { reader: None }
    }
}
