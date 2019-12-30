macro_rules! try_get {
	($entity:expr, $storage:expr) => {
		match $storage.get($entity) {
			Some(x) => x,
			None => {
				try_get![__internal, $entity, $storage];
				return;
			}
		}
	};
	($entity:expr, mut $storage:expr) => {
		match $storage.get_mut($entity) {
			Some(x) => x,
			None => {
				try_get![__intenral, $entity, $storage];
				return;
			}
		}
	};

	[__internal, $entity:expr, $storage:expr] => {
		error!(
			"Unable to fetch component from {} for {:?} (line {})",
			stringify!($storage),
			$entity,
			line!()
		);
	}
}
