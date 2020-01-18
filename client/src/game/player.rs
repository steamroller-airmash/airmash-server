use airmash_protocol::server::*;
use airmash_protocol::*;

#[derive(Default, Debug, Clone)]
pub struct Player {
    pub name: String,
    pub flag: FlagCode,
    pub id: u16,
    pub team: Team,
    pub level: Option<u8>,
    pub plane: PlaneType,
    pub status: PlayerStatus,
    pub muted: bool,
    pub visible: bool,
    pub rank: u16,
    pub score: u32,
    pub earnings: u32,
    pub is_spec: bool,

    pub kills: u32,
    pub deaths: u32,
    pub captures: u32,

    pub pos: Position,
    pub rot: Rotation,
    pub vel: Velocity,
    pub health: Health,
    pub energy: Energy,
    pub health_regen: HealthRegen,
    pub energy_regen: EnergyRegen,

    pub flagspeed: bool,
    pub keystate: ServerKeyState,
    pub upgrades: Upgrades,
    pub unused_upgrades: u16,
}

impl Player {
    pub fn update(&mut self, packet: &PlayerUpdate) {
        self.pos = packet.pos;
        self.rot = packet.rot;
        self.vel = packet.speed;
        self.keystate = packet.keystate;
        self.upgrades = packet.upgrades;
        self.status = PlayerStatus::Alive;
    }
}
