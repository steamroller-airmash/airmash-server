use std::fmt::Display;

use itertools::Itertools;
use smallvec::SmallVec;

pub(crate) trait StringEscape {
  fn escaped(&self) -> AsciiEscapedString;
}

impl StringEscape for [u8] {
  fn escaped(&self) -> AsciiEscapedString {
    AsciiEscapedString(self)
  }
}

pub(crate) struct AsciiEscapedString<'a>(&'a [u8]);

impl<'a> Display for AsciiEscapedString<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut bytes = self.0;

    loop {
      if let Some(len) = bytes
        .iter()
        .find_position(|&c| match c {
          b'"' | b'\'' | b'\\' => false,
          0x20..=0x7E => true,
          _ => false,
        })
        .map(|x| x.0)
      {
        f.write_str(unsafe { std::str::from_utf8_unchecked(&bytes[..len]) })?;
        bytes = &bytes[len..];
      }

      let next = match bytes.split_first() {
        Some((next, rest)) => {
          bytes = rest;
          *next
        }
        None => break,
      };

      let temp: SmallVec<[u8; 4]> = std::ascii::escape_default(next).collect();
      f.write_str(unsafe { std::str::from_utf8_unchecked(&temp) })?;
    }

    Ok(())
  }
}
