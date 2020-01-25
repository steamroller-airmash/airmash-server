/// Attempt to fetch data from a storage.
///
/// If there is no such data for that entity then prints
/// an error message and returns from the current function.
///
/// Usage: `try_get!(<entity>, (mut)? <storage>)`
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
				try_get![__internal, $entity, $storage];
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

/// Similar to the standard library's `dbg!` macro, except outputs
/// at the debug log level.
#[allow(unused_macros)]
macro_rules! logdbg {
    ($expr:expr) => {{
        let value = $expr;
        debug!("[line {}] {} = {:#?}", line!(), stringify!($expr), value);
        value
    }};
}
