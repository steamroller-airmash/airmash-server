use std::time::Duration;

pub trait ToClock {
    fn to_clock(&self) -> u32;
}

impl ToClock for Duration {
    // Unit is hundredths of a millisecond. (1/1e5)
    fn to_clock(&self) -> u32 {
        ((self.as_secs() * 1_000_000) as u32 + self.subsec_micros()) / 10
    }
}
