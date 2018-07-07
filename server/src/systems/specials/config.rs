use types::*;

use std::time::Duration;

lazy_static! {
	/// The pred special causes negative energy regen
	/// this value is the rate at which it causes 
	/// energy to decrease.
	pub static ref PREDATOR_SPECIAL_REGEN: EnergyRegen = EnergyRegen::new(-0.01);

	pub static ref GOLIATH_SPECIAL_ENERGY: Energy = Energy::new(0.5);
	// TODO: Replace this with real value (see issue #2)
	/// The distance out to which a goliath repel has an effect
	pub static ref GOLIATH_SPECIAL_RADIUS: Distance = Distance::new(100.0);

	pub static ref GOLIATH_SPECIAL_REFLECT_SPEED: Speed = Speed::new(5.0);

	pub static ref TORNADO_SPECIAL_ENERGY: Energy = Energy::new(0.9);

	pub static ref PROWLER_SPECIAL_ENERGY: Energy = Energy::new(0.6);
	pub static ref PROWLER_SPECIAL_DELAY: Duration = Duration::from_millis(1500);
}
