mod bounce;
mod grids;
mod missile_terrain;
mod player_terrain;

pub use bounce::player_bounce;
pub use grids::{build_missile_grid, build_player_grid};
pub use missile_terrain::missile_terrain_collision;
pub use player_terrain::player_terrain_collision;

pub fn register(builder: &mut crate::ecs::Builder) {
    builder
        .with::<build_player_grid>()
        .with::<build_missile_grid>()
        .with::<player_bounce>()
        .with::<missile_terrain_collision>()
        .with::<player_terrain_collision>();
}
