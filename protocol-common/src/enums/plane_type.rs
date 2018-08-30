/// Used to indicate the type of plane
/// that the packet refers to.
///
/// Used in:
/// - TODO
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum PlaneType {
	Predator = 1,
	Goliath = 2,
	Mohawk = 3,
	Tornado = 4,
	Prowler = 5,
}
