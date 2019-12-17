#[cfg(feature = "specs")]
use specs::{Component, DenseVecStorage};
use std::convert::TryFrom;
use std::str::FromStr;

/// All player flags currently available within
/// the game.
///
/// This enum can be determined from a flag code
/// string using the [`FromStr`][0] or
/// [`TryFrom`][1] implementations. Usually the
/// server will parse invalid flag code strings
/// into the [`UnitedNations`][2] variant, but
/// this is left up to the user.
///
/// # Restricted Flags
/// In the official server the following flags are
/// restricted to players level 4 and above:
/// - [`JollyRogers`](#variant.JollyRogers)
/// - [`Communist`](#variant.Communist)
/// - [`ImperialJapan`](#variant.ImperialJapan)
/// - [`Confederate`](#variant.Confederate)
/// - [`Rainbow`](#variant.Rainbow)
///
/// Changing flags in-game are restricted to those
/// level 2 and above, although any (non-restricted)
/// flag can be chosen when logging in.
///
/// [0]: https://doc.rust-lang.org/std/str/trait.FromStr.html
/// [1]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html
/// [2]: #variant.UnitedNations
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Primitive)]
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

impl_try_from2!(FlagCode);

impl<'a> TryFrom<&'a str> for FlagCode {
	type Error = <Self as TryFrom<String>>::Error;

	fn try_from(s: &'a str) -> Result<Self, Self::Error> {
		Self::try_from(s.to_owned())
	}
}

impl TryFrom<String> for FlagCode {
	type Error = ();

	fn try_from(s: String) -> Result<Self, ()> {
		use crate::consts::flags::STR_TO_FLAG;
		let ref_str: &str = &s.to_uppercase();

		match STR_TO_FLAG.get(ref_str) {
			Some(&f) => Ok(f),
			None => Err(()),
		}
	}
}

impl FromStr for FlagCode {
	type Err = <Self as TryFrom<String>>::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Self::try_from(s)
	}
}

impl Default for FlagCode {
	fn default() -> Self {
		FlagCode::UnitedNations
	}
}

impl<'a> From<FlagCode> for &'a str {
	fn from(code: FlagCode) -> &'a str {
		use self::FlagCode::*;

		match code {
			SyrianArabRepublic => "SY",
			Thailand => "TH",
			Turkmenistan => "TM",
			Tunisia => "TN",
			Turkey => "TR",
			TrinidadandTobago => "TT",
			Taiwan => "TW",
			Tanzania => "TZ",
			Ukraine => "UA",
			UnitedNations => "UN",
			UnitedStates => "US",
			Uruguay => "UY",
			Uzbekistan => "UZ",
			Venezuela => "VE",
			VietNam => "VN",
			PuertoRico => "PR",
			Portugal => "PT",
			Paraguay => "PY",
			Qatar => "QA",
			Rainbow => "RAINBOW",
			Romania => "RO",
			Serbia => "RS",
			RussianFederation => "RU",
			SaudiArabia => "SA",
			Sweden => "SE",
			Singapore => "SG",
			Slovenia => "SI",
			Slovakia => "SK",
			SanMarino => "SM",
			Macedonia => "MK",
			Macao => "MO",
			Malta => "MT",
			Mexico => "MX",
			Malaysia => "MY",
			Nigeria => "NG",
			Netherlands => "NL",
			Norway => "NO",
			Nepal => "NP",
			NewZealand => "NZ",
			Oman => "OM",
			Panama => "PA",
			Peru => "PE",
			Japan => "JP",
			DPRK => "KP",
			SouthKorea => "KR",
			Kuwait => "KW",
			Kazakhstan => "KZ",
			Lebanon => "LB",
			Liechtenstein => "LI",
			SriLanka => "LK",
			Lithuania => "LT",
			Luxembourg => "LU",
			Latvia => "LV",
			Honduras => "HN",
			Croatia => "HR",
			Hungary => "HU",
			Indonesia => "ID",
			Ireland => "IE",
			Israel => "IL",
			IsleofMan => "IM",
			ImperialJapan => "IMPERIAL",
			India => "IN",
			Iraq => "IQ",
			Germany => "DE",
			Denmark => "DK",
			DominicanRepublic => "DO",
			Algeria => "DZ",
			Ecuador => "EC",
			Estonia => "EE",
			Egypt => "EG",
			Spain => "ES",
			EuropeanUnion => "EU",
			Bahrain => "BH",
			Bolivia => "BO",
			Brazil => "BR",
			Bhutan => "BT",
			Belarus => "BY",
			Canada => "CA",
			Switzerland => "CH",
			Andorra => "AD",
			UnitedArabEmirates => "AE",
			Albania => "AL",
			Armenia => "AM",
			Chile => "CL",
			Antarctica => "AQ",
			China => "CN",
			Argentina => "AR",
			Finland => "FI",
			Colombia => "CO",
			Austria => "AT",
			Iran => "IR",
			France => "FR",
			Communist => "COMMUNIST",
			Australia => "AU",
			LibyanArabJamahiriya => "LY",
			Iceland => "IS",
			UnitedKingdom => "GB",
			Confederate => "CONFEDERATE",
			Azerbaijan => "AZ",
			Morocco => "MA",
			Italy => "IT",
			Georgia => "GE",
			CostaRica => "CR",
			BosniaAndHerzegovina => "BA",
			Philippines => "PH",
			Monaco => "MC",
			Jamaica => "JM",
			Greece => "GR",
			Cuba => "CU",
			Bangladesh => "BD",
			Somalia => "SO",
			Pakistan => "PK",
			Moldova => "MD",
			Jordan => "JO",
			Guatemala => "GT",
			Cyprus => "CY",
			Belgium => "BE",
			SouthAfrica => "ZA",
			ElSalvador => "SV",
			Poland => "PL",
			Montenegro => "ME",
			JollyRogers => "JOLLY",
			HongKong => "HK",
			CzechRepublic => "CZ",
			Bulgaria => "BG",
		}
	}
}

impl From<FlagCode> for String {
	fn from(code: FlagCode) -> String {
		let s: &'static str = code.into();
		s.to_owned()
	}
}
