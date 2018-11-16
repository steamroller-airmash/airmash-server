extern crate airmash_server;

use airmash_server::AirmashServer;

#[test]
fn no_system_dependency_loops() {
	AirmashServer::new("0.0.0.0:3501")
		.with_engine()
		.test_build_systems();
}
