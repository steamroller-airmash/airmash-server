extern crate airmash_server;
extern crate specs;

use airmash_server::AirmashServerConfig;

#[test]
fn no_system_dependency_loops() {
	let config = AirmashServerConfig::new_no_gamemode("0.0.0.0:3501").with_engine();

	config.builder.build();
}
