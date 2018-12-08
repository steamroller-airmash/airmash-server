use types::event::*;
use types::*;

use std::net::{IpAddr, Ipv4Addr, ToSocketAddrs};
use std::sync::mpsc::Sender;
use std::thread::{self, JoinHandle};

use status;

use ws::{
	Builder, CloseCode, Handler, Handshake, Message as WsMessage, Request, Response,
	Result as WsResult, Sender as WsSender, Settings,
};

struct MessageHandler {
	channel: Sender<ConnectionEvent>,
	sender: WsSender,
	id: ConnectionId,
	closed: bool,
}

impl Handler for MessageHandler {
	fn on_shutdown(&mut self) {
		if self.closed {
			return;
		}

		self.channel
			.send(ConnectionEvent::ConnectionClose(ConnectionClose {
				conn: self.id,
			}))
			.map_err(|e| error!(target: "server", "Channel send error: {}", e))
			// Swallow error since the only option is to panic
			// which isn't very useful. It's been logged anyway.
			.err();
	}

	fn on_open(&mut self, shake: Handshake) -> WsResult<()> {
		let (realaddr, origin) = get_real_ip(&shake)?;

		self.channel
			.send(ConnectionEvent::ConnectionOpen(ConnectionOpen {
				conn: self.id,
				sink: self.sender.clone(),
				addr: realaddr,
				origin: origin,
			}))
			.map_err(|e| error!(target: "server", "Channel send error: {}", e))
			// Swallow error since if this errors
			// we are most likely shutting down.
			// The error will be logged anyway.
			.err();

		Ok(())
	}

	fn on_message(&mut self, msg: WsMessage) -> WsResult<()> {
		self.channel
			.send(ConnectionEvent::Message(Message {
				conn: self.id,
				msg: msg.into_data(),
			}))
			.map_err(|e| error!(target: "server", "Channel send error: {}", e))
			// Swallow error since if this errors
			// we are most likely shutting down.
			// The error will be logged anyway.
			.err();

		Ok(())
	}

	fn on_close(&mut self, _: CloseCode, _: &str) {
		if self.closed {
			return;
		};
		self.closed = true;

		self.channel
			.send(ConnectionEvent::ConnectionClose(ConnectionClose {
				conn: self.id,
			}))
			.map_err(|e| error!(target: "server", "Channel send error: {}", e))
			// Swallow error since if this errors
			// we are most likely shutting down.
			// The error will be logged anyway.
			.err();
	}

	fn on_request(&mut self, req: &Request) -> WsResult<Response> {
		let req = Response::from_request(req);

		Ok(req.unwrap_or_else(|_| {
			let status = status::generate_status_page();

			let mut res = Response::new(200, "OK", status.into_bytes());

			res.headers_mut().push((
				"Content-Type".to_owned(),
				"application/json; charset=utf-8".to_owned().into_bytes(),
			));

			res
		}))
	}
}

fn get_real_ip(shake: &Handshake) -> WsResult<(IpAddr, Option<String>)> {
	let default_ipaddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
	let ref req = shake.request;

	Ok((
		shake
			.remote_addr()?
			.map(|x| x.parse().unwrap_or(default_ipaddr))
			.unwrap_or(default_ipaddr),
		req.origin()?.map(|x| x.to_owned()),
	))
}

fn acceptor<A>(addr: A, channel: Sender<ConnectionEvent>, max_connections: usize
) -> WsResult<()>
where
	A: ToSocketAddrs,
{
	let mut builder = Builder::new();
	builder.with_settings(Settings {
		max_connections,
		queue_size: 10,
		..Default::default()
	});

	builder
		.build(move |out| MessageHandler {
			id: ConnectionId::new(),
			channel: channel.clone(),
			sender: out,
			closed: false,
		})
		.and_then(move |ws| ws.listen(addr))
		.map(|_|())
}

pub(crate) fn spawn_acceptor<A>(
	addrs: A,
	channel: Sender<ConnectionEvent>,
	max_connections: usize,
) -> JoinHandle<WsResult<()>>
where
	A: ToSocketAddrs + Send + 'static,
{
	thread::spawn(move || {
		acceptor(addrs, channel, max_connections)
	})
}
