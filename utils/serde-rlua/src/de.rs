use super::Error;
use rlua::{FromLua, TablePairs, TableSequence, Value};
use serde::de::{self, Deserializer, IntoDeserializer as _};
use serde::forward_to_deserialize_any;

struct Empty;

impl<'lua> FromLua<'lua> for Empty {
  fn from_lua(_: Value<'lua>, _: rlua::Context<'lua>) -> rlua::Result<Self> {
    Ok(Self)
  }
}

pub struct LuaDeserializer<'lua> {
  value: Value<'lua>,
}

impl<'lua> LuaDeserializer<'lua> {
  pub fn new(value: Value<'lua>) -> Self {
    Self { value }
  }

  fn invalid_type<E, U>(value: Value<'lua>, expected: E) -> Result<U, Error>
  where
    E: de::Expected,
  {
    use serde::de::{Error, Unexpected};

    #[rustfmt::skip]
    let unexpected: Unexpected = match value {
      Value::Boolean(v)       => Unexpected::Bool(v),
      Value::Number(v)         => Unexpected::Float(v),
      Value::Integer(v)        => Unexpected::Signed(v),
      Value::String(ref v) => Unexpected::Str(v.to_str()?),
      Value::Nil                    => Unexpected::Unit,
      Value::Table(_)               => Unexpected::Map,
      Value::Thread(_)              => Unexpected::Other("thread"),
      Value::LightUserData(_)       => Unexpected::Other("userdata"),
      Value::UserData(_)            => Unexpected::Other("userdata"),
      Value::Function(_)            => Unexpected::Other("function"),
      Value::Error(_)               => Unexpected::Other("error"),
    };

    Err(Error::invalid_type(unexpected, &expected))
  }
}

impl<'lua, 'de> Deserializer<'de> for LuaDeserializer<'lua> {
  type Error = Error;

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: de::Visitor<'de>,
  {
    match self.value {
      Value::Nil => visitor.visit_unit(),
      Value::Boolean(v) => visitor.visit_bool(v),
      Value::Integer(v) => visitor.visit_i64(v),
      Value::Number(v) => visitor.visit_f64(v),
      Value::String(v) => visitor.visit_str(v.to_str()?),
      Value::Table(table) => {
        let len = table.raw_len() as usize;

        if len == 0 {
          return visitor.visit_map(MapDeserializer::new(table.pairs()));
        }

        if table
          .clone()
          .pairs::<Empty, Empty>()
          .skip(len)
          .next()
          .is_some()
        {
          visitor.visit_map(MapDeserializer::new(table.pairs()))
        } else {
          visitor.visit_seq(SeqDeserializer(table.sequence_values()))
        }
      }
      value => Self::invalid_type(value, "value type"),
    }
  }

  fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: de::Visitor<'de>,
  {
    match self.value {
      Value::Nil => visitor.visit_none(),
      _ => visitor.visit_unit(),
    }
  }

  fn deserialize_enum<V>(
    self,
    _name: &'static str,
    _variants: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: de::Visitor<'de>,
  {
    let (variant, value) = match self.value {
      Value::Table(value) => {
        let mut iter = value.pairs::<String, Value>();
        let (variant, value) = match iter.next() {
          Some(v) => v?,
          None => {
            return Err(de::Error::invalid_value(
              de::Unexpected::Map,
              &"map with a single key",
            ))
          }
        };

        if iter.next().is_some() {
          return Err(de::Error::invalid_value(
            de::Unexpected::Map,
            &"map with a single key",
          ));
        }

        (variant, Some(value))
      }
      Value::String(variant) => (variant.to_str()?.to_owned(), None),
      value => return Self::invalid_type(value, visitor),
    };

    visitor.visit_enum(EnumDeserializer { variant, value })
  }

  fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: de::Visitor<'de>,
  {
    match self.value {
      Value::Table(v) => {
        let len = v.len()? as usize;
        let mut de = SeqDeserializer(v.sequence_values());
        let seq = visitor.visit_seq(&mut de)?;

        match de.0.count() {
          0 => return Ok(seq),
          n => Err(de::Error::custom(format_args!(
            "invalid length {}, expected sequence with {} elements",
            len,
            len - n
          ))),
        }
      }
      value => Self::invalid_type(value, "sequence"),
    }
  }

  fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: de::Visitor<'de>,
  {
    match self.value {
      Value::Table(table) => visitor.visit_map(MapDeserializer::new(table.pairs())),
      value => Self::invalid_type(value, visitor),
    }
  }

  fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: de::Visitor<'de>,
  {
    self.deserialize_seq(visitor)
  }

  fn deserialize_tuple_struct<V>(
    self,
    _name: &'static str,
    _len: usize,
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: de::Visitor<'de>,
  {
    self.deserialize_seq(visitor)
  }

  forward_to_deserialize_any! {
    bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
    byte_buf unit unit_struct newtype_struct
    struct identifier ignored_any
  }
}

struct SeqDeserializer<'lua>(TableSequence<'lua, Value<'lua>>);

impl<'lua, 'de> de::SeqAccess<'de> for SeqDeserializer<'lua> {
  type Error = Error;

  fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
  where
    T: de::DeserializeSeed<'de>,
  {
    Ok(match self.0.next() {
      Some(value) => Some(seed.deserialize(LuaDeserializer::new(value?))?),
      None => None,
    })
  }

  fn size_hint(&self) -> Option<usize> {
    match self.0.size_hint() {
      (lower, Some(upper)) if lower == upper => Some(upper),
      _ => None,
    }
  }
}

struct MapDeserializer<'lua> {
  iter: TablePairs<'lua, Value<'lua>, Value<'lua>>,
  value: Option<Value<'lua>>,
}

impl<'lua> MapDeserializer<'lua> {
  pub fn new(iter: TablePairs<'lua, Value<'lua>, Value<'lua>>) -> Self {
    Self { iter, value: None }
  }
}

impl<'lua, 'de> de::MapAccess<'de> for MapDeserializer<'lua> {
  type Error = Error;

  fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
  where
    K: de::DeserializeSeed<'de>,
  {
    Ok(match self.iter.next() {
      Some(item) => {
        let (key, value) = item?;
        self.value = Some(value);

        Some(seed.deserialize(LuaDeserializer::new(key))?)
      }
      None => None,
    })
  }

  fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
  where
    V: de::DeserializeSeed<'de>,
  {
    let value = self
      .value
      .take()
      .expect("next_value_seed called before next_key_seed");

    seed.deserialize(LuaDeserializer::new(value))
  }

  fn next_entry_seed<K, V>(
    &mut self,
    kseed: K,
    vseed: V,
  ) -> Result<Option<(K::Value, V::Value)>, Self::Error>
  where
    K: de::DeserializeSeed<'de>,
    V: de::DeserializeSeed<'de>,
  {
    Ok(match self.iter.next() {
      Some(item) => {
        let (key, value) = item?;

        let key = kseed.deserialize(LuaDeserializer::new(key))?;
        let val = vseed.deserialize(LuaDeserializer::new(value))?;

        Some((key, val))
      }
      None => None,
    })
  }

  fn size_hint(&self) -> Option<usize> {
    match self.iter.size_hint() {
      (lower, Some(upper)) if lower == upper => Some(upper),
      _ => None,
    }
  }
}

struct EnumDeserializer<'lua> {
  variant: String,
  value: Option<Value<'lua>>,
}

impl<'lua, 'de> de::EnumAccess<'de> for EnumDeserializer<'lua> {
  type Error = Error;
  type Variant = VariantDeserializer<'lua>;

  fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
  where
    V: de::DeserializeSeed<'de>,
  {
    let variant = self.variant.into_deserializer();
    let access = VariantDeserializer(self.value);

    seed.deserialize(variant).map(move |v| (v, access))
  }
}

struct VariantDeserializer<'lua>(Option<Value<'lua>>);

impl<'lua, 'de> de::VariantAccess<'de> for VariantDeserializer<'lua> {
  type Error = Error;

  fn unit_variant(self) -> Result<(), Self::Error> {
    match self.0 {
      Some(_) => Err(de::Error::invalid_type(
        de::Unexpected::NewtypeVariant,
        &"unit variant",
      )),
      None => Ok(()),
    }
  }

  fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
  where
    T: de::DeserializeSeed<'de>,
  {
    match self.0 {
      Some(value) => seed.deserialize(LuaDeserializer::new(value)),
      None => Err(de::Error::invalid_type(
        de::Unexpected::UnitVariant,
        &"newtype variant",
      )),
    }
  }

  fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: de::Visitor<'de>,
  {
    match self.0 {
      Some(value) => LuaDeserializer::new(value).deserialize_tuple(len, visitor),
      None => Err(de::Error::invalid_type(
        de::Unexpected::UnitVariant,
        &"tuple variant",
      )),
    }
  }

  fn struct_variant<V>(
    self,
    _fields: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: de::Visitor<'de>,
  {
    match self.0 {
      Some(value) => LuaDeserializer::new(value).deserialize_map(visitor),
      None => Err(de::Error::invalid_type(
        de::Unexpected::UnitVariant,
        &"struct variant",
      )),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::super::from_value;
  use rlua::Lua;

  #[test]
  fn test_struct() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Test {
      int: u32,
      seq: Vec<String>,
      map: std::collections::HashMap<i32, i32>,
      empty: Vec<()>,
    }

    let expected = Test {
      int: 1,
      seq: vec!["a".to_owned(), "b".to_owned()],
      map: vec![(1, 2), (4, 1)].into_iter().collect(),
      empty: vec![],
    };

    println!("{:?}", expected);
    let lua = Lua::new();
    lua.context(|lua| {
      let value = lua
        .load(
          r#"
                a = {}
                a.int = 1
                a.seq = {"a", "b"}
                a.map = {2, [4]=1}
                a.empty = {}
                return a
            "#,
        )
        .eval()
        .unwrap();
      let got = from_value(value).unwrap();
      assert_eq!(expected, got);
    });
  }

  #[test]
  fn test_tuple() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct Rgb(u8, u8, u8);

    let lua = Lua::new();
    lua.context(|lua| {
      let expected = Rgb(1, 2, 3);
      let value = lua
        .load(
          r#"
                a = {1, 2, 3}
                return a
            "#,
        )
        .eval()
        .unwrap();
      let got = from_value(value).unwrap();
      assert_eq!(expected, got);

      let expected = (1, 2, 3);
      let value = lua
        .load(
          r#"
                a = {1, 2, 3}
                return a
            "#,
        )
        .eval()
        .unwrap();
      let got = from_value(value).unwrap();
      assert_eq!(expected, got);
    });
  }

  #[test]
  fn test_enum() {
    #[derive(Deserialize, PartialEq, Debug)]
    enum E {
      Unit,
      Newtype(u32),
      Tuple(u32, u32),
      Struct { a: u32 },
    }

    let lua = Lua::new();
    lua.context(|lua| {
      let expected = E::Unit;
      let value = lua
        .load(
          r#"
                return "Unit"
            "#,
        )
        .eval()
        .unwrap();
      let got = from_value(value).unwrap();
      assert_eq!(expected, got);

      let expected = E::Newtype(1);
      let value = lua
        .load(
          r#"
                a = {}
                a["Newtype"] = 1
                return a
            "#,
        )
        .eval()
        .unwrap();
      let got = from_value(value).unwrap();
      assert_eq!(expected, got);

      let expected = E::Tuple(1, 2);
      let value = lua
        .load(
          r#"
                a = {}
                a["Tuple"] = {1, 2}
                return a
            "#,
        )
        .eval()
        .unwrap();
      let got = from_value(value).unwrap();
      assert_eq!(expected, got);

      let expected = E::Struct { a: 1 };
      let value = lua
        .load(
          r#"
                a = {}
                a["Struct"] = {}
                a["Struct"]["a"] = 1
                return a
            "#,
        )
        .eval()
        .unwrap();
      let got = from_value(value).unwrap();
      assert_eq!(expected, got);
    });
  }
}
