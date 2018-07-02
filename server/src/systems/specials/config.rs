
use types::*;

lazy_static! {
	/// The pred special causes negative energy regen
	pub static ref PREDATOR_SPECIAL_REGEN: EnergyRegen = EnergyRegen::new(-0.01);

	pub static ref GOLIATH_SPECIAL_ENERGY: Energy = unimplemented!();
	pub static ref TORNADO_SPECIAL_ENERGY: Energy = unimplemented!();
	pub static ref PROWLER_SPECIAL_ENERGY: Energy = unimplemented!();
}
