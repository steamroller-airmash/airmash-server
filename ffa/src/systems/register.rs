use airmash_server::Builder;

use super::*;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
    builder
        .with::<AddDamage>()
        .with::<TrackDamage>()
        .with::<SendScoreDetailed>()
}
