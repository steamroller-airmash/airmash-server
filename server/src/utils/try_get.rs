macro_rules! try_get_error {
	($ent:expr, $storage:expr) => {
		error!(
			"Unable to fetch component from {} for {:?} (line {})",
			stringify!($storage),
			$ent,
			line!()
		);
	};
}

#[macro_export]
macro_rules! log_none {
	($ent:expr, $storage:expr) => {
		match $storage.get($ent) {
			Some(x) => Some(x),
			None => {
				try_get_error!($ent, $storage);
				None
				}
			}
	};
	($ent:expr, mut $storage:expr) => {
		match $storage.get_mut($ent) {
			Some(x) => Some(x),
			None => {
				try_get_error!($ent, $storage);
				None
				}
			}
	};
}

// Note: Only use with EventHandlers
#[macro_export]
macro_rules! try_get {
	($ent:expr, $storage:expr) => {
		match $storage.get($ent) {
			Some(x) => x,
			None => {
				try_get_error!($ent, $storage);
				return;
				}
			}
	};
	($ent:expr, mut $storage:expr) => {
		match $storage.get_mut($ent) {
			Some(x) => x,
			None => {
				try_get_error!($ent, $storage);
				return;
				}
			}
	};
}
