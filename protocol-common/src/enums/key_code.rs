/// The key that's had it's state changed.
/// This is only used for client -> server
/// communication.
///
/// It is used in the following packets:
/// - TODO
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum KeyCode {
	Up = 1,
	Down = 2,
	Left = 3,
	Right = 4,
	Fire = 5,
	Special = 6,
}
