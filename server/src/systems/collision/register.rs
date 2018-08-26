use super::*;
use dispatch::Builder;

pub fn register<'a, 'b>(disp: Builder<'a, 'b>) -> Builder<'a, 'b> {
	disp.with::<PlaneCollisionSystem>()
		.with::<MissileTerrainCollisionSystem>()
		.with::<PlayerMissileCollisionSystem>()
		.with::<PlayerUpgradeCollisionSystem>()
		.with::<BounceSystem>()
		.with::<MissileExplodeSystem>()
}
