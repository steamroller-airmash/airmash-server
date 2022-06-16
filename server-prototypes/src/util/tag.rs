use serde::ser::{self, Impossible};
use serde::Serializer;

pub(crate) struct TagSerializer<'a, S> {
  stname: &'static str,
  tag: &'static str,
  value: &'a str,
  ser: S,
}

impl<'a, M: Serializer> TagSerializer<'a, M> {
  pub fn new(stname: &'static str, tag: &'static str, value: &'a str, ser: M) -> Self {
    Self {
      stname,
      tag,
      value,
      ser,
    }
  }

  fn bad_type<U>(ty: &str) -> Result<U, M::Error> {
    Err(ser::Error::custom(format_args!(
      "can only flatten structs (got {})",
      ty
    )))
  }
}

#[allow(unused_variables)]
impl<'a, M: Serializer> Serializer for TagSerializer<'a, M> {
  type Ok = M::Ok;
  type Error = M::Error;

  type SerializeStruct = M::SerializeStruct;

  fn serialize_struct(
    self,
    _: &'static str,
    len: usize,
  ) -> Result<Self::SerializeStruct, Self::Error> {
    use serde::ser::SerializeStruct;

    let mut ser = self.ser.serialize_struct(self.stname, len + 1)?;
    ser.serialize_field(self.tag, self.value)?;
    Ok(ser)
  }

  fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
    use serde::ser::SerializeStruct;

    let mut ser = self.ser.serialize_struct(self.stname, 1)?;
    ser.serialize_field(self.tag, self.value)?;
    ser.end()
  }

  // Error boilerplate
  type SerializeSeq = Impossible<Self::Ok, M::Error>;
  type SerializeTuple = Impossible<Self::Ok, M::Error>;
  type SerializeTupleStruct = Impossible<Self::Ok, M::Error>;
  type SerializeMap = Impossible<Self::Ok, M::Error>;
  type SerializeTupleVariant = Impossible<Self::Ok, M::Error>;
  type SerializeStructVariant = Impossible<Self::Ok, M::Error>;

  fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
    Self::bad_type("bool")
  }

  fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
    Self::bad_type("i8")
  }

  fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
    Self::bad_type("i16")
  }

  fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
    Self::bad_type("i32")
  }

  fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
    Self::bad_type("i64")
  }

  fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
    Self::bad_type("u8")
  }

  fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
    Self::bad_type("u16")
  }

  fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
    Self::bad_type("u32")
  }

  fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
    Self::bad_type("u64")
  }

  fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
    Self::bad_type("f32")
  }

  fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
    Self::bad_type("f64")
  }

  fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
    Self::bad_type("char")
  }

  fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
    Self::bad_type("str")
  }

  fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
    Self::bad_type("bytes")
  }

  fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
    Self::bad_type("none")
  }

  fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
  where
    T: serde::Serialize,
  {
    Self::bad_type("some")
  }

  fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
    Self::bad_type("unit")
  }

  fn serialize_unit_variant(
    self,
    name: &'static str,
    variant_index: u32,
    variant: &'static str,
  ) -> Result<Self::Ok, Self::Error> {
    Self::bad_type("unit enum variant")
  }

  fn serialize_newtype_struct<T: ?Sized>(
    self,
    name: &'static str,
    value: &T,
  ) -> Result<Self::Ok, Self::Error>
  where
    T: serde::Serialize,
  {
    Self::bad_type("newtype struct")
  }

  fn serialize_newtype_variant<T: ?Sized>(
    self,
    name: &'static str,
    variant_index: u32,
    variant: &'static str,
    value: &T,
  ) -> Result<Self::Ok, Self::Error>
  where
    T: serde::Serialize,
  {
    Self::bad_type("newtype variant")
  }

  fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
    Self::bad_type("seq")
  }

  fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
    Self::bad_type("tuple")
  }

  fn serialize_tuple_struct(
    self,
    name: &'static str,
    len: usize,
  ) -> Result<Self::SerializeTupleStruct, Self::Error> {
    Self::bad_type("tuple struct")
  }

  fn serialize_tuple_variant(
    self,
    name: &'static str,
    variant_index: u32,
    variant: &'static str,
    len: usize,
  ) -> Result<Self::SerializeTupleVariant, Self::Error> {
    Self::bad_type("tuple variant")
  }

  fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
    Self::bad_type("map")
  }

  fn serialize_struct_variant(
    self,
    name: &'static str,
    variant_index: u32,
    variant: &'static str,
    len: usize,
  ) -> Result<Self::SerializeStructVariant, Self::Error> {
    Self::bad_type("struct variant")
  }
}
