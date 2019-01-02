/// Shortcut macro for implementing [`SystemInfo`](crate::SystemInfo)
/// to avoid writing out some unnecessary boilerplate.
///
/// This macro requires that system implements
/// [`Default`](std::default::Default). If the system doesn't
/// implement default, then it is necessary to implement `SystemInfo`
/// yourself.
///
/// # Example
/// ```
/// #[macro_use]
/// extern crate airmash_server;
///
/// use airmash_server::systems::PositionUpdate;
///
/// #[derive(Default)]
/// pub struct MySystem;
///
/// // Implement System or EventHandler here...
///
/// system_info! {
///   impl SystemInfo for MySystem {
///     // Specify that MySystem should run after
///     // the PositionUpdate system.
///     type Dependencies = PositionUpdate;
///   }
/// }
///
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! system_info {
	{
		$(
		impl SystemInfo for $name:ty {
			type Dependencies = $deps:ty ;
		}
		)*
	} => {
		$(
		impl ::airmash_server::SystemInfo for $name {
			type Dependencies = $deps;

			fn name() -> &'static str {
				concat!(module_path!(), "::", line!())
			}

			fn new() -> Self {
				Self::default()
			}
		}
		)*
	}
}
