use cadence::*;
use std::net::UdpSocket;

lazy_static! {
	pub static ref CLIENT: StatsdClient = {
		let host = ("127.0.0.1", DEFAULT_PORT);

		let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
		socket.set_nonblocking(true).unwrap();

		let udp_sink = BufferedUdpMetricSink::from(host, socket).unwrap();
		let queuing_sink = QueuingMetricSink::from(udp_sink);
		StatsdClient::from_sink("airmash", queuing_sink)
	};
}
