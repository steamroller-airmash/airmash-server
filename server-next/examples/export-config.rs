use airmash_server::resource::Config;

fn main() {
  let config = Config::default();

  serde_json::to_writer_pretty(std::io::stdout(), &config).expect("Failed to serialize config");
  println!("");
}
