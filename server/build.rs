extern crate phf_codegen;
extern crate specgen;

mod flags {
	use phf_codegen;
	use std::env;
	use std::fs::File;
	use std::io::{BufWriter, Write};
	use std::path::Path;

	const KEY_TABLE: [[&'static str; 2]; 125] = [
		["SY", "FlagCode::SyrianArabRepublic"],
		["TH", "FlagCode::Thailand"],
		["TM", "FlagCode::Turkmenistan"],
		["TN", "FlagCode::Tunisia"],
		["TR", "FlagCode::Turkey"],
		["TT", "FlagCode::TrinidadandTobago"],
		["TW", "FlagCode::Taiwan"],
		["TZ", "FlagCode::Tanzania"],
		["UA", "FlagCode::Ukraine"],
		["UN", "FlagCode::UnitedNations"],
		["US", "FlagCode::UnitedStates"],
		["UY", "FlagCode::Uruguay"],
		["UZ", "FlagCode::Uzbekistan"],
		["VE", "FlagCode::Venezuela"],
		["VN", "FlagCode::VietNam"],
		["PR", "FlagCode::PuertoRico"],
		["PT", "FlagCode::Portugal"],
		["PY", "FlagCode::Paraguay"],
		["QA", "FlagCode::Qatar"],
		["RAINBOW", "FlagCode::Rainbow"],
		["RO", "FlagCode::Romania"],
		["RS", "FlagCode::Serbia"],
		["RU", "FlagCode::RussianFederation"],
		["SA", "FlagCode::SaudiArabia"],
		["SE", "FlagCode::Sweden"],
		["SG", "FlagCode::Singapore"],
		["SI", "FlagCode::Slovenia"],
		["SK", "FlagCode::Slovakia"],
		["SM", "FlagCode::SanMarino"],
		["MK", "FlagCode::Macedonia"],
		["MO", "FlagCode::Macao"],
		["MT", "FlagCode::Malta"],
		["MX", "FlagCode::Mexico"],
		["MY", "FlagCode::Malaysia"],
		["NG", "FlagCode::Nigeria"],
		["NL", "FlagCode::Netherlands"],
		["NO", "FlagCode::Norway"],
		["NP", "FlagCode::Nepal"],
		["NZ", "FlagCode::NewZealand"],
		["OM", "FlagCode::Oman"],
		["PA", "FlagCode::Panama"],
		["PE", "FlagCode::Peru"],
		["JP", "FlagCode::Japan"],
		["KP", "FlagCode::DPRK"],
		["KR", "FlagCode::SouthKorea"],
		["KW", "FlagCode::Kuwait"],
		["KZ", "FlagCode::Kazakhstan"],
		["LB", "FlagCode::Lebanon"],
		["LI", "FlagCode::Liechtenstein"],
		["LK", "FlagCode::SriLanka"],
		["LT", "FlagCode::Lithuania"],
		["LU", "FlagCode::Luxembourg"],
		["LV", "FlagCode::Latvia"],
		["HN", "FlagCode::Honduras"],
		["HR", "FlagCode::Croatia"],
		["HU", "FlagCode::Hungary"],
		["ID", "FlagCode::Indonesia"],
		["IE", "FlagCode::Ireland"],
		["IL", "FlagCode::Israel"],
		["IM", "FlagCode::IsleofMan"],
		["IMPERIAL", "FlagCode::ImperialJapan"],
		["IN", "FlagCode::India"],
		["IQ", "FlagCode::Iraq"],
		["DE", "FlagCode::Germany"],
		["DK", "FlagCode::Denmark"],
		["DO", "FlagCode::DominicanRepublic"],
		["DZ", "FlagCode::Algeria"],
		["EC", "FlagCode::Ecuador"],
		["EE", "FlagCode::Estonia"],
		["EG", "FlagCode::Egypt"],
		["ES", "FlagCode::Spain"],
		["EU", "FlagCode::EuropeanUnion"],
		["BH", "FlagCode::Bahrain"],
		["BO", "FlagCode::Bolivia"],
		["BR", "FlagCode::Brazil"],
		["BT", "FlagCode::Bhutan"],
		["BY", "FlagCode::Belarus"],
		["CA", "FlagCode::Canada"],
		["CH", "FlagCode::Switzerland"],
		["AD", "FlagCode::Andorra"],
		["AE", "FlagCode::UnitedArabEmirates"],
		["AL", "FlagCode::Albania"],
		["AM", "FlagCode::Armenia"],
		["CL", "FlagCode::Chile"],
		["AQ", "FlagCode::Antarctica"],
		["CN", "FlagCode::China"],
		["AR", "FlagCode::Argentina"],
		["FI", "FlagCode::Finland"],
		["CO", "FlagCode::Colombia"],
		["AT", "FlagCode::Austria"],
		["IR", "FlagCode::Iran"],
		["FR", "FlagCode::France"],
		["COMMUNIST", "FlagCode::Communist"],
		["AU", "FlagCode::Australia"],
		["LY", "FlagCode::LibyanArabJamahiriya"],
		["IS", "FlagCode::Iceland"],
		["GB", "FlagCode::UnitedKingdom"],
		["CONFEDERATE", "FlagCode::Confederate"],
		["AZ", "FlagCode::Azerbaijan"],
		["MA", "FlagCode::Morocco"],
		["IT", "FlagCode::Italy"],
		["GE", "FlagCode::Georgia"],
		["CR", "FlagCode::CostaRica"],
		["BA", "FlagCode::BosniaandHerzegovina"],
		["PH", "FlagCode::Philippines"],
		["MC", "FlagCode::Monaco"],
		["JM", "FlagCode::Jamaica"],
		["GR", "FlagCode::Greece"],
		["CU", "FlagCode::Cuba"],
		["BD", "FlagCode::Bangladesh"],
		["SO", "FlagCode::Somalia"],
		["PK", "FlagCode::Pakistan"],
		["MD", "FlagCode::Moldova"],
		["JO", "FlagCode::Jordan"],
		["GT", "FlagCode::Guatemala"],
		["CY", "FlagCode::Cyprus"],
		["BE", "FlagCode::Belgium"],
		["ZA", "FlagCode::SouthAfrica"],
		["SV", "FlagCode::ElSalvador"],
		["PL", "FlagCode::Poland"],
		["ME", "FlagCode::Montenegro"],
		["JOLLY", "FlagCode::JollyRogers"],
		["HK", "FlagCode::HongKong"],
		["CZ", "FlagCode::CzechRepublic"],
		["BG", "FlagCode::Bulgaria"],
	];

	pub fn write() {
		let path = Path::new(&env::var("OUT_DIR").unwrap()).join("flags-phf.rs");
		let mut file = BufWriter::new(File::create(&path).unwrap());

		write!(&mut file, "use phf;").unwrap();
		write!(
			&mut file,
			"static FLAG_MAP: phf::Map<&'static str, FlagCode> ="
		).unwrap();
		let mut map = phf_codegen::Map::new();

		for entry in KEY_TABLE.iter() {
			map.entry(entry[0], entry[1]);
		}

		map.build(&mut file).unwrap();
		write!(&mut file, ";\n").unwrap();
	}
}

mod protocol {
	use std::env;
	use std::fs::File;
	use std::io::{BufWriter, Read};
	use std::path::Path;

	use specgen::*;

	const PRELUDE: &'static str = "
	#![allow(unused_imports)]
	use specs::*;
	use protocol::serde_am::{Serializer, Deserializer, Serialize, Deserialize};
	use protocol::error::{SerError, DeError};
	use protocol::include::*;
	use protocol::datatypes::*;
	use types::{
		Position,
		Rotation,
		Speed,
		Velocity,
		Accel,
		Team,
		Score,
		Level,
		Energy,
		EnergyRegen,
		Health,
		HealthRegen
	};
	";

	fn name_map(s: &str) -> String {
		if s == "type" {
			return "ty".to_owned();
		}
		s.to_owned()
	}

	fn serde_map(s: &FieldType) -> &'static str {
		match s {
			FieldType::Simple(name) => match name {
				&"Text" => return "::protocol::field::text",
				&"TextBig" => return "::protocol::field::textbig",
				&"Entity" => return "::protocol::field::entity",
				&"Score" => return "::protocol::field::score",
				&"Level" => return "::protocol::field::level",
				&"Team" => return "::protocol::field::team",
				&"Position" => return "::protocol::field::pos",
				&"Position24" => return "::protocol::field::pos24",
				&"Position_f32" => return "::protocol::field::pos_f32",
				&"Speed" => return "::protocol::field::speed",
				&"Velocity" => return "::protocol::field::vel_u",
				&"Accel" => return "::protocol::field::accel",
				&"LowResPos" => return "::protocol::field::lowrespos",
				&"Health" => return "::protocol::field::health",
				&"Energy" => return "::protocol::field::energy",
				&"HealthRegen" => return "::protocol::field::health_regen",
				&"EnergyRegen" => return "::protocol::field::energy_regen",
				&"Rotation" => return "::protocol::field::rotation",
				&"Flag" => return "::protocol::field::flag",
				_ => (),
			},
			FieldType::Compound(name, rest) => match name {
				&"Array" => return "::protocol::field::array",
				&"ArraySmall" => return "::protocol::field::arraysmall",
				&"Option" => {
					if rest.len() != 1 {
						panic!("Found Option type with multiple or zero arguments!");
					}

					if rest[0].to_str() == "Entity" {
						return "::protocol::field::option_entity";
					}
				}
				_ => (),
			},
		}

		"default"
	}
	fn type_map(s: &FieldType) -> String {
		match s {
			FieldType::Simple(s) => match s {
				&"Position24" => return "Position".to_owned(),
				&"Position_f32" => return "Position".to_owned(),
				&"Text" => return "String".to_owned(),
				&"TextBig" => return "String".to_owned(),
				&"LowResPos" => return "Position".to_owned(),
				&"Flag" => return "Team".to_owned(),
				_ => (),
			},
			_ => (),
		}

		s.to_str()
	}

	fn ser_map(s: &FieldType) -> String {
		serde_map(s).to_owned() + "::serialize"
	}
	fn de_map(s: &FieldType) -> String {
		serde_map(s).to_owned() + "::deserialize"
	}

	pub fn write() {
		let path = Path::new(&env::var("OUT_DIR").unwrap()).join("protocol-spec.rs");
		let mut file = BufWriter::new(File::create(&path).unwrap());

		let mut bytes = vec![];
		File::open("airmash.prtcl")
			.unwrap()
			.read_to_end(&mut bytes)
			.unwrap();

		SerdeBuilder::new()
			.map_name(name_map)
			.map_type(type_map)
			.type_ser(ser_map)
			.type_de(de_map)
			.prelude(PRELUDE)
			.def_attr("derive(Component)")
			.build(&bytes, &mut file)
			.unwrap();
	}
}

fn main() {
	flags::write();
	protocol::write();
}
