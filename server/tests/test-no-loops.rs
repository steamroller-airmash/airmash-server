
extern crate airmash_server;

use airmash_server::AirmashServer;

#[test]
fn no_system_dependency_loops() {
	let _ = AirmashServer::new("0.0.0.0:3501").with_engine();
}
