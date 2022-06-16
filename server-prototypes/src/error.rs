use std::error::Error;
use std::fmt;

pub struct Path {
  segments: Vec<Segment>,
}

impl Path {
  pub fn new(segment: Segment) -> Self {
    Self {
      segments: vec![segment],
    }
  }

  pub fn push(&mut self, segment: Segment) {
    self.segments.push(segment);
  }

  pub fn with(mut self, segment: Segment) -> Self {
    self.push(segment);
    self
  }
}

impl fmt::Display for Path {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut iter = self.segments.iter();

    if let Some(seg) = iter.next() {
      seg.fmt(f)?;
    }

    for seg in iter {
      f.write_str(".")?;
      seg.fmt(f)?;
    }

    Ok(())
  }
}

impl fmt::Debug for Path {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!("\"{}\"", self))
  }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Segment {
  Field(&'static str),
  Index(usize),
}

impl fmt::Display for Segment {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Field(field) => f.write_str(field),
      Self::Index(index) => index.fmt(f),
    }
  }
}

impl From<&'static str> for Segment {
  fn from(field: &'static str) -> Self {
    Self::Field(field)
  }
}

impl From<usize> for Segment {
  fn from(index: usize) -> Self {
    Self::Index(index)
  }
}

struct StringError(String);

impl fmt::Debug for StringError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(&self.0)
  }
}

impl fmt::Display for StringError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(&self.0)
  }
}

impl Error for StringError {}

#[derive(Debug)]
pub struct ValidationError {
  path: Path,
  error: Box<dyn Error + 'static>,
}

impl ValidationError {
  pub fn new<I, E>(field: I, error: E) -> Self
  where
    I: Into<Segment>,
    E: Error + 'static,
  {
    Self {
      path: Path::new(field.into()),
      error: Box::new(error),
    }
  }

  pub fn custom<I>(field: I, message: &dyn fmt::Display) -> Self
  where
    I: Into<Segment>,
  {
    Self::new(field.into(), StringError(format!("{}", message)))
  }

  pub fn with<I>(mut self, field: I) -> Self
  where
    I: Into<Segment>,
  {
    self.path.push(field.into());
    self
  }

  pub fn path(&self) -> &Path {
    &self.path
  }
}

impl fmt::Display for ValidationError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_fmt(format_args!("Error while validating field `{}`", self.path))
  }
}

impl Error for ValidationError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    Some(&*self.error)
  }
}

pub(crate) trait ValidationExt {
  fn with<I: Into<Segment>>(self, field: I) -> Self;
}

impl<T> ValidationExt for Result<T, ValidationError> {
  fn with<I: Into<Segment>>(self, field: I) -> Self {
    self.map_err(move |e| e.with(field.into()))
  }
}
