use airmash_server::resource::Config;
use miette::{Diagnostic, NamedSource, SourceOffset};
use serde_deserialize_over::DeserializeOver;
use std::io::Read;
use std::path::Path;
use std::{env, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
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

fn validate_one(path: &Path) -> Result<(), ValidationError> {
  let mut file = std::fs::File::open(&path).map_err(|e| ValidationError::FileIoError {
    inner: e,
    filename: path.to_owned(),
  })?;

  let mut data = String::new();
  file
    .read_to_string(&mut data)
    .map_err(|e| ValidationError::FileIoError {
      inner: e,
      filename: path.to_owned(),
    })?;

  let mut config = Config::default();
  let mut track = serde_path_to_error::Track::new();

  let mut jd = serde_json::Deserializer::from_str(&data);
  let de = serde_path_to_error::Deserializer::new(&mut jd, &mut track);

  if let Err(e) = config.deserialize_over(de) {
    return Err(ValidationError::ValidationError {
      span: SourceOffset::from_location(&data, e.line(), e.column()),
      source: NamedSource::new(path.display().to_string(), data),
      path: track.path(),
      error: e,
    });
  }

  Ok(())
}

fn main() {
  let mut count = 0;
  let mut failures = Vec::new();

  for arg in env::args_os().skip(1) {
    count += 1;
    let path = PathBuf::from(arg);
    if let Err(e) = validate_one(&path) {
      let report = miette::Report::from(e);
      failures.push(path);

      eprintln!("{:?}", report);
    }
  }

  if count == 0 {
    eprintln!("WARNING: No input files supplied!");
    eprintln!("USAGE: validate-config <config-file>...");
  }

  if !failures.is_empty() {
    eprintln!("Error: Validation failed for {} files:", failures.len());

    for failure in &failures {
      eprintln!("  - {}", failure.display());
    }

    std::process::exit(1);
  }
}
