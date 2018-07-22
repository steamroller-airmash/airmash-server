use special_map::BidirRemovableMap;
use specs::Entity;

use fnv::FnvBuildHasher;

#[derive(Debug, Default)]
pub struct PlayerNames(pub BidirRemovableMap<String, Entity, FnvBuildHasher>);
