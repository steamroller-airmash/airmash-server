use specs::*;

use std::collections::VecDeque;
use std::time::Instant;

#[derive(Copy, Clone, Component, Default, Debug)]
pub struct Ping(pub f32);

#[derive(Copy, Clone, Debug)]
pub struct PingFrame {
	pub sent: Instant,
	pub idx: u32,
}

#[derive(Clone, Default, Debug, Component)]
pub struct PingData {
	pub frames: VecDeque<PingFrame>,
	pub idx: u32,
}

impl Ping {
	pub fn as_secs(&self) -> f32 {
		self.0 * 1000.0
	}
	pub fn as_millis(&self) -> f32 {
		self.0
	}
}

impl PingData {
	pub fn new_ping(&mut self, now: Instant) -> PingFrame {
		let frame = PingFrame {
			idx: self.idx,
			sent: now,
		};

		self.idx += 1;

		self.frames.push_back(frame);
		frame
	}

	pub fn receive_ping(&mut self, idx: u32, now: Instant) -> Option<Ping> {
		let i = self.frames.iter().position(|&frame| frame.idx == idx);

		match i {
			None => None,
			Some(i) => {
				let ping = self.frames.drain(0..i + 1).last().unwrap();

				let dur = now - ping.sent;

				Some(Ping(
					(dur.as_secs() * 1_000_000 + dur.subsec_micros() as u64) as f32 * 1e-3,
				))
			}
		}
	}
}
