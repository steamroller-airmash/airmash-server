/// TODO: Reverse engineer
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Conversions)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FirewallUpdateType {
    #[doc(hidden)]
    /// Not a real value, just makes derives work
    /// remove this once the enum is reverse engineered
    _Unknown = 0,
}
