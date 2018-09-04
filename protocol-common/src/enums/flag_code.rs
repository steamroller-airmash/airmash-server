use specs::DenseVecStorage;
use std::convert::TryFrom;
use std::str::FromStr;

impl_try_from_enum! {
	/// All player flags currently available within
	/// the game.
	#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
	#[cfg_attr(feature = "specs", derive(Component))]
	#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
	pub enum FlagCode {
		SyrianArabRepublic = 1,
		Thailand = 2,
		Turkmenistan = 3,
		Tunisia = 4,
		Turkey = 5,
		TrinidadandTobago = 6,
		Taiwan = 7,
		Tanzania = 8,
		Ukraine = 9,
		UnitedNations = 10,
		UnitedStates = 11,
		Uruguay = 12,
		Uzbekistan = 13,
		Venezuela = 14,
		VietNam = 15,
		PuertoRico = 16,
		Portugal = 17,
		Paraguay = 18,
		Qatar = 19,
		Rainbow = 20,
		Romania = 21,
		Serbia = 22,
		RussianFederation = 23,
		SaudiArabia = 24,
		Sweden = 25,
		Singapore = 26,
		Slovenia = 27,
		Slovakia = 28,
		SanMarino = 29,
		Macedonia = 30,
		Macao = 31,
		Malta = 32,
		Mexico = 33,
		Malaysia = 34,
		Nigeria = 35,
		Netherlands = 36,
		Norway = 37,
		Nepal = 38,
		NewZealand = 39,
		Oman = 40,
		Panama = 41,
		Peru = 42,
		Japan = 43,
		DPRK = 44,
		SouthKorea = 45,
		Kuwait = 46,
		Kazakhstan = 47,
		Lebanon = 48,
		Liechtenstein = 49,
		SriLanka = 50,
		Lithuania = 51,
		Luxembourg = 52,
		Latvia = 53,
		Honduras = 54,
		Croatia = 55,
		Hungary = 56,
		Indonesia = 57,
		Ireland = 58,
		Israel = 59,
		IsleofMan = 60,
		ImperialJapan = 61,
		India = 62,
		Iraq = 63,
		Germany = 64,
		Denmark = 65,
		DominicanRepublic = 66,
		Algeria = 67,
		Ecuador = 68,
		Estonia = 69,
		Egypt = 70,
		Spain = 71,
		EuropeanUnion = 72,
		Bahrain = 73,
		Bolivia = 74,
		Brazil = 75,
		Bhutan = 76,
		Belarus = 77,
		Canada = 78,
		Switzerland = 79,
		Andorra = 80,
		UnitedArabEmirates = 81,
		Albania = 82,
		Armenia = 83,
		Chile = 84,
		Antarctica = 85,
		China = 86,
		Argentina = 87,
		Finland = 88,
		Colombia = 89,
		Austria = 90,
		Iran = 91,
		France = 92,
		Communist = 93,
		Australia = 94,
		LibyanArabJamahiriya = 95,
		Iceland = 96,
		UnitedKingdom = 97,
		Confederate = 98,
		Azerbaijan = 99,
		Morocco = 100,
		Italy = 101,
		Georgia = 102,
		CostaRica = 103,
		BosniaAndHerzegovina = 104,
		Philippines = 105,
		Monaco = 106,
		Jamaica = 107,
		Greece = 108,
		Cuba = 109,
		Bangladesh = 110,
		Somalia = 111,
		Pakistan = 112,
		Moldova = 113,
		Jordan = 114,
		Guatemala = 115,
		Cyprus = 116,
		Belgium = 117,
		SouthAfrica = 118,
		ElSalvador = 119,
		Poland = 120,
		Montenegro = 121,
		JollyRogers = 122,
		HongKong = 123,
		CzechRepublic = 124,
		Bulgaria = 125,
	}
}

impl<'a> TryFrom<&'a str> for FlagCode {
	type Error = ();

	fn try_from(s: &'a str) -> Result<Self, ()> {
		Self::try_from(s.to_owned())
	}
}

impl TryFrom<String> for FlagCode {
	type Error = ();

	fn try_from(s: String) -> Result<Self, ()> {
		use consts::flags::STR_TO_FLAG;
		let ref_str: &str = &s.to_uppercase();

		match STR_TO_FLAG.get(ref_str) {
			Some(&f) => Ok(f),
			None => Err(()),
		}
	}
}

impl FromStr for FlagCode {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, ()> {
		Self::try_from(s)
	}
}
