mod args;
mod handler;

/// Automatically register a static function as an event handler.
///
/// This macro is meant to be placed on a function definition. Optionally, it
/// can also take a priority with which to register the event handler. This
/// priority can be any const expression evaluating to an `i32`. The attribute
/// macro expects that the first parameter is a reference to the type of the
/// event that the function is supposed to handle. Any struct should
/// work here as long as it meets the requirements for the `Event` trait.
///
/// # Caveats
/// Internally this macro uses the [`linkme`] crate. `linkme` has an
/// [issue](https://github.com/dtolnay/linkme/issues/31) where if a module
/// doesn't have anything else that is used by the program then it will be
/// dropped incorrectly even if it contains elements that would be part of the
/// linked slice. In practical terms, this means that modules that contain only
/// handlers registered via `#[handler]` will be incorrectly dropped until we
/// get a rust version where the compiler bug is fixed.
///
/// # Example
/// ```ignore
/// # struct MyEvent;
/// # struct AirmashGame; // We can't import airmash here
/// const MY_CUSTOM_PRIORITY: i32 = 335;
///
/// // Here we create an event handler with the default priority.
/// #[handler]
/// fn my_first_handler(event: &MyEvent, game: &mut AirmashGame) {
///   // ... do stuff here
/// }
///
/// // Create an event handler with a custom priority. Note that we can use
/// // any expression that is const-evaluatable.
/// #[handler(priority = MY_CUSTOM_PRIORITY)]
/// fn my_custom_handler(event: &MyEvent, game: &mut AirmashGame) {
///   // .. do stuff here
/// }
/// ```
///
/// [`linkme`]: https://docs.rs/linkme
#[proc_macro_attribute]
pub fn handler(
  attr: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  handler::handler(attr, item)
}
