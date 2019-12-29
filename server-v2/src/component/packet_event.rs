//! Note:
//! Normally this should all go in events.rs; however,
//! this file has a lot more implementation stuff for
//! making PacketEvent cleaner so I've chosen to have
//! it in its own file.

use crate::types::*;
use std::time::Instant;

#[derive(Copy, Clone, Debug)]
pub struct PacketEvent<T> {
	pub data: T,
	pub received: Instant,
	pub conn: ConnectionId,
}

impl<T> PacketEvent<T> {
	pub fn new(conn: ConnectionId, data: T, received: Instant) -> Self {
		Self {
			conn,
			data,
			received,
		}
	}
}
