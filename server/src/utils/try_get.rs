#[doc(hidden)]
#[macro_export]
macro_rules! try_get_error {
	($ent:expr, $storage:expr) => {
		let ent = $ent;

		error!(
			"Unable to fetch component from {} for {:?} (line {})",
			stringify!($storage),
			ent,
			line!()
			);

		#[cfg(features = "sentry")]
		::airmash_server::utils::_internal_log_sentry_error(
			module_path!(),
			line!(),
			stringify!($storage),
			ent,
			);
	};
}

/// Internal sentry logging hook for use with `try_get!`
/// and `log_none!`. Don't use this yourself.
#[doc(hidden)]
#[cfg(features = "sentry")]
pub fn _internal_log_sentry_error(
	module: &'static str,
	line: u32,
	stmt: &'static str,
	ent: ::specs::Entity,
) {
	use sentry::*;

	Hub::with_active(|hub| {
		hub.capture_message(
			&*format!(
				"[{}, line {}] Unable to fetch component from {} for entity {:?}",
				module, line, stmt, ent
			),
			Level::Error,
		);
	})
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
