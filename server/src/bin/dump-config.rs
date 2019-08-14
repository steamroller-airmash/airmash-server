use airmash_server::types::Config;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	let config = Config::default();
	let json = serde_json::to_string_pretty(&config)?;

	println!("{}", json);

	Ok(())
}
