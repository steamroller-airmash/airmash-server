use std::str::Utf8Error;

/// All the errors that can occur
/// when serializing or deserializing
/// messages within the airmash protocol.
#[derive(Debug, Clone)]
pub enum SerError {
	Utf8Error(Utf8Error),
	ArrayLengthTooBig,
}

#[derive(Debug, Clone, Copy)]
pub enum DeError {
	Eof,
	Utf8Error(Utf8Error),
	InvalidPacketType,
	TrailingBytes,
	InvalidPlaneType(u8),
	InvalidFlag(u16),
	InvalidLevelType(u8),
	InvalidMobType(u8),
	InvalidPlayerStatus(u8),
	InvalidKeyCode(u8),
	EntityMayNotBeDeserialized,
	InvalidEnumValue(&'static str, u64),
}
