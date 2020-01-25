use crate::component::{flag::IsPlayer, Energy, EnergyRegen, Time, Upgrades};
use crate::ecs::*;
use crate::resource::{Config, CurrentFrame, LastFrame};

/// Regenerate the energy of all players each frame.
#[system(deps = super::missile::missile_fire)]
fn update_player_energy<'a>(
    last_frame: ReadExpect<'a, LastFrame>,
    this_frame: ReadExpect<'a, CurrentFrame>,
    config: Read<'a, Config>,

    mut energy: WriteStorage<'a, Energy>,
    energy_regen: ReadStorage<'a, EnergyRegen>,
    flag: ReadStorage<'a, IsPlayer>,
    upgrades: ReadStorage<'a, Upgrades>,
) {
    let dt = Time::new((this_frame.0 - last_frame.0).subsec_nanos() as f32 * (60.0 / 1.0e9));

    (&mut energy, &upgrades, &energy_regen, &flag)
        .join()
        .map(|(energy, upgrades, &regen, _)| {
            let mult = config.upgrades.energy.factor[upgrades.energy as usize];

            (energy, regen * mult)
        })
        .for_each(|(energy, regen)| {
            let val = *energy + regen * dt;

            *energy = Energy::new(val.inner().min(1.0).max(0.0));
        });
}
