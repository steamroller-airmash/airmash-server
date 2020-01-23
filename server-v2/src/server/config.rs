use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

// TODO: Validation
pub struct AirmashServerConfig {
    pub framerate: f64,
    pub socket: SocketAddr,
}

impl AirmashServerConfig {
    pub fn frame_duration(&self) -> Duration {
        Duration::from_secs_f64(1.0 / self.framerate)
    }
}

impl Default for AirmashServerConfig {
    fn default() -> Self {
        Self {
            framerate: 60.0,
            // 0.0.0.0:3501
            socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 3501),
        }
    }
}
