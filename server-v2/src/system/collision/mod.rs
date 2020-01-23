mod bounce;
mod grids;
mod player_terrain;

pub use bounce::player_bounce;
pub use grids::build_player_grid;
pub use player_terrain::player_terrain_collision;

pub fn register(builder: &mut crate::ecs::Builder) {
    builder
        .with::<build_player_grid>()
        .with::<player_bounce>()
        .with::<player_terrain_collision>();
}
