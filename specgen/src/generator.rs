use parser::*;
use std::io::{Error, Write};

pub struct Generator {
    prelude: Option<String>,
    attrs: Vec<String>,
    typemap: Box<Fn(&SpecFieldType) -> String>,
    ser: Box<Fn(&SpecFieldType) -> String>,
    de: Box<Fn(&SpecFieldType) -> String>,
    namemap: Box<Fn(&str) -> String>,
}

impl Generator {
    const DEFAULT_SER: &'static str = "default::serialize";
    const DEFAULT_DE: &'static str = "default::deserialize";
    pub fn new() -> Self {
        Self {
            prelude: None,
            attrs: vec![],
            typemap: Box::new(|x| x.to_str()),
            namemap: Box::new(|x| x.to_owned()),
            ser: Box::new(|_| Self::DEFAULT_SER.to_string()),
            de: Box::new(|_| Self::DEFAULT_DE.to_string()),
        }
    }

    pub fn map_type<F>(mut self, f: F) -> Self
    where
        F: 'static + Fn(&SpecFieldType) -> String,
    {
        self.typemap = Box::new(f);
        self
    }
    pub fn map_name<F>(mut self, f: F) -> Self
    where
        F: 'static + Fn(&str) -> String,
    {
        self.namemap = Box::new(f);
        self
    }
    pub fn type_ser<F>(mut self, f: F) -> Self
    where
        F: 'static + Fn(&SpecFieldType) -> String,
    {
        self.ser = Box::new(f);
        self
    }
    pub fn type_de<F>(mut self, f: F) -> Self
    where
        F: 'static + Fn(&SpecFieldType) -> String,
    {
        self.de = Box::new(f);
        self
    }
    pub fn def_attr(mut self, attr: &str) -> Self {
        self.attrs.push(attr.to_owned());
        self
    }
    pub fn prelude(mut self, prelude: &str) -> Self {
        self.prelude = Some(prelude.to_owned());
        self
    }

    fn print_docs<'a, W>(writer: &mut W, docs: &[&'a str]) -> Result<(), Error>
    where
        W: Write,
    {
        for doc in docs {
            writeln!(writer, "///{}", doc)?;
        }

        Ok(())
    }
    fn print_attrs<W>(&self, writer: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        for attr in self.attrs.iter() {
            writeln!(writer, "#[{}]", attr)?;
        }

        Ok(())
    }

    pub fn build<W>(self, specdata: &[u8], writer: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        let Protocol { specs, types } = parse(specdata).unwrap();

        writeln!(writer, "use std::convert::From;")?;

        for ty in types {
            match ty.class {
                TypeClass::Enum(branches, base_ty) => {
                    self.print_attrs(writer)?;

                    writeln!(writer, "#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]")?;
                    writeln!(writer, "pub enum {name} {{", name = ty.name)?;

                    for branch in branches.iter() {
                        for doc in branch.docs.iter() {
                            writeln!(writer, "///{}", doc)?;
                        }

                        writeln!(writer, "{},", branch.name)?;
                    }

                    writeln!(writer, "}}")?;

                    writeln!(
                        writer,
                        "impl From<{name}> for {base} {{",
                        name = ty.name,
                        base = base_ty.to_str()
                    )?;

                    writeln!(
                        writer,
                        "fn from(_v: {}) -> {} {{",
                        ty.name,
                        base_ty.to_str()
                    )?;
                    if branches.len() == 0 {
                        writeln!(writer, "unimplemented!();")?;
                    } else {
                        writeln!(writer, "match _v {{")?;

                        for branch in branches.iter() {
                            writeln!(
                                writer,
                                "{name}::{branch} => {num},",
                                name = ty.name,
                                branch = branch.name,
                                num = branch.value
                            )?;
                        }

                        writeln!(writer, "}}")?;
                    }

                    writeln!(writer, "}}\n}}")?;

                    writeln!(writer, "impl {} {{", ty.name)?;
                    writeln!(
                        writer,
                        "pub fn try_from(val: {}) -> Option<Self> {{",
                        base_ty.to_str()
                    )?;

                    writeln!(writer, "match val {{")?;
                    for branch in branches.iter() {
                        writeln!(
                            writer,
                            "{num} => Some({name}::{branch}),",
                            num = branch.value,
                            branch = branch.name,
                            name = ty.name
                        )?;
                    }
                    writeln!(writer, "_ => None")?;
                    writeln!(writer, "}}")?;
                    writeln!(writer, "}}\n}}")?;

                    writeln!(writer, "impl Serialize for {name} {{", name = ty.name)?;
                    writeln!(
                        writer,
                        "fn serialize(&self, ser: &mut Serializer) -> Result<(), SerError> {{"
                    )?;
                    writeln!(writer, "{}::from(*self).serialize(ser)", base_ty.to_str())?;
                    writeln!(writer, "}}\n}}")?;

                    writeln!(
                        writer,
                        "impl<'de> Deserialize<'de> for {name} {{",
                        name = ty.name
                    )?;
                    writeln!(
                        writer,
                        "fn deserialize(de: &mut Deserializer<'de>) -> Result<Self, DeError> {{"
                    )?;
                    writeln!(
                        writer,
                        "let val = {base}::deserialize(de)?;",
                        base = base_ty.to_str()
                    )?;

                    writeln!(
                        writer,
                        "
						match Self::try_from(val) {{
							Some(v) => Ok(v),
							None => Err(DeError::InvalidEnumValue(\"{name}\", val as u64))
						}}",
                        name = ty.name
                    )?;
                    writeln!(writer, "}}\n}}")?;
                }
            }
        }

        for spec in specs {
            Self::print_docs(writer, &spec.docs)?;
            writeln!(writer, "pub mod {} {{", spec.name)?;

            if let Some(ref prelude) = self.prelude {
                writeln!(writer, "{}", prelude)?
            }

            writeln!(
                writer,
                "
				mod default {{
					{prelude}

					pub fn serialize<T>(v: &T, ser: &mut Serializer) -> Result<(), SerError>
					where T: Serialize
					{{
						v.serialize(ser)
					}}

					pub fn deserialize<'de, T>(de: &mut Deserializer<'de>) -> Result<T, DeError>
					where T: Deserialize<'de>
					{{
						T::deserialize(de)
					}}
				}}",
                prelude = self.prelude.clone().unwrap_or("".to_string())
            )?;

            for def in spec.defs {
                Self::print_docs(writer, &def.docs)?;
                self.print_attrs(writer)?;
                writeln!(
                    writer,
                    "#[derive(Clone, Debug)] pub struct {name} {{",
                    name = def.name
                )?;

                for field in def.fields.iter() {
                    Self::print_docs(writer, &field.docs)?;
                    writeln!(writer,
						"pub {name}: {type},",
						name=(self.namemap)(&field.name),
						type=(self.typemap)(&field.ty)
					)?;
                }

                writeln!(writer, "}}")?;

                writeln!(writer, "impl Serialize for {name} {{", name = def.name)?;
                writeln!(
                    writer,
                    "fn serialize(&self, ser: &mut Serializer) -> Result<(), SerError> {{"
                )?;

                for field in def.fields.iter() {
                    writeln!(
                        writer,
                        "{ser}(&self.{name}, ser)?;",
                        name = (self.namemap)(&field.name),
                        ser = (self.ser)(&field.ty)
                    )?;
                }

                writeln!(writer, "Ok(())\n}}\n}}")?;

                writeln!(
                    writer,
                    "impl<'de> Deserialize<'de> for {name} {{",
                    name = def.name
                )?;
                writeln!(
                    writer,
                    "fn deserialize(de: &mut Deserializer<'de>) -> Result<Self, DeError> {{"
                )?;
                writeln!(writer, "Ok(Self {{")?;

                for field in def.fields.iter() {
                    writeln!(
                        writer,
                        "{name}: {de}(de)?,",
                        name = (self.namemap)(&field.name),
                        de = (self.de)(&field.ty)
                    )?;
                }

                writeln!(writer, "}})")?;
                writeln!(writer, "}}")?;
                writeln!(writer, "}}")?;
            }

            writeln!(writer, "}}")?;
        }

        Ok(())
    }
}
