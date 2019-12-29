use special_map::BidirRemovableMap;
use specs::Entity;

#[derive(Debug, Default)]
pub struct PlayerNames(pub BidirRemovableMap<String, Entity>);
