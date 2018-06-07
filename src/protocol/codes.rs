
pub mod client {
    pub const LOGIN:   u8 = 0;
    pub const BACKUP:  u8 = 1;
    pub const HORIZON: u8 = 2;
    pub const ACK:     u8 = 5;
    pub const PONG:    u8 = 6;
    pub const KEY:     u8 = 10;
    pub const COMMAND: u8 = 11;
    pub const SCORE_DETAILED: u8 = 12;
    pub const CHAT:    u8 = 20;
    pub const WHISPER: u8 = 21;
    pub const SAY:     u8 = 22;
    pub const TEAMCHAT:u8 = 23;
    pub const VOTEMUTE:u8 = 25;
    pub const LOCALPING: u8 = 255;
}

pub mod server {
    pub const LOGIN: u8 = 0;
    pub const BACKUP: u8 = 1;
    pub const PING: u8 = 5;
    pub const PING_RESULT: u8 = 6;
    pub const ACK: u8 = 7;
    pub const ERROR: u8 = 8;
    pub const COMMAND_REPLY: u8 = 9;
    pub const PLAYER_NEW: u8 = 10;
    pub const PLAYER_LEAVE: u8 = 11;
    pub const PLAYER_UPDATE: u8 = 12;
    pub const PLAYER_FIRE: u8 = 13;
    pub const PLAYER_HIT: u8 = 14;
    pub const PLAYER_RESPAWN: u8 = 15;
    pub const PLAYER_FLAG: u8 = 16;
    pub const PLAYER_KILL: u8 = 17;
    pub const PLAYER_UPGRADE: u8 = 18;
    pub const PLAYER_TYPE: u8 = 19;
    pub const PLAYER_POWERUP: u8 = 20;
    pub const PLAYER_LEVEL: u8 = 21;
    pub const PLAYER_RETEAM: u8 = 22;
    pub const GAME_FLAG: u8 = 30;
    pub const GAME_SPECTATE: u8 = 31;
    pub const GAME_PLAYERSALIVE: u8 = 32;
    pub const GAME_FIREWALL: u8 = 33;
    pub const EVENT_REPEL: u8 = 40;
    pub const EVENT_BOOST: u8 = 41;
    pub const EVENT_BOUNCE: u8 = 42;
    pub const EVENT_STEALTH: u8 = 43;
    pub const EVENT_LEAVEHORIZON: u8 = 44;
    pub const MOB_UPDATE: u8 = 60;
    pub const MOB_UPDATE_STATIONARY: u8 = 61;
    pub const MOB_DESPAWN: u8 = 62;
    pub const MOB_DESPAWN_COORDS: u8 = 63;
    pub const CHAT_PUBLIC: u8 = 70;
    pub const CHAT_TEAM: u8 = 71;
    pub const CHAT_SAY: u8 = 72;
    pub const CHAT_WHISPER: u8 = 73;
    pub const CHAT_VOTEMUTEPASSED: u8 = 78;
    pub const CHAT_VOTEMUTED: u8 = 79;
    pub const SCORE_UPDATE: u8 = 80;
    pub const SCORE_BOARD: u8 = 81;
    pub const SCORE_DETAILED_FFA: u8 = 82;
    pub const SCORE_DETAILED_CTF: u8 = 83;
    pub const SCORE_DETAILED_BTR: u8 = 84;
    pub const SERVER_MESSAGE: u8 = 90;
    pub const SERVER_CUSTOM: u8 = 91;
}
