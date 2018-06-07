macro_rules! special_field_serialize {
    ($self:ident, $ser:expr, $name:ident : $type:ident) => {
        ::protocol::field::$type::serialize(&$self.$name, $ser)?
    };
}
macro_rules! special_field_deserialize {
    ($de:expr, $type:ident) => {
        ::protocol::field::$type::deserialize($de)?
    };
}

macro_rules! field_serialize_2 {
    ($self:ident, $ser:expr, $name:ident : text) => {
        special_field_serialize!($self, $ser, $name: text)
    };
    ($self:ident, $ser:expr, $name:ident : textbig) => {
        special_field_serialize!($self, $ser, $name: textbig)
    };
    ($self:ident, $ser:expr, $name:ident : array) => {
        special_field_serialize!($self, $ser, $name: array)
    };
    ($self:ident, $ser:expr, $name:ident : arraysmall) => {
        special_field_serialize!($self, $ser, $name: arraysmall)
    };
    ($self:ident, $ser:expr, $name:ident : rotation) => {
        special_field_serialize!($self, $ser, $name: rotation)
    };
    ($self:ident, $ser:expr, $name:ident : healthnergy) => {
        special_field_serialize!($self, $ser, $name: healthnergy)
    };
    ($self:ident, $ser:expr, $name:ident : uint24) => {
        special_field_serialize!($self, $ser, $name: uint24)
    };
    ($self:ident, $ser:expr, $name:ident : coordx) => {
        special_field_serialize!($self, $ser, $name: coordx)
    };
    ($self:ident, $ser:expr, $name:ident : coordy) => {
        special_field_serialize!($self, $ser, $name: coordy)
    };
    ($self:ident, $ser:expr, $name:ident : coord24) => {
        special_field_serialize!($self, $ser, $name: coord24)
    };
    ($self:ident, $ser:expr, $name:ident : regen) => {
        special_field_serialize!($self, $ser, $name: regen)
    };
    ($self:ident, $ser:expr, $name:ident : accel) => {
        special_field_serialize!($self, $ser, $name: accel)
    };
    ($self:ident, $ser:expr, $name:ident : speed) => {
        special_field_serialize!($self, $ser, $name: speed)
    };

    ($self:ident, $ser:expr, $name:ident : $type:ty) => {
        $self.$name.serialize($ser)?
    };
}

macro_rules! field_serialize {
    ($self:ident, $de:expr, $name:ident : $type:ident) => {
        field_serialize_2!($self, $de, $name: $type);
    };
    ($self:ident, $de:expr, $name:ident : $type:ident $a:ty) => {
        field_serialize_2!($self, $de, $name: $type);
    };
}

macro_rules! field_deserialize_2 {
    ($de:expr, $name:ident : text) => {
        special_field_deserialize!($de, text)
    };
    ($de:expr, $name:ident : textbig) => {
        special_field_deserialize!($de, textbig)
    };
    ($de:expr, $name:ident : array) => {
        special_field_deserialize!($de, array)
    };
    ($de:expr, $name:ident : arraysmall) => {
        special_field_deserialize!($de, arraysmall)
    };
    ($de:expr, $name:ident : rotation) => {
        special_field_deserialize!($de, rotation)
    };
    ($de:expr, $name:ident : healthnergy) => {
        special_field_deserialize!($de, healthnergy)
    };
    ($de:expr, $name:ident : uint24) => {
        special_field_deserialize!($de, uint24)
    };
    ($de:expr, $name:ident : coordy) => {
        special_field_deserialize!($de, coordy)
    };
    ($de:expr, $name:ident : coordx) => {
        special_field_deserialize!($de, coordx)
    };
    ($de:expr, $name:ident : coord24) => {
        special_field_deserialize!($de, coord24)
    };
    ($de:expr, $name:ident : regen) => {
        special_field_deserialize!($de, regen)
    };
    ($de:expr, $name:ident : accel) => {
        special_field_deserialize!($de, accel)
    };
    ($de:expr, $name:ident : speed) => {
        special_field_deserialize!($de, speed)
    };

    ($de:expr, $name:ident : $type:ident) => {
        $type::deserialize($de)?
    };
}

macro_rules! field_deserialize {
    ($de:expr, $name:ident : $type:ident) => {
        field_deserialize_2!($de, $name: $type);
    };
    ($de:expr, $name:ident : $type:ident + $a:ty) => {
        field_deserialize_2!($de, $name: $type);
    };
}

macro_rules! get_field_type {
    (text) => { String };
    (textbig) => { String };
    (array[$subty:ty]) => { ::std::vec::Vec<get_field_type!($subty)> };
    (arraysmall[$subty:ty]) => { ::std::vec::Vec<get_field_type!($subty)> };
    (rotation) => { f32 };
    (healthnergy) => { f32 };
    (uint24) => { u32 };
    (coordy) => { f32 };
    (coordx) => { f32 };
    (coord24) => { f32 };
    (regen) => { f32 };
    (accel) => { f32 };
    (speed) => { f32 };

    ($type:ty) => { $type };
}

macro_rules! serde_decl {
    ($(#[$attr:meta])* struct $name:ident { $($( #[$fattr:meta] )* $field:ident : $type:tt $([ $targs:ty ])*),* }) => {
        impl ::protocol::serde_am::Serialize for $name {
            fn serialize(&self, ser: &mut ::protocol::serde_am::Serializer) -> ::protocol::serde_am::Result<()> {
                #[allow(unused_imports)]
                use ::protocol::serde_am::*;

                // This is harmless, since unit 
                // serializes to nothing
                let _result = ser.serialize_unit()?;

                $(
                    let _result = field_serialize!(self, ser, $field : $type);
                )*

                Ok(_result)
            }
        }

        impl<'de> ::protocol::serde_am::Deserialize<'de> for $name {
            fn deserialize(_de: &mut ::protocol::serde_am::Deserializer<'de>) -> ::protocol::serde_am::Result<Self> {
                #[allow(unused_imports)]
                use protocol::serde_am::*;

                Ok(Self {
                    $(
                        $field: field_deserialize!(_de, $field : $type),
                    )*
                })
            }
        }

        $(#[$attr])*
        pub struct $name {
            $(
                $( #[$fattr] )*
                pub $field: get_field_type!($type $([ $targs ])*),
            )*
        }
    };
}

/// Takes a series of of structs of the form
/// ```no_test
/// #[attr]...
/// pub struct $StructName {
///     pub $field: $type
/// }
/// ```
/// where $type can be any serializable type, 
/// or one of these extra identifiers: `text`,
/// `textbig`, `array`, `arraysmall`, `coordx`,
/// `coordy`, `speed`, `accel`, `regen`, 
/// `healthnergy`, `rotation`, `coord24`, or
/// `uint24`. See the documentation within
/// [`fields.rs`](../../src/airmash_protocol/fields.rs.html)
/// for more details on how these custom types are 
/// serialized and deserialized.
macro_rules! serde_decls {
    {$($(#[$attr:meta])* pub struct $name:ident { $($( #[$fattr:meta] )* pub $field:ident : $type:tt $([ $targs:ty ])*),* })*} => {
        $(
            serde_decl!{
                $( #[$attr] )*
                struct $name {
                    $( $( #[$fattr] )* $field : $type $([ $targs ])* ),*
                }
            }
        )*
    };
}
