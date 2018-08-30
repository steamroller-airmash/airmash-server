/// Types of all mobs present in the game.
///
/// In AIRMASH, mobs are any non-player and non-wall
/// items that can be interacted with. This includes
/// powerups, upgrades, and all missiles.
///
/// Used by:
/// - TODO
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum MobType {
	PredatorMissile = 1,
	GoliathMissile = 2,
	MohawkMissile = 3,
	Upgrade = 4,
	TornadoSingleMissile = 5,
	TornadoTripleMissile = 6,
	ProwlerMissile = 7,
	Shield = 8,
	Inferno = 9,
}
