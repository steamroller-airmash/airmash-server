
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



