use std::net::SocketAddr;
use std::time::Duration;

// TODO: Validation
pub struct AirmashServerConfig {
    pub framerate: f64,
    pub port: SocketAddr,
}

impl AirmashServerConfig {
    pub fn frame_duration(&self) -> Duration {
        Duration::from_secs_f64(1.0 / self.framerate)
    }
}

impl Default for AirmashServerConfig {
    fn default() -> Self {
        unimplemented!()
    }
}
