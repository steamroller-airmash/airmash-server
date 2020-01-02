mod grids;

pub use grids::build_player_grid;

pub fn register(builder: &mut crate::ecs::Builder) {
    builder.with::<build_player_grid>();
}
