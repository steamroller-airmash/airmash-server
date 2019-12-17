use crate::types::{Energy, EnergyRegen, Player};

/// A prowler has entered/exited stealth mode
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventStealth {
	pub id: Player,
	pub state: bool,
	pub energy: Energy,
	pub energy_regen: EnergyRegen,
}
