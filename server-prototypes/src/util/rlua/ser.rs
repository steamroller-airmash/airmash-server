use super::Error;
use rlua::{prelude::LuaString, Context, Table, Value};
use serde::ser::{self, Serialize, Serializer};

pub(crate) struct LuaSerializer<'lua> {
  lua: Context<'lua>,
}

impl<'lua> LuaSerializer<'lua> {
  pub fn new(lua: Context<'lua>) -> Self {
    Self { lua }
  }
}

impl<'lua> Serializer for LuaSerializer<'lua> {
  type Ok = Value<'lua>;
  type Error = Error;

  type SerializeSeq = SerializeVec<'lua>;
  type SerializeTuple = SerializeVec<'lua>;
  type SerializeTupleStruct = SerializeVec<'lua>;
  type SerializeTupleVariant = SerializeTupleVariant<'lua>;
  type SerializeMap = SerializeMap<'lua>;
  type SerializeStruct = SerializeMap<'lua>;
  type SerializeStructVariant = SerializeStructVariant<'lua>;

  #[inline]
  fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
    Ok(Value::Boolean(value))
  }

  #[inline]
  fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
    self.serialize_i64(i64::from(value))
  }

  #[inline]
  fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
    self.serialize_i64(i64::from(value))
  }

  #[inline]
  fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
    self.serialize_i64(i64::from(value))
  }

  #[inline]
  fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
    Ok(Value::Integer(value))
  }

  #[inline]
  fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
    self.serialize_i64(i64::from(value))
  }

  #[inline]
  fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
    self.serialize_i64(i64::from(value))
  }

  #[inline]
  fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
    self.serialize_i64(i64::from(value))
  }

  #[inline]
  fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
    self.serialize_i64(value as i64)
  }

  #[inline]
  fn serialize_f32(self, value: f32) -> Result<Self::Ok, Self::Error> {
    self.serialize_f64(f64::from(value))
  }

  #[inline]
  fn serialize_f64(self, value: f64) -> Result<Self::Ok, Self::Error> {
    Ok(Value::Number(value))
  }

  #[inline]
  fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
    let mut s = String::new();
    s.push(value);
    self.serialize_str(&s)
  }

  #[inline]
  fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
    Ok(Value::String(self.lua.create_string(value)?))
  }

  #[inline]
  fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
    Ok(Value::Table(
      self.lua.create_sequence_from(value.iter().cloned())?,
    ))
  }

  #[inline]
  fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
    Ok(Value::Nil)
  }

  #[inline]
  fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
    self.serialize_unit()
  }

  #[inline]
  fn serialize_unit_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
  ) -> Result<Self::Ok, Self::Error> {
    self.serialize_str(variant)
  }

  #[inline]
  fn serialize_newtype_struct<T>(
    self,
    _name: &'static str,
    value: &T,
  ) -> Result<Self::Ok, Self::Error>
  where
    T: ?Sized + serde::Serialize,
  {
    value.serialize(self)
  }

  fn serialize_newtype_variant<T>(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    value: &T,
  ) -> Result<Self::Ok, Self::Error>
  where
    T: ?Sized + serde::Serialize,
  {
    let table = self.lua.create_table()?;
    let variant = self.lua.create_string(variant)?;
    let value = value.serialize(self)?;
    table.set(variant, value)?;
    Ok(Value::Table(table))
  }

  #[inline]
  fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
    self.serialize_unit()
  }

  #[inline]
  fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
  where
    T: ?Sized + serde::Serialize,
  {
    value.serialize(self)
  }

  fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
    SerializeVec::new(self.lua)
  }

  fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
    self.serialize_seq(Some(len))
  }

  fn serialize_tuple_struct(
    self,
    _name: &'static str,
    len: usize,
  ) -> Result<Self::SerializeTupleStruct, Self::Error> {
    self.serialize_seq(Some(len))
  }

  fn serialize_tuple_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeTupleVariant, Self::Error> {
    SerializeTupleVariant::new(self.lua, variant)
  }

  fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
    SerializeMap::new(self.lua)
  }

  fn serialize_struct(
    self,
    _name: &'static str,
    len: usize,
  ) -> Result<Self::SerializeStruct, Self::Error> {
    self.serialize_map(Some(len))
  }

  fn serialize_struct_variant(
    self,
    _name: &'static str,
    _variant_index: u32,
    variant: &'static str,
    _len: usize,
  ) -> Result<Self::SerializeStructVariant, Self::Error> {
    SerializeStructVariant::new(self.lua, variant)
  }
}

pub(crate) struct SerializeVec<'lua> {
  lua: Context<'lua>,
  table: Table<'lua>,
  index: u64,
}

impl<'lua> SerializeVec<'lua> {
  fn new(lua: Context<'lua>) -> Result<Self, Error> {
    let table = lua.create_table()?;

    Ok(Self {
      lua,
      table,
      index: 1,
    })
  }
}

impl<'lua> ser::SerializeSeq for SerializeVec<'lua> {
  type Ok = Value<'lua>;
  type Error = Error;

  fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
  where
    T: Serialize,
  {
    let value = value.serialize(LuaSerializer::new(self.lua))?;
    self.table.set(self.index, value)?;
    self.index += 1;
    Ok(())
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    Ok(Value::Table(self.table))
  }
}

impl<'lua> ser::SerializeTuple for SerializeVec<'lua> {
  type Ok = Value<'lua>;
  type Error = Error;

  fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
  where
    T: Serialize,
  {
    ser::SerializeSeq::serialize_element(self, value)
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    ser::SerializeSeq::end(self)
  }
}

impl<'lua> ser::SerializeTupleStruct for SerializeVec<'lua> {
  type Ok = Value<'lua>;
  type Error = Error;

  fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
  where
    T: Serialize,
  {
    ser::SerializeSeq::serialize_element(self, value)
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    ser::SerializeSeq::end(self)
  }
}

pub(crate) struct SerializeMap<'lua> {
  lua: Context<'lua>,
  table: Table<'lua>,
  next: Option<Value<'lua>>,
}

impl<'lua> SerializeMap<'lua> {
  pub fn new(lua: Context<'lua>) -> Result<Self, Error> {
    Ok(Self {
      lua,
      table: lua.create_table()?,
      next: None,
    })
  }
}

impl<'lua> ser::SerializeMap for SerializeMap<'lua> {
  type Ok = Value<'lua>;
  type Error = Error;

  fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
  where
    T: Serialize,
  {
    self.next = Some(key.serialize(LuaSerializer::new(self.lua))?);
    Ok(())
  }

  fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
  where
    T: Serialize,
  {
    let key = self
      .next
      .take()
      .expect("serialize_value called before serialize_key");

    self
      .table
      .set(key, value.serialize(LuaSerializer::new(self.lua))?)?;
    Ok(())
  }

  fn serialize_entry<K: ?Sized, V: ?Sized>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
  where
    K: Serialize,
    V: Serialize,
  {
    let key = key.serialize(LuaSerializer::new(self.lua))?;
    let value = value.serialize(LuaSerializer::new(self.lua))?;

    self.table.set(key, value)?;
    Ok(())
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    Ok(Value::Table(self.table))
  }
}

impl<'lua> ser::SerializeStruct for SerializeMap<'lua> {
  type Ok = Value<'lua>;
  type Error = Error;

  fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
  where
    T: Serialize,
  {
    ser::SerializeMap::serialize_entry(self, key, value)
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    ser::SerializeMap::end(self)
  }
}

pub(crate) struct SerializeStructVariant<'lua> {
  lua: Context<'lua>,
  name: LuaString<'lua>,
  table: Table<'lua>,
}

impl<'lua> SerializeStructVariant<'lua> {
  pub fn new(lua: Context<'lua>, name: &str) -> Result<Self, Error> {
    let table = lua.create_table()?;
    let name = lua.create_string(name)?;

    Ok(Self { lua, table, name })
  }
}

impl<'lua> ser::SerializeStructVariant for SerializeStructVariant<'lua> {
  type Ok = Value<'lua>;
  type Error = Error;

  fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
  where
    T: Serialize,
  {
    self
      .table
      .set(key, value.serialize(LuaSerializer::new(self.lua))?)?;
    Ok(())
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    let table = self.lua.create_table()?;
    table.set(self.name, self.table)?;
    Ok(Value::Table(table))
  }
}

pub(crate) struct SerializeTupleVariant<'lua> {
  lua: Context<'lua>,
  name: LuaString<'lua>,
  table: Table<'lua>,
  index: u64,
}

impl<'lua> SerializeTupleVariant<'lua> {
  pub fn new(lua: Context<'lua>, name: &str) -> Result<Self, Error> {
    let table = lua.create_table()?;
    let name = lua.create_string(name)?;

    Ok(Self {
      lua,
      table,
      name,
      index: 1,
    })
  }
}

impl<'lua> ser::SerializeTupleVariant for SerializeTupleVariant<'lua> {
  type Ok = Value<'lua>;
  type Error = Error;

  fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
  where
    T: Serialize,
  {
    self
      .table
      .set(self.index, value.serialize(LuaSerializer::new(self.lua))?)?;
    self.index += 1;
    Ok(())
  }

  fn end(self) -> Result<Self::Ok, Self::Error> {
    let table = self.lua.create_table()?;
    table.set(self.name, self.table)?;
    Ok(Value::Table(table))
  }
}

#[cfg(test)]
mod tests {
  use super::super::to_value;
  use rlua::Lua;

  #[test]
  fn test_struct() {
    #[derive(Serialize)]
    struct Test {
      int: u32,
      seq: Vec<&'static str>,
    }

    let test = Test {
      int: 1,
      seq: vec!["a", "b"],
    };

    let lua = Lua::new();
    lua
      .context(|lua| {
        let value = to_value(lua, &test).unwrap();
        lua.globals().set("value", value).unwrap();
        lua
          .load(
            r#"
                assert(value["int"] == 1)
                assert(value["seq"][1] == "a")
                assert(value["seq"][2] == "b")
            "#,
          )
          .exec()
      })
      .unwrap()
  }

  #[test]
  fn test_num() {
    #[derive(Serialize)]
    enum E {
      Unit,
      Newtype(u32),
      Tuple(u32, u32),
      Struct { a: u32 },
    }

    let lua = Lua::new();

    lua
      .context(|lua| {
        let u = E::Unit;
        let value = to_value(lua, &u).unwrap();
        lua.globals().set("value", value).unwrap();
        lua
          .load(
            r#"
                assert(value == "Unit")
            "#,
          )
          .exec()
          .unwrap();

        let n = E::Newtype(1);
        let value = to_value(lua, &n).unwrap();
        lua.globals().set("value", value).unwrap();
        lua
          .load(
            r#"
                assert(value["Newtype"] == 1)
            "#,
          )
          .exec()
          .unwrap();

        let t = E::Tuple(1, 2);
        let value = to_value(lua, &t).unwrap();
        lua.globals().set("value", value).unwrap();
        lua
          .load(
            r#"
                assert(value["Tuple"][1] == 1)
                assert(value["Tuple"][2] == 2)
            "#,
          )
          .exec()
          .unwrap();

        let s = E::Struct { a: 1 };
        let value = to_value(lua, &s).unwrap();
        lua.globals().set("value", value).unwrap();
        lua
          .load(
            r#"
                assert(value["Struct"]["a"] == 1)
            "#,
          )
          .exec()
      })
      .unwrap();
  }
}
