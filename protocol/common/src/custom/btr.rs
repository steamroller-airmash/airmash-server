use crate::enums::FlagCode;
use std::time::Duration;

use super::utils::*;

/// Serde serialization declaration for BTR [`ServerCustom`][0]
/// data.
///
/// This struct will serialize from/deserialize to the JSON
/// representation used in the `data` field of `ServerCustom`.
///
/// # Serialization Notes
/// - If the server sends an invalid flag code it will be
///   deserialized as [`FlagCode::UnitedNations`][1]
/// - `duration` is only encoded at the resolution of seconds.
///
/// [0]: ../packets/client/struct.ServerCustom.html
/// [1]: ../enum.FlagCode.html#variant.UnitedNations
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BTRData {
	#[serde(rename = "p")]
	pub player: String,
	#[serde(rename = "b")]
	pub bounty: u32,
	#[serde(rename = "f")]
	#[serde(with = "flag_code")]
	pub flag: FlagCode,
	#[serde(rename = "k")]
	pub kills: u32,
	#[serde(rename = "t")]
	#[serde(with = "duration")]
	pub duration: Duration,
}
