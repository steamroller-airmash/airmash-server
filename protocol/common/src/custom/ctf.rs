use crate::Team;
use std::time::Duration;

use super::utils::*;

/// Serde serialization declaration for CTF [`ServerCustom`][0]
/// data.
///
/// This struct will serialize from/deserialize to the JSON
/// representation used in the `data` field of `ServerCustom`.
///
/// # Serialization Notes
/// - `duration` is only encoded at the resolution of seconds.
///
/// [0]: ../packets/client/struct.ServerCustom.html
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct CTFData {
	#[serde(rename = "w")]
	pub winner: Team,
	#[serde(rename = "b")]
	pub bounty: u32,
	#[serde(rename = "t")]
	#[serde(with = "duration")]
	pub duration: Duration,
}
