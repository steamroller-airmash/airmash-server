use serde::Deserializer;

pub(crate) struct SeqFwdDeserializer<A>(pub A);

impl<'de, A> SeqFwdDeserializer<A>
where
  A: serde::de::SeqAccess<'de>
{
  #[allow(dead_code, unused_variables)]
  fn bad_type(found: &str) -> <Self as Deserializer<'de>>::Error {
    serde::de::Error::custom(format_args!(
      "expected "
    ))
  }
}

#[allow(unused_variables)]
impl<'de, A> Deserializer<'de> for SeqFwdDeserializer<A>
where
  A: serde::de::SeqAccess<'de>,
{
  type Error = A::Error;

  fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    visitor.visit_seq(self.0)
  }

  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    visitor.visit_seq(self.0)
  }
  
  fn deserialize_struct<V>(
    self,
    _: &'static str,
    _: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    visitor.visit_seq(self.0)
  }

  fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_unit_struct<V>(
    self,
    name: &'static str,
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_newtype_struct<V>(
    self,
    name: &'static str,
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_tuple_struct<V>(
    self,
    name: &'static str,
    len: usize,
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_enum<V>(
    self,
    name: &'static str,
    variants: &'static [&'static str],
    visitor: V,
  ) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }

  fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
  where
    V: serde::de::Visitor<'de>,
  {
    todo!()
  }
}
