macro_rules! impl_serde_inner {
	{
		struct $src:ty {
			$(
				$field:ident : $ty:ident,
			)*
		}
	} => {
		impl Serialize for $src {
			fn serialize(&self, ser: &mut Serializer) -> Result<(), SerializeError> {
				// Ensure that all serialize/deserialize pairs are in scope
				#[allow(unused_imports)]
				use funcs::*;
				use error::{ChainError, FieldSpec, FieldName};

				$(
					$ty::serialize(&self.$field, ser)
						.map_err(|e| {
							e.chain(FieldSpec{
								field: FieldName::Name(stringify!($field)),
								ty: stringify!($src).into()
							})
						})?;
				)*

				Ok(())
			}
		}

		impl Deserialize for $src {
			fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Self, DeserializeError> {
				// Ensure that all serialize/deserialize pairs are in scope
				#[allow(unused_imports)]
				use funcs::*;
				use error::{ChainError, FieldSpec, FieldName};

				Ok(Self {
					$(
						$field: $ty::deserialize(de)
							.map_err(|e| {
								e.chain(FieldSpec {
									field: FieldName::Name(stringify!($field)),
									ty: stringify!($src).into()
								})
							})
						?,
					)*
				})
			}
		}
	}
}

macro_rules! impl_serde {
	{
		$(
			struct $src:ty {
				$(
					$field:ident : $ty:ident
				),*
				$(,)*
			}
		)*
	} => {
		$(
			impl_serde_inner!{
				struct $src {
					$(
						$field : $ty,
					)*
				}
			}
		)*
	}
}
