use crate::types::*;
use specs::prelude::*;

use crate::component::flag::IsPlayer;
use crate::component::time::{LastFrame, ThisFrame};

#[derive(Default)]
pub struct EnergyRegenSystem;

#[derive(SystemData)]
pub struct EnergyRegenSystemData<'a> {
  pub lastframe: Read<'a, LastFrame>,
  pub thisframe: Read<'a, ThisFrame>,
  pub config: Read<'a, Config>,

  pub energy: WriteStorage<'a, Energy>,
  pub energy_regen: ReadStorage<'a, EnergyRegen>,
  pub flag: ReadStorage<'a, IsPlayer>,
  pub upgrades: ReadStorage<'a, Upgrades>,
}

impl<'a> System<'a> for EnergyRegenSystem {
  type SystemData = EnergyRegenSystemData<'a>;

  fn run(&mut self, data: Self::SystemData) {
    let Self::SystemData {
      lastframe,
      thisframe,
      config,
      mut energy,
      flag,
      upgrades,
      energy_regen,
    } = data;

    let dt = Time::new((thisframe.0 - lastframe.0).subsec_nanos() as f32 * (60.0 / 1.0e9));

    (&mut energy, &flag, &upgrades, &energy_regen)
      .join()
      .map(|(energy, _, upgrades, regen)| {
        let mult = config.upgrades.energy.factor[upgrades.energy as usize];

        (energy, *regen * mult)
      })
      .for_each(|(energy, regen)| {
        let val: Energy = *energy + regen * dt;

        *energy = Energy::new(val.inner().min(1.0).max(0.0));
      });
  }
}

system_info! {
  impl SystemInfo for EnergyRegenSystem {
    type Dependencies = super::missile::MissileFireHandler;
  }
}
