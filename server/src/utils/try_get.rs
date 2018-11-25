macro_rules! log_none {
	($ent:expr, $storage:expr) => {
		match $storage.get($ent) {
			Some(x) => Some(x),
			None => {
				error!(
					"Unable to fetch component from {} for {:?}",
					stringify!($storage),
					$ent
				);
				None
				}
			}
	};
	($ent:expr, mut $storage:expr) => {
		match $storage.get_mut($ent) {
			Some(x) => Some(x),
			None => {
				error!(
					"Unable to fetch component from {} for {:?}",
					stringify!($storage),
					$ent
				);
				None
				}
			}
	};
}

// Note: Only use with EventHandlers
macro_rules! try_get {
	($ent:expr, $storage:expr) => {
		match $storage.get($ent) {
			Some(x) => x,
			None => {
				error!(
					"Unable to fetch component from {} for {:?}",
					stringify!($storage),
					$ent
				);
				return;
				}
			}
	};
	($ent:expr, mut $storage:expr) => {
		match $storage.get_mut($ent) {
			Some(x) => x,
			None => {
				error!(
					"Unable to fetch component from {} for {:?}",
					stringify!($storage),
					$ent
				);
				return;
				}
			}
	};
}
