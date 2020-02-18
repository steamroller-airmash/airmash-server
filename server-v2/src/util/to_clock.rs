use std::time::Duration;

pub trait ToClock {
    fn to_clock(&self) -> u32;
}

impl ToClock for Duration {
    // Unit is hundredths of a millisecond. (1/1e5)
    fn to_clock(&self) -> u32 {
        self.as_secs() as u32 * 100_000 + self.subsec_micros() / 10
    }
}
