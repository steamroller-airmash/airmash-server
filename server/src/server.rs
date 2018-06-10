//! Loop for accepting new connections
//! and passing on all network packets

use types::*;
use types::event::*;

use std::fmt::Debug;
use std::net::ToSocketAddrs;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use std::io::Write;

use futures::{Future, Stream};
use websocket::server::async::Server;
use websocket::server::InvalidConnection;
use websocket::OwnedMessage;

use tokio_core::reactor::Core;

const RESPONSE_STR: &'static [u8] = b"\
418 IM_A_TEAPOT\n\
Content-Type: text/html\n\
\n\
<body>\
	<img src=\"https://upload.wikimedia.org/wikipedia/commons/thumb/4/44/Black_tea_pot_cropped.jpg/330px-Black_tea_pot_cropped.jpg\"\
</body>";
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
			info!(
				target: "server",
				"A client failed to connect with error: {}",
				e.error
			);

			if let Some(mut stream) = e.stream {
				// Make a best-effort attempt to
				// send a response, if this fails
				// we ignore it
				stream.write_all(RESPONSE_STR).err();
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

			let f = upgrade.accept().and_then({
				let channel = channel.clone();
				move |(s, _)| {
					info!(
							target: "server",
							"Created new connection with id {} and addr {}",
							id.0, addr
						);

					let (sink, stream) = s.split();

					channel.send(ConnectionEvent::ConnectionOpen(ConnectionOpen {
							conn: id,
							sink: Mutex::new(Some(sink))
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
								debug!(
                                    target: "server",
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
							target: "server",
							"Connection {:?} closed with error: {}",
							id, e
						);

						channel
							.send(ConnectionEvent::ConnectionClose(ConnectionClose {
								conn: id,
							}))
							.map_err(|e| error!(target: "server", "Channel send error: {}", e))
							.unwrap();
					}
				}).map({
					let channel = channel.clone();
					move |_| {
						info!(
							target: "server",
							"Connection {:?} closed",
							id
						);

						channel
							.send(ConnectionEvent::ConnectionClose(ConnectionClose {
								conn: id,
							}))
							.map_err(|e| error!(target: "server", "Channel send error: {}", e))
							.unwrap();
					}
				})
				.or_else(|_| -> Result<(), ()> { Ok(()) }),
			);
			Ok(())
		});

	reactor.run(f).unwrap();
}
