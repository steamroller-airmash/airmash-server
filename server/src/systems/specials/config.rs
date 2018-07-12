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
	pub static ref GOLIATH_SPECIAL_RADIUS_MISSILE: Distance = Distance::new(225.0);
	pub static ref GOLIATH_SPECIAL_RADIUS_PLAYER: Distance = Distance::new(180.0);
	/// The speed at which players and mobs will be going when
	/// they are reflected
	pub static ref GOLIATH_SPECIAL_REFLECT_SPEED: Speed = Speed::new(5.0);
	/// Minimum time between reflects
	pub static ref GOLIATH_SPECIAL_INTERVAL: Duration = Duration::from_secs(1);

	pub static ref TORNADO_SPECIAL_ENERGY: Energy = Energy::new(0.9);
	pub static ref TORNADO_MISSILE_DETAILS: Vec<MissileFireInfo> = vec![
		MissileFireInfo {
			pos_offset: Position::new(
				Distance::new(0.0),
				Distance::new(40.1),
			),
			rot_offset: Rotation::new(0.0),
			ty: Mob::TornadoTripleMissile,
		},
		MissileFireInfo {
			pos_offset: Position::new(
				Distance::new(15.0),
				Distance::new(9.6)
			),
			rot_offset: Rotation::new(-0.05012323812),
			ty: Mob::TornadoTripleMissile,
		},
		MissileFireInfo {
			pos_offset: Position::new(
				Distance::new(-15.0),
				Distance::new(9.6),
			),
			rot_offset: Rotation::new(0.05012323812),
			ty: Mob::TornadoTripleMissile,
		}
	];

	pub static ref PROWLER_SPECIAL_ENERGY: Energy = Energy::new(0.6);
	pub static ref PROWLER_SPECIAL_DELAY: Duration = Duration::from_millis(1500);
}
