pub(crate) mod flag_code {
	use crate::enums::FlagCode;
	use serde::*;
	use std::convert::TryInto;

	pub fn serialize<S>(flag: &FlagCode, s: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		s.serialize_u32(*flag as u32)
	}

	pub fn deserialize<'de, D>(de: D) -> Result<FlagCode, D::Error>
	where
		D: Deserializer<'de>,
	{
		Ok(u8::deserialize(de)?
			.try_into()
			.unwrap_or(FlagCode::UnitedNations))
	}
}

pub(crate) mod duration {
	use serde::*;
	use std::time::Duration;

	pub fn serialize<S>(duration: &Duration, s: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		s.serialize_u64(duration.as_secs())
	}

	pub fn deserialize<'de, D>(de: D) -> Result<Duration, D::Error>
	where
		D: Deserializer<'de>,
	{
		Ok(Duration::from_secs(u64::deserialize(de)?))
	}
}
