use server_prototypes::GamePrototype;

fn main() {
  let prototype = GamePrototype::default();
  let stdout = std::io::stdout().lock();

  serde_json::to_writer_pretty(stdout, &prototype).unwrap();
}
