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

macro_rules! impl_try_from_enum_inner1 {
	{
		enum $name:ident : $base:ty {
			$(
				$case:ident,
			)*
		}
	} => {
		impl ::std::convert::TryFrom<$base> for $name {
			type Error = ::error::EnumValueOutOfRangeError<$base>;

			#[allow(non_upper_case_globals, unreachable_code)]
			fn try_from(v: $base) -> Result<Self, Self::Error> {
				$(
					const $case: $base = $name::$case as $base;
				)*

				Ok(match v {
					$(
						$case => $name::$case,
					)*
					x => return Err(::error::EnumValueOutOfRangeError(x))
				})
			}
		}

		impl From<$name> for $base {
			#[allow(unreachable_code)]
			fn from(v: $name) -> Self {
				match v {
					$(
						$name::$case => $name::$case as $base,
					)*
				}
			}
		}
	}
}

macro_rules! impl_try_from_enum {
	{
		$(
			#[$attr:meta]
		)*
		pub enum $name:ident {
			$(
				$(
					#[$caseattr:meta]
				)*
				$case:ident = $val:expr,
			)*
		}
	} => {
		$(
			#[$attr]
		)*
		pub enum $name {
			$(
				$(
					#[$caseattr]
				)*
				$case = $val,
			)*
		}

		impl_try_from_enum_inner1! {
			enum $name : u8 {
				$( $case, )*
			}
		}
		impl_try_from_enum_inner1! {
			enum $name : u16 {
				$( $case, )*
			}
		}
		impl_try_from_enum_inner1! {
			enum $name : u32 {
				$( $case, )*
			}
		}
		impl_try_from_enum_inner1! {
			enum $name : i8 {
				$( $case, )*
			}
		}
		impl_try_from_enum_inner1! {
			enum $name : i16 {
				$( $case, )*
			}
		}
		impl_try_from_enum_inner1! {
			enum $name : i32 {
				$( $case, )*
			}
		}
	}
}
