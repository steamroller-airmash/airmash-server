//! Loop for accepting new connections
//! and passing on all network packets

use types::event::*;
use types::*;

use std::fmt::Debug;
use std::net::{ToSocketAddrs, IpAddr, Ipv4Addr};
use std::sync::mpsc::Sender;

use status;

use ws::{self, Sender as WsSender, Handler, Handshake, Result as WsResult, Message as WsMessage, CloseCode, Request, Response};

struct MessageHandler {
	channel: Sender<ConnectionEvent>,
	sender: WsSender,
	id: ConnectionId,
	closed: bool
}

fn get_real_ip(shake: &Handshake) -> WsResult<(IpAddr, Option<String>)> {
	let default_ipaddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
	let ref req = shake.request;
	
	Ok((
		shake.remote_addr()?
			.map(|x| x.parse().unwrap_or(default_ipaddr))
			.unwrap_or(default_ipaddr),
		req.origin()?.map(|x| x.to_owned()),
	))
}

impl Handler for MessageHandler {
	fn on_shutdown(&mut self) {
		if self.closed { return };

		self.channel.send(ConnectionEvent::ConnectionClose(ConnectionClose { conn: self.id }))
			.map_err(|e| {
				error!(target: "server", "Channel send error: {}", e)
			})
			// Swallow error since if this errors
			// we are most likely shutting down.
			// The error will be logged anyway.
			.err();
	}

	fn on_open(&mut self, shake: Handshake) -> WsResult<()> {
		let (realaddr, origin) = get_real_ip(&shake)?;

		self.channel.send(ConnectionEvent::ConnectionOpen(ConnectionOpen {
			conn: self.id,
			sink: self.sender.clone(),
			addr: realaddr,
			origin: origin,
		}))
		.map_err(|e| {
			error!(target: "server", "Channel send error: {}", e)
		})
		// Swallow error since if this errors
		// we are most likely shutting down.
		// The error will be logged anyway.
		.err();

		Ok(())
	}

	fn on_message(&mut self, msg: WsMessage) -> WsResult<()> {
		self.channel.send(ConnectionEvent::Message(Message {
			conn: self.id,
			msg: msg.into_data(),
		}))
		.map_err(|e| {
			error!(target: "server", "Channel send error: {}", e)
		})
		// Swallow error since if this errors
		// we are most likely shutting down.
		// The error will be logged anyway.
		.err();

		Ok(())
	}

	fn on_close(&mut self, _: CloseCode, _: &str) {
		if self.closed { return };
		self.closed = true;

		self.channel.send(ConnectionEvent::ConnectionClose(ConnectionClose { conn: self.id }))
			.map_err(|e| {
				error!(target: "server", "Channel send error: {}", e)
			})
			// Swallow error since if this errors
			// we are most likely shutting down.
			// The error will be logged anyway.
			.err();
	}

	fn on_request(&mut self, req: &Request) -> WsResult<Response> {
		let req = Response::from_request(req);

		Ok(
			req
				.unwrap_or_else(|_| {
					let status = status::generate_status_page();

					let mut res = Response::new(
						200,
						"OK",
						status.into_bytes()
					);

					res.headers_mut().push((
						"Content-Type".to_owned(), 
						"application/json; charset=utf-8".to_owned().into_bytes()
					));
					
					res
				})
		)
	}
}

pub fn run_acceptor<A>(addr: A, channel: Sender<ConnectionEvent>)
where
	A: ToSocketAddrs + Debug,
{
	info!(
		target: "server",
		"starting server at {:?}",
		addr
	);

	let result = ws::listen(addr, move |out| MessageHandler {
		id: ConnectionId::new(),
		channel: channel.clone(),
		sender: out,
		closed: false
	});

	if let Err(e) = result {
		error!("Server failed with error {}", e);
		// TODO: Maybe force shutdown here?
	}
}
