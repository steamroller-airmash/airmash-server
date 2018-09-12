use std::error::Error;
use std::fmt::{Display, Error as FmtError, Formatter};
use std::string::FromUtf8Error;

use protocol_common::error::EnumValueOutOfRangeError;

#[derive(Copy, Clone, Debug)]
pub enum FieldName {
	Name(&'static str),
	Index(usize),
}

#[derive(Copy, Clone, Debug)]
pub struct FieldSpec {
	pub field: FieldName,
	pub ty: Option<&'static str>,
}

#[derive(Debug)]
pub enum SerializeErrorType {
	ArrayTooLarge(usize),
	InvalidFlagId(u16),
}

#[derive(Debug)]
pub enum DeserializeErrorType {
	UnexpectedEndOfMessage,
	Utf8Error(FromUtf8Error),
	InvalidEnumValue(usize),
}

#[derive(Debug)]
pub struct SerializeError {
	pub ty: SerializeErrorType,
	pub trace: Vec<FieldSpec>,
}

#[derive(Debug)]
pub struct DeserializeError {
	pub ty: DeserializeErrorType,
	pub trace: Vec<FieldSpec>,
}

impl Display for SerializeErrorType {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
		use SerializeErrorType::*;
		match self {
			ArrayTooLarge(size) => write!(
				fmt,
				"Array too large. This type of array can have at most {size} elements.",
				size = size
			),
			InvalidFlagId(id) => write!(
				fmt,
				"Flags may only take on ids less than 255. Flag had an id of {id}.",
				id = id
			),
		}
	}
}

impl Display for DeserializeErrorType {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
		use DeserializeErrorType::*;

		match self {
			UnexpectedEndOfMessage => {
				write!(
					fmt,
					"Reached the end of the buffer before deserialization was complete!"
				)
			},
			Utf8Error(e) => {
				write!(
					fmt,
					"A string contained invalid UTF-8. Inner error: {}",
					e
				)
			},
			InvalidEnumValue(v) => {
				write!(
					fmt,
					"Attempted to deserialize an enum with an invalid value. {} does not deserialize to an enum case.",
					v
				)
			}
		}
	}
}

fn display_trace(trace: &[FieldSpec], fmt: &mut Formatter) -> Result<(), FmtError> {
	for spec in trace.iter().rev() {
		match spec.field {
			FieldName::Name(field) => writeln!(
				fmt,
				"\tat {ty}::{field},",
				ty = spec.ty.unwrap_or("<unnamed>"),
				field = field
			)?,
			FieldName::Index(i) => writeln!(fmt, "\tat index ${idx},", idx = i)?,
		}
	}

	Ok(())
}

impl Display for SerializeError {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
		writeln!(fmt, "{}", self.ty)?;
		display_trace(&self.trace, fmt)
	}
}

impl Display for DeserializeError {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
		writeln!(fmt, "{}", self.ty)?;
		display_trace(&self.trace, fmt)
	}
}

impl Error for SerializeError {}

impl Error for DeserializeError {
	fn cause(&self) -> Option<&Error> {
		use DeserializeErrorType::*;
		match self.ty {
			Utf8Error(ref e) => Some(e),
			_ => None,
		}
	}
}

impl From<FromUtf8Error> for DeserializeError {
	fn from(e: FromUtf8Error) -> Self {
		Self {
			ty: DeserializeErrorType::Utf8Error(e),
			trace: vec![],
		}
	}
}

impl From<EnumValueOutOfRangeError<u8>> for DeserializeError {
	fn from(e: EnumValueOutOfRangeError<u8>) -> Self {
		Self {
			ty: DeserializeErrorType::InvalidEnumValue(e.0 as usize),
			trace: vec![],
		}
	}
}

impl From<EnumValueOutOfRangeError<u16>> for DeserializeError {
	fn from(e: EnumValueOutOfRangeError<u16>) -> Self {
		Self {
			ty: DeserializeErrorType::InvalidEnumValue(e.0 as usize),
			trace: vec![],
		}
	}
}

impl From<!> for DeserializeError {
	fn from(never: !) -> Self {
		never
	}
}

impl From<!> for SerializeError {
	fn from(never: !) -> Self {
		never
	}
}

fn append_to<T>(mut v: Vec<T>, elem: T) -> Vec<T> {
	v.push(elem);
	v
}

pub trait ChainError<T> {
	fn chain(self, elem: T) -> Self;
}

impl<T, E, U> ChainError<U> for Result<T, E>
where
	E: ChainError<U>,
{
	fn chain(self, elem: U) -> Self {
		self.map_err(move |e| e.chain(elem))
	}
}

impl ChainError<FieldSpec> for SerializeError {
	fn chain(self, elem: FieldSpec) -> Self {
		Self {
			trace: append_to(self.trace, elem),
			..self
		}
	}
}

impl ChainError<FieldSpec> for DeserializeError {
	fn chain(self, elem: FieldSpec) -> Self {
		Self {
			trace: append_to(self.trace, elem),
			..self
		}
	}
}
