use std::error::Error;
use std::fmt::{Display, Error as FmtError, Formatter};
use std::string::FromUtf8Error;

use protocol_common::error::EnumValueOutOfRangeError;

#[derive(Copy, Clone, Debug)]
pub enum FieldName {
	Name(&'static str),
	Index(usize),
	None,
}

#[derive(Copy, Clone, Debug)]
pub struct FieldSpec {
	pub field: FieldName,
	pub ty: Option<&'static str>,
}

#[derive(Debug)]
pub enum SerializeErrorType {
	ArrayTooLarge,
}

#[derive(Debug)]
pub enum DeserializeErrorType {
	UnexpectedEndOfMessage,
	Utf8Error(FromUtf8Error),
	InvalidEnumValue(u8),
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

impl Display for SerializeError {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
		unimplemented!()
	}
}

impl Display for DeserializeError {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
		unimplemented!()
	}
}

impl Error for SerializeError {}

impl Error for DeserializeError {}

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
			ty: DeserializeErrorType::InvalidEnumValue(e.0),
			trace: vec![],
		}
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
