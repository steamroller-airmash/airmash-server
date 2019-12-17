macro_rules! wrapper_serde_decl {
	($type:tt) => {
		#[cfg(feature = "serde")]
		impl ::serde::Serialize for $type {
			fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
			where
				S: ::serde::Serializer,
			{
				self.0.serialize(ser)
			}
		}

		#[cfg(feature = "serde")]
		impl<'de> ::serde::Deserialize<'de> for $type {
			fn deserialize<D>(de: D) -> Result<Self, D::Error>
			where
				D: ::serde::Deserializer<'de>,
			{
				Ok($type(::serde::Deserialize::deserialize(de)?))
			}
		}
	};
}

macro_rules! impl_from_newtype_inner {
	($enum:tt, $type:tt) => {
		impl From<$type> for $enum {
			fn from(v: $type) -> Self {
				$enum::$type(v)
			}
		}
	};
}

macro_rules! impl_from_empty_inner {
	($enum:tt, $type:tt) => {
		impl From<$type> for $enum {
			fn from(_: $type) -> Self {
				$enum::$type
			}
		}
	};
}

macro_rules! impl_try_from2_inner {
	($enum:ty, $val:ident, $from:ident, $to:ident) => {
		impl std::convert::TryFrom<$val> for $enum {
			type Error = crate::error::EnumValueOutOfRangeError<$val>;

			fn try_from(v: $val) -> Result<Self, Self::Error> {
				use num_traits::FromPrimitive;

				match Self::$from(v) {
					Some(x) => Ok(x),
					None => Err(crate::error::EnumValueOutOfRangeError(v)),
				}
			}
		}

		impl From<$enum> for $val {
			fn from(v: $enum) -> Self {
				use num_traits::ToPrimitive;

				v.$to().expect("Failed to convert enum to value")
			}
		}
	};
}

macro_rules! impl_try_from2 {
	($enum:ty) => {
		impl_try_from2_inner!($enum, u8, from_u8, to_u8);
		impl_try_from2_inner!($enum, u16, from_u16, to_u16);
		impl_try_from2_inner!($enum, u32, from_u32, to_u32);
		impl_try_from2_inner!($enum, u64, from_u64, to_u64);
		impl_try_from2_inner!($enum, i8, from_i8, to_i8);
		impl_try_from2_inner!($enum, i16, from_i16, to_i16);
		impl_try_from2_inner!($enum, i32, from_i32, to_i32);
		impl_try_from2_inner!($enum, i64, from_i64, to_i64);
	};
}
