use fnv::FnvHashMap;
use protocol::include::FlagCode;

const COUNTRY_CODES: [Option<&'static str>; 126] = [
	None,
	Some("SY"),
	Some("TH"),
	Some("TM"),
	Some("TN"),
	Some("TR"),
	Some("TT"),
	Some("TW"),
	Some("TZ"),
	Some("UA"),
	Some("UN"),
	Some("US"),
	Some("UY"),
	Some("UZ"),
	Some("VE"),
	Some("VN"),
	Some("PR"),
	Some("PT"),
	Some("PY"),
	Some("QA"),
	Some("rainbow"),
	Some("RO"),
	Some("RS"),
	Some("RU"),
	Some("SA"),
	Some("SE"),
	Some("SG"),
	Some("SI"),
	Some("SK"),
	Some("SM"),
	Some("MK"),
	Some("MO"),
	Some("MT"),
	Some("MX"),
	Some("MY"),
	Some("NG"),
	Some("NL"),
	Some("NO"),
	Some("NP"),
	Some("NZ"),
	Some("OM"),
	Some("PA"),
	Some("PE"),
	Some("JP"),
	Some("KP"),
	Some("KR"),
	Some("KW"),
	Some("KZ"),
	Some("LB"),
	Some("LI"),
	Some("LK"),
	Some("LT"),
	Some("LU"),
	Some("LV"),
	Some("HN"),
	Some("HR"),
	Some("HU"),
	Some("ID"),
	Some("IE"),
	Some("IL"),
	Some("IM"),
	Some("imperial"),
	Some("IN"),
	Some("IQ"),
	Some("DE"),
	Some("DK"),
	Some("DO"),
	Some("DZ"),
	Some("EC"),
	Some("EE"),
	Some("EG"),
	Some("ES"),
	Some("EU"),
	Some("BH"),
	Some("BO"),
	Some("BR"),
	Some("BT"),
	Some("BY"),
	Some("CA"),
	Some("CH"),
	Some("AD"),
	Some("AE"),
	Some("AL"),
	Some("AM"),
	Some("CL"),
	Some("AQ"),
	Some("CN"),
	Some("AR"),
	Some("FI"),
	Some("CO"),
	Some("AT"),
	Some("IR"),
	Some("FR"),
	Some("communist"),
	Some("AU"),
	Some("LY"),
	Some("IS"),
	Some("GB"),
	Some("confederate"),
	Some("AZ"),
	Some("MA"),
	Some("IT"),
	Some("GE"),
	Some("CR"),
	Some("BA"),
	Some("PH"),
	Some("MC"),
	Some("JM"),
	Some("GR"),
	Some("CU"),
	Some("BD"),
	Some("SO"),
	Some("PK"),
	Some("MD"),
	Some("JO"),
	Some("GT"),
	Some("CY"),
	Some("BE"),
	Some("ZA"),
	Some("SV"),
	Some("PL"),
	Some("ME"),
	Some("jolly"),
	Some("HK"),
	Some("CZ"),
	Some("BG"),
];

lazy_static! {
	static ref STR_TO_FLAG: FnvHashMap<&'static str, FlagCode> = {
		let mut map = FnvHashMap::default();

		map.insert("SY", FlagCode::SyrianArabRepublic);
		map.insert("TH", FlagCode::Thailand);
		map.insert("TM", FlagCode::Turkmenistan);
		map.insert("TN", FlagCode::Tunisia);
		map.insert("TR", FlagCode::Turkey);
		map.insert("TT", FlagCode::TrinidadandTobago);
		map.insert("TW", FlagCode::Taiwan);
		map.insert("TZ", FlagCode::Tanzania);
		map.insert("UA", FlagCode::Ukraine);
		map.insert("UN", FlagCode::UnitedNations);
		map.insert("US", FlagCode::UnitedStates);
		map.insert("UY", FlagCode::Uruguay);
		map.insert("UZ", FlagCode::Uzbekistan);
		map.insert("VE", FlagCode::Venezuela);
		map.insert("VN", FlagCode::VietNam);
		map.insert("PR", FlagCode::PuertoRico);
		map.insert("PT", FlagCode::Portugal);
		map.insert("PY", FlagCode::Paraguay);
		map.insert("QA", FlagCode::Qatar);
		map.insert("RAINBOW", FlagCode::Rainbow);
		map.insert("RO", FlagCode::Romania);
		map.insert("RS", FlagCode::Serbia);
		map.insert("RU", FlagCode::RussianFederation);
		map.insert("SA", FlagCode::SaudiArabia);
		map.insert("SE", FlagCode::Sweden);
		map.insert("SG", FlagCode::Singapore);
		map.insert("SI", FlagCode::Slovenia);
		map.insert("SK", FlagCode::Slovakia);
		map.insert("SM", FlagCode::SanMarino);
		map.insert("MK", FlagCode::Macedonia);
		map.insert("MO", FlagCode::Macao);
		map.insert("MT", FlagCode::Malta);
		map.insert("MX", FlagCode::Mexico);
		map.insert("MY", FlagCode::Malaysia);
		map.insert("NG", FlagCode::Nigeria);
		map.insert("NL", FlagCode::Netherlands);
		map.insert("NO", FlagCode::Norway);
		map.insert("NP", FlagCode::Nepal);
		map.insert("NZ", FlagCode::NewZealand);
		map.insert("OM", FlagCode::Oman);
		map.insert("PA", FlagCode::Panama);
		map.insert("PE", FlagCode::Peru);
		map.insert("JP", FlagCode::Japan);
		map.insert("KP", FlagCode::DPRK);
		map.insert("KR", FlagCode::SouthKorea);
		map.insert("KW", FlagCode::Kuwait);
		map.insert("KZ", FlagCode::Kazakhstan);
		map.insert("LB", FlagCode::Lebanon);
		map.insert("LI", FlagCode::Liechtenstein);
		map.insert("LK", FlagCode::SriLanka);
		map.insert("LT", FlagCode::Lithuania);
		map.insert("LU", FlagCode::Luxembourg);
		map.insert("LV", FlagCode::Latvia);
		map.insert("HN", FlagCode::Honduras);
		map.insert("HR", FlagCode::Croatia);
		map.insert("HU", FlagCode::Hungary);
		map.insert("ID", FlagCode::Indonesia);
		map.insert("IE", FlagCode::Ireland);
		map.insert("IL", FlagCode::Israel);
		map.insert("IM", FlagCode::IsleofMan);
		map.insert("IMPERIAL", FlagCode::ImperialJapan);
		map.insert("IN", FlagCode::India);
		map.insert("IQ", FlagCode::Iraq);
		map.insert("DE", FlagCode::Germany);
		map.insert("DK", FlagCode::Denmark);
		map.insert("DO", FlagCode::DominicanRepublic);
		map.insert("DZ", FlagCode::Algeria);
		map.insert("EC", FlagCode::Ecuador);
		map.insert("EE", FlagCode::Estonia);
		map.insert("EG", FlagCode::Egypt);
		map.insert("ES", FlagCode::Spain);
		map.insert("EU", FlagCode::EuropeanUnion);
		map.insert("BH", FlagCode::Bahrain);
		map.insert("BO", FlagCode::Bolivia);
		map.insert("BR", FlagCode::Brazil);
		map.insert("BT", FlagCode::Bhutan);
		map.insert("BY", FlagCode::Belarus);
		map.insert("CA", FlagCode::Canada);
		map.insert("CH", FlagCode::Switzerland);
		map.insert("AD", FlagCode::Andorra);
		map.insert("AE", FlagCode::UnitedArabEmirates);
		map.insert("AL", FlagCode::Albania);
		map.insert("AM", FlagCode::Armenia);
		map.insert("CL", FlagCode::Chile);
		map.insert("AQ", FlagCode::Antarctica);
		map.insert("CN", FlagCode::China);
		map.insert("AR", FlagCode::Argentina);
		map.insert("FI", FlagCode::Finland);
		map.insert("CO", FlagCode::Colombia);
		map.insert("AT", FlagCode::Austria);
		map.insert("IR", FlagCode::Iran);
		map.insert("FR", FlagCode::France);
		map.insert("COMMUNIST", FlagCode::Communist);
		map.insert("AU", FlagCode::Australia);
		map.insert("LY", FlagCode::LibyanArabJamahiriya);
		map.insert("IS", FlagCode::Iceland);
		map.insert("GB", FlagCode::UnitedKingdom);
		map.insert("CONFEDERATE", FlagCode::Confederate);
		map.insert("AZ", FlagCode::Azerbaijan);
		map.insert("MA", FlagCode::Morocco);
		map.insert("IT", FlagCode::Italy);
		map.insert("GE", FlagCode::Georgia);
		map.insert("CR", FlagCode::CostaRica);
		map.insert("BA", FlagCode::BosniaAndHerzegovina);
		map.insert("PH", FlagCode::Philippines);
		map.insert("MC", FlagCode::Monaco);
		map.insert("JM", FlagCode::Jamaica);
		map.insert("GR", FlagCode::Greece);
		map.insert("CU", FlagCode::Cuba);
		map.insert("BD", FlagCode::Bangladesh);
		map.insert("SO", FlagCode::Somalia);
		map.insert("PK", FlagCode::Pakistan);
		map.insert("MD", FlagCode::Moldova);
		map.insert("JO", FlagCode::Jordan);
		map.insert("GT", FlagCode::Guatemala);
		map.insert("CY", FlagCode::Cyprus);
		map.insert("BE", FlagCode::Belgium);
		map.insert("ZA", FlagCode::SouthAfrica);
		map.insert("SV", FlagCode::ElSalvador);
		map.insert("PL", FlagCode::Poland);
		map.insert("ME", FlagCode::Montenegro);
		map.insert("JOLLY", FlagCode::JollyRogers);
		map.insert("HK", FlagCode::HongKong);
		map.insert("CZ", FlagCode::CzechRepublic);
		map.insert("BG", FlagCode::Bulgaria);

		map
	};
}

impl FlagCode {
	/// Returns the numeric value that corresponds
	/// to the flag code.
	///
	/// This is the value that is actually sent
	/// in the serialized packet and is used
	/// in the client.
	///
	/// Credit to Bombita for determining the
	/// flag -> number mapping. Sources
	/// [here](https://gist.github.com/Molesmalo/5e6d51aef8558e193720c3d963aeb5b7).
	pub fn to_u16(self) -> u16 {
		u16::from(self)
	}

	/// Return the flag code that corresponds to
	/// the numeric value, or `None` if there
	/// is no corresponding flag.
	///
	/// Credit to Bombita for determining the
	/// flag -> number mapping. Sources
	/// [here](https://gist.github.com/Molesmalo/5e6d51aef8558e193720c3d963aeb5b7).
	pub fn from_u16(v: u16) -> Option<FlagCode> {
		FlagCode::try_from(v)
	}

	/// Get the ISO-2 code associated with
	/// the flag (or special codes for non-country
	/// flags)
	pub fn to_str(self) -> &'static str {
		COUNTRY_CODES[self.to_u16() as usize].unwrap()
	}

	/// Get the flag code associated with an
	/// ISO-2 country code. (or special codes
	/// for non-country flags).
	pub fn from_str(v: &str) -> Option<Self> {
		let uppercase: String = v.to_uppercase();
		let ref_val: &str = &uppercase;

		STR_TO_FLAG.get(ref_val).map(|x| *x)
	}
}

impl Default for FlagCode {
	fn default() -> Self {
		return FlagCode::UnitedNations;
	}
}