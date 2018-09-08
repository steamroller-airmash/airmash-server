/*extern crate cadence;

use std::error::Error;
use std::net::UdpSocket;
use std::time::Duration;

pub use self::cadence::prelude::*;
use self::cadence::{BufferedUdpMetricSink, QueuingMetricSink, StatsdClient, DEFAULT_PORT};

#[derive(Clone)]
pub struct MetricsHandler(StatsdClient);

pub fn handler() -> MetricsHandler {
	let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
	socket.set_nonblocking(true).unwrap();

	let host = ("127.0.0.1", DEFAULT_PORT);
	let udp_sink = BufferedUdpMetricSink::from(host, socket).unwrap();
	let sink = QueuingMetricSink::from(udp_sink);
	MetricsHandler(StatsdClient::from_sink("airmash", sink))
}

impl MetricsHandler {
	pub fn time_duration(&self, _name: &str, _time: Duration) -> Result<(), Box<Error>> {
		//self.0.time_duration(&name.replace("::", "."), time)?;
		Ok(())
	}

	pub fn count(&self, _name: &str, _count: i64) -> Result<(), Box<Error>> {
		//self.0.count(&name.replace("::", "."), count)?;
		Ok(())
	}
}*/

#![allow(unused)]

use std::fs::File;
use std::io::{Error, Write};
use std::sync::mpsc::*;
use std::sync::{Arc, Mutex};
use std::thread;

use std::time::Duration;

enum Message {
	Msg(String),
	End,
}

#[derive(Clone)]
#[deprecated]
pub struct MetricsHandler {
	send: Arc<Mutex<Sender<Message>>>,
	thread: Arc<thread::JoinHandle<()>>,
}

impl MetricsHandler {
	pub fn time_duration(&self, tag: &str, d: Duration) -> Result<(), Error> {
		let send = self.send.lock().unwrap().clone();
		send.send(Message::Msg(format!(
			"{}: {}.{:03}",
			tag,
			d.as_secs() * 1000 + (d.subsec_millis() as u64),
			d.subsec_micros()
		))).err();
		Ok(())
	}

	pub fn count(&self, tag: &str, d: i64) -> Result<(), Error> {
		let send = self.send.lock().unwrap().clone();
		send.send(Message::Msg(format!("{}: {}", tag, d))).err();
		Ok(())
	}
}

impl Drop for MetricsHandler {
	fn drop(&mut self) {
		let send = self.send.lock().unwrap().clone();
		send.send(Message::End).err();
	}
}

pub fn handler() -> MetricsHandler {
	let (send, recv) = channel();

	let handle = thread::spawn(move || {
		//let mut file = File::create("logs.txt").unwrap();
		while let Ok(Message::Msg(_s)) = recv.recv_timeout(Duration::from_secs(3600)) {
			//writeln!(&mut file, "{}", s).err();
		}
	});

	MetricsHandler {
		send: Arc::new(Mutex::new(send)),
		thread: Arc::new(handle),
	}
}
