use super::*;
use crate::dispatch::Builder;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
  builder
    .with::<CheckExpired>()
    .with::<SpawnRandomPowerup>()
    .with::<SpawnFixedPowerup>()
    .with_handler::<Pickup>()
    .with_handler::<SendDespawn>()
}
