
use super::*;

/// Tests for loops in system dependencies
#[test]
fn test_no_loops() {
	let mut config = AirmashServerConfig::new_no_gamemode("0.0.0.0:3501").with_engine();
  config.builder = systems::register(&mut config.world, config.builder);

	config.builder.build();
}
