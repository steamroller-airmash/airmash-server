use std::time::Instant;

#[derive(Debug, Default, Copy, Clone)]
pub struct ClientUpgrades {
    pub speed: u8,
    pub defense: u8,
    pub energy: u8,
    pub missile: u8,
    pub unused: u16,
}

#[derive(Debug, Default, Clone)]
pub struct CurrentPlayer {
    pub id: u16,
    pub upgrades: ClientUpgrades,
    pub powerup_expiry: Option<Instant>,

    pub token: String,
}
