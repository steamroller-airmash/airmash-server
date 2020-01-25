macro_rules! impl_serde_inner {
	{
		struct $src:ident $([$lt:lifetime])?  {
			$(
				$field:ident : $ty:ident,
			)*
		}
	} => {
		impl$(<$lt>)? Serialize for $src$(<$lt>)? {
			fn serialize(&self, ser: &mut Serializer) -> Result<(), $crate::error::SerializeError> {
				// Ensure that all serialize/deserialize pairs are in scope
				#[allow(unused_imports)]
				use funcs::*;
				#[allow(unused_imports)]
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

		impl$(<$lt>)? Deserialize for $src$(<$lt>)? {
			fn deserialize<'de>(de: &mut Deserializer<'de>) -> Result<Self, $crate::error::DeserializeError> {
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
			struct $src:ident $([$lt:lifetime])? {
				$(
					$field:ident : $ty:ident
				),*
				$(,)*
			}
		)*
	} => {
		$(
			impl_serde_inner!{
				struct $src $([$lt])? {
					$(
						$field : $ty,
					)*
				}
			}
		)*
	}
}
