
use specs::Entity;
use special_map::BidirRemovableMap;

use fnv::FnvBuildHasher;

#[derive(Debug, Default)]
pub struct PlayerNames(pub BidirRemovableMap<String, Entity, FnvBuildHasher>);
