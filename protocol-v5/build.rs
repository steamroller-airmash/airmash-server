extern crate specgen;

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
                &"Velocity" => return "::protocol::field::velocity",
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
    protocol::write();
}
