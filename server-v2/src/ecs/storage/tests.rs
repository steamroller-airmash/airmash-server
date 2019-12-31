macro_rules! declare_tests {
	[ $storage:ident ] => {
		#[test]
		fn insert() {
			let mut storage = $storage::default();
			storage.insert(0, 32);
			storage.insert(10, 44);

			assert_eq!(storage.get(0).copied(), Some(32));
			assert_eq!(storage.get(10).copied(), Some(44));
		}
	};
	( $( $storage:ident ),* $(,)? ) => {

		$(
			#[allow(non_snake_case)]
			mod $storage {
				use crate::ecs::*;

				declare_tests![$storage];
			}
		)*
	}
}

declare_tests!(DenseVecStorage, HashMapStorage, VecStorage);
