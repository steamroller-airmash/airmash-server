
extern crate specgen;

use specgen::*;

use std::io::Read;
use std::fs::File;
use std::env;

fn name_map(s: &str) -> String {
	if s == "type" { return "ty".to_owned(); }
	s.to_owned()
}

fn main() {
	env::set_var("RUST_BACKTRACE", "1");

	let mut bytes = vec![];
	File::open("proto.spec")
		.unwrap()
		.read_to_end(&mut bytes)
		.unwrap();
	
	let mut file = File::create(&std::env::args().collect::<Vec<String>>()[1])
		.unwrap();

	SerdeBuilder::new()
		.map_name(name_map)
		.build(&bytes, &mut file)
		.unwrap();
}
