/// Purpose unknown, doesn't appear to be
/// used in the official client.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(features = "serde", derive(Serialize, Deserialize))]
pub struct LocalPing {
	pub auth: u32,
}