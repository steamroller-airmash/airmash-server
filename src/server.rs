//! Loop for accepting new connections
//! and passing on all network packets

use types::*;

use std::fmt::Debug;
use std::net::ToSocketAddrs;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

use futures::{Future, Stream};
use websocket::OwnedMessage;
use websocket::server::async::Server;
use websocket::server::InvalidConnection;

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
        .map_err(|InvalidConnection { error, .. }| {
            info!(
				target: "server",
				"A client failed to connect with error: {}",
				error
			);

            ()
        })
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
                }),
            );
            Ok(())
        });

    reactor.run(f).unwrap();
}
