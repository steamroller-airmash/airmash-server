mod sysbuilder;
mod sysinfo;
mod syswrapper;

mod builder;

pub use self::builder::Builder;
pub use self::sysinfo::*;

mod inner {
	use std::cell::RefCell;
	use std::ptr;

	use crate::utils::DebugAdapter;

	thread_local! {
		#[doc(hidden)]
		pub static DEBUG_ADAPTER: RefCell<*mut DebugAdapter<'static>> = RefCell::new(ptr::null_mut());
	}
}

#[doc(hidden)]
pub use inner::DEBUG_ADAPTER;
