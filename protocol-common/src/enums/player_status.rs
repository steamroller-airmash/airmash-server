use specs::DenseVecStorage;

impl_try_from_enum! {
	/// Flag for indicating whether a player is
	/// alive or dead.
	///
	/// This is used in the following packets:
	/// - [`Login`][0] (specifically [`LoginPlayer`][1])
	/// - [`PlayerNew`][2]
	///
	/// [0]: server/struct.login.html
	/// [1]: server/struct.loginplayer.html
	/// [2]: server/struct.playernew.html
	#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
	#[cfg_attr(feature = "specs", derive(Component))]
	#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
	pub enum PlayerStatus {
		Alive = 0,
		Dead = 1,
	}
}
