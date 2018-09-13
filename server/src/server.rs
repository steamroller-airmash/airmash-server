//! Loop for accepting new connections
//! and passing on all network packets

use types::event::*;
use types::*;

use std::fmt::Debug;
use std::net::ToSocketAddrs;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

use futures::{Future, Stream};
use websocket::server::async::Server;
use websocket::OwnedMessage;

use hyper::header::{ContentType, Headers};
use hyper::mime::{Attr, Mime, SubLevel, TopLevel, Value};
use hyper::server::Response;
use status;

#[cfg(feature = "proxied")]
mod hyperuse {
	pub use hyper::header::{Header, HeaderFormat};
	use hyper::Error as HyperError;
	use std::fmt::{Formatter, Result as FmtResult};
	pub use std::net::IpAddr;
	use std::str;

	#[derive(Clone, Debug)]
	pub struct XForwardedFor {
		pub addrs: Vec<IpAddr>,
	}

	impl Header for XForwardedFor {
		fn header_name() -> &'static str {
			return "X-Forwarded-For";
		}

		fn parse_header(raw: &[Vec<u8>]) -> Result<Self, HyperError> {
			if raw.len() != 1 {
				return Err(HyperError::Header);
			}

			let s = match str::from_utf8(&raw[0]) {
				Ok(s) => s,
				Err(e) => return Err(HyperError::Utf8(e)),
			};

			let mut addrs = vec![];

			for s in s.split(',') {
				addrs.push(match s.parse() {
					Ok(v) => v,
					Err(_) => return Err(HyperError::Header),
				});
			}

			Ok(Self { addrs })
		}
	}

	impl HeaderFormat for XForwardedFor {
		fn fmt_header(&self, fmt: &mut Formatter) -> FmtResult {
			let strs = self
				.addrs
				.iter()
				.map(|x| x.to_string())
				.collect::<Vec<String>>();
			write!(fmt, "{}", strs.join(", "))
		}
	}
}

#[cfg(feature = "proxied")]
use self::hyperuse::*;

use tokio_core::reactor::Core;

pub fn run_acceptor<A>(addr: A, channel: Sender<ConnectionEvent>)
where
	A: ToSocketAddrs + Debug,
{
	info!(
		target: "server",
		"starting server at {:?}",
		addr
	);

	let mut reactor = Core::new().unwrap();
	let handle = reactor.handle();

	let socket = Server::bind(addr, &handle).unwrap();

	let f = socket
		.incoming()
		.map_err(|e| {
			if let Some(mut stream) = e.stream {
				let mut headers = Headers::new();
				headers.set(ContentType(Mime(
						TopLevel::Application,
						SubLevel::Json,
						vec![(Attr::Charset, Value::Utf8)]
				)));

				let response = Response::new(
					&mut stream,
					&mut headers
				);

				let status = status::generate_status_page();

				if let Err(e) = response.send(status.as_bytes()) {
					warn!(
						"Failed to respond with status page to request with error {}",
						e
					);
				}
			}
		})
		// The following two operators filter out
		// all connection errors from the stream.
		// We don't want to crash the server when
		// somebody connects directly. We end up
		// simply dropping connections when this
		// happens, causing nginx to return a 502
		// (if we are proxying with nginx for https)
		.then(|v| -> Result<_, ()> {
			match v {
				Ok(inner) => Ok(Some(inner)),
				Err(_) => Ok(None)
			}
		})
		.filter_map(|x| x)
		.for_each(move |(upgrade, addr)| {
			let id = ConnectionId::new();

			// Make a best-effort attempt to 
			// set TCP_NODELAY. If this fails,
			// then the client will just be 
			// using a less optimal stream.
			#[cfg(feature="nodelay")]
			upgrade.stream.set_nodelay(true).err();

			let origin = upgrade.origin().map(|x| x.to_owned());

			#[cfg(feature="proxied")]
			let realaddr = match upgrade.request.headers.get::<XForwardedFor>() {
				Some(v) => match v.addrs.get(0) {
					Some(v) => *v,
					None => addr.ip()
				},
				None => addr.ip()
			};

			#[cfg(not(feature="proxied"))]
			let realaddr = addr.ip();

			let f = upgrade.accept()
			.and_then({
				let channel = channel.clone();
				move |(s, _)| {
					info!(
							"Created new connection with id {} and addr {}",
							id.0, realaddr
						);

					let (sink, stream) = s.split();

					channel.send(ConnectionEvent::ConnectionOpen(ConnectionOpen {
							conn: id,
							sink: Mutex::new(Some(sink)),
							addr: realaddr,
							origin: origin
						})).map_err(|e| {
							error!(target: "server", "Channel send error: {}", e)
						})
						// Swallow error since if this errors
						// we are most likely shutting down.
						// The error will be logged anyway.
						.err();

					stream.take_while(|m| Ok(!m.is_close())).for_each({
						let channel = channel.clone();
						move |m| {
							if m != OwnedMessage::Binary(vec![5]) {
								trace!(
										target: "airmash:packet-dump",
										"{:?} sent {:?}",
										id, m
								);
							}

							channel.send(ConnectionEvent::Message(Message{
										conn: id,
										msg: m
								})).map_err(|e| {
										error!(target: "server", "Channel send error: {}", e)
								})
								// Swallow error since we logged it
								// and are probably shutting down.
								.err();
							Ok(())
						}
					})
				}
			});

			handle.spawn(
				f.map_err({
					let channel = channel.clone();
					move |e| {
						info!(
							"Connection {:?} closed with error: {}",
							id, e
						);

						channel
							.send(ConnectionEvent::ConnectionClose(ConnectionClose {
								conn: id,
							}))
							.map_err(|e| error!("Channel send error: {}", e))
							.unwrap();
					}
				}).map({
					let channel = channel.clone();
					move |_| {
						info!(
							"Connection {:?} closed",
							id
						);

						channel
							.send(ConnectionEvent::ConnectionClose(ConnectionClose {
								conn: id,
							}))
							.map_err(|e| error!("Channel send error: {}", e))
							.unwrap();
					}
				})
				.or_else(|_| -> Result<(), ()> { Ok(()) }),
			);
			Ok(())
		});

	reactor.run(f).unwrap();
}
