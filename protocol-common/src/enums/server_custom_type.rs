/// Specific identifiers for server custom messages.
///
/// TODO: Reverse Engineer
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum ServerCustomType {
	/// TODO: Determine if this name is accurate
	CTFWin = 2,
}
