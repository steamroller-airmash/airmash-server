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
