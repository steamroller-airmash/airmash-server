

/*
extern crate cadence;

use std::net::UdpSocket;

pub use self::cadence::prelude::*;
use self::cadence::{
	StatsdClient, 
	QueuingMetricSink,
	BufferedUdpMetricSink,
	DEFAULT_PORT
};

pub type MetricsHandler = StatsdClient;

pub fn handler() -> MetricsHandler {
	let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
	socket.set_nonblocking(true).unwrap();

	let host = ("127.0.0.1", DEFAULT_PORT);
	let udp_sink = BufferedUdpMetricSink::from(host, socket).unwrap();
	let sink = QueuingMetricSink::from(udp_sink);
	StatsdClient::from_sink("airmash", sink)
}
*/

use std::io::{Write, Error};
use std::fs::File;
use std::sync::{Mutex, Arc};

use std::time::Duration;

#[derive(Clone)]
pub struct MetricsHandler {
	file: Arc<Mutex<File>>
}

impl MetricsHandler {
	pub fn time_duration(&self, tag: &str, d: Duration) -> Result<(), Error> {
		writeln!(
			&mut *self.file.lock().unwrap(),
			"{}: {}.{}",
			tag,
			d.as_secs() * 1000 + (d.subsec_millis() as u64),
			d.subsec_micros()
		)
	}

	pub fn count(&self, tag: &str, d: i64) -> Result<(), Error> {
		writeln!(
			&mut *self.file.lock().unwrap(),
			"{}: {}",
			tag,
			d
		)
	}
}

pub fn handler() -> MetricsHandler {
	let file = File::create("logs.txt").unwrap();

	MetricsHandler{
		file: Arc::new(Mutex::new(file))
	}
}


