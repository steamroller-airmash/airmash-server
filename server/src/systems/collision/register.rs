use super::*;
use crate::dispatch::Builder;

pub fn register<'a, 'b>(disp: Builder<'a, 'b>) -> Builder<'a, 'b> {
  disp
    .with::<PlaneCollisionSystem>()
    .with::<MissileTerrainCollisionSystem>()
    .with::<PlayerMissileCollisionSystem>()
    .with::<PlayerPowerupCollisionSystem>()
    .with_handler::<BounceSystem>()
    .with_handler::<MissileExplodeSystem>()
    .with::<GenPlaneGrid>()
    .with::<GenMissileGrid>()
    .with::<GenPowerupGrid>()
}
