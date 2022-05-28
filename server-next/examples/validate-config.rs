use airmash_server::resource::Config;
use miette::{Diagnostic, NamedSource, Result, SourceOffset};
use serde_deserialize_over::DeserializeOver;
use std::io::Read;
use std::{env, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[diagnostic()]
enum ValidationError {
  #[error("Unable to read config file `{filename}`")]
  FileIoError {
    #[source]
    inner: std::io::Error,
    filename: PathBuf,
  },

  #[error("Error while validating `{path}`")]
  ValidationError {
    #[source]
    error: serde_json::Error,
    path: serde_path_to_error::Path,

    #[source_code]
    source: NamedSource,

    #[label]
    span: SourceOffset,
  },
}

fn main() -> Result<()> {
  let mut count = 0;

  for arg in env::args_os().skip(1) {
    count += 1;
    let path = PathBuf::from(arg);

    let mut file = std::fs::File::open(&path).map_err(|e| ValidationError::FileIoError {
      inner: e,
      filename: path.clone(),
    })?;

    let mut data = String::new();
    file
      .read_to_string(&mut data)
      .map_err(|e| ValidationError::FileIoError {
        inner: e,
        filename: path.clone(),
      })?;

    let mut config = Config::default();
    let mut track = serde_path_to_error::Track::new();

    let mut jd = serde_json::Deserializer::from_str(&data);
    let de = serde_path_to_error::Deserializer::new(&mut jd, &mut track);

    if let Err(e) = config.deserialize_over(de) {
      Err(ValidationError::ValidationError {
        span: SourceOffset::from_location(&data, e.line(), e.column()),
        source: NamedSource::new(path.display().to_string(), data),
        path: track.path(),
        error: e,
      })?
    }
  }

  if count == 0 {
    eprintln!("WARNING: No input files supplied!");
    eprintln!("USAGE: validate-config <config-file>...");
  }

  Ok(())
}
