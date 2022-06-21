use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};
use std::{env, fs};

const PROBE: &str = r#"
  #![feature(specialization)]

  use std::fmt;

  struct DebugAny<T>(T);

  impl<T> fmt::Debug for DebugAny<T> {
    default fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      f.write_str("..")
    }
  }

  impl<T> fmt::Debug for DebugAny<T>
  where
    T: fmt::Debug
  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      self.0.fmt(f)
    }
  }
"#;

fn compile_probe() -> Option<ExitStatus> {
  let rustc = env::var_os("RUSTC")?;
  let outdir = env::var_os("OUT_DIR")?;
  let probe = Path::new(&outdir).join("probe.rs");
  fs::write(&probe, PROBE).ok()?;

  Command::new(rustc)
    .stderr(Stdio::null())
    .arg("--crate-name=anymap_probe")
    .arg("--crate-type=lib")
    .arg("--emit=metadata")
    .arg("--out-dir")
    .arg(outdir)
    .arg(probe)
    .status()
    .ok()
}

fn main() {
  match compile_probe() {
    Some(status) if status.success() => println!("cargo:rustc-cfg=anydebug"),
    _ => (),
  }
}
