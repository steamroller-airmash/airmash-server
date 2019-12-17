extern crate proc_macro;

mod event_deps;
mod event_handler;
mod macro_args;
mod system;
mod systemdata;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};

enum CrateName {
    CurrentCrate,
    Named(String),
}

impl ToTokens for CrateName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            CrateName::CurrentCrate => tokens.extend(quote! { crate }),
            CrateName::Named(name) => {
                let name = Ident::new(&name, Span::call_site());
                tokens.extend(quote! { :: #name });
            }
        }
    }
}

fn crate_name(name: &str) -> Result<CrateName, String> {
    if std::env::var("CARGO_PKG_NAME").unwrap() == name {
        return Ok(CrateName::CurrentCrate);
    }

    proc_macro_crate::crate_name(name).map(CrateName::Named)
}

fn hash_fn(func: &syn::ItemFn) -> u64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = fxhash::FxHasher::default();
    func.hash(&mut hasher);

    hasher.finish()
}

#[proc_macro_derive(EventDeps)]
pub fn derive_event_deps(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::event_deps::derive_event_deps(stream)
}

#[proc_macro_derive(SystemDataCustom)]
pub fn derive_custom_systemdata(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut res = proc_macro::TokenStream::new();

    res.extend(crate::systemdata::system_data(stream.clone()));
    res.extend(crate::derive_event_deps(stream));

    res
}

/// Custom attribute macro that creates an event handler from
/// a function.
///
/// The first lifetime parameter to the function is assumed to
/// be the same lifetime parameter that would be used within
/// the `SystemData` declaration.
///
/// The only thing this macro requires is that the `SystemDataCustom`
/// macro is in scope.
///
/// # Parameters
/// The macro has three named parameter. Only `name` is
/// required.
/// - `name` (required): The struct name for the resulting system.
/// - `deps` (optional): Dependencies for the system. Specified
///    as they would be if they were specified for `SystemInfo`.
///    If not given then the default is to use `()`.
/// - `vis` (optional): Visibility of the generated structs.
///    By default this is `pub`.
///
/// # Example
/// ```ignore
/// #[event_handler(name=MyEventHandler)]
/// fn my_event_handler<'a>(evt: &MyEvent, entities: &Entities<'a>) {
///     // Do stuff ...
/// }
///
/// // MyEventHandler will be an EventHandler that runs my_event_handler
/// // with each recieved event.
/// ```
#[proc_macro_attribute]
pub fn event_handler(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    crate::event_handler::event_handler(attr, item)
}

/// Custom attribute macro that creates a system from
/// a function.
///
/// The first lifetime parameter to the function is assumed
/// to be the same lifetime parameter that would be used
/// within the `SystemData` declaration.
///
/// The only thing this macro requires is that the `SystemDataCustom`
/// macro is in scope.
///
/// # Parameters
/// The macro has three named parameter. Only `name` is
/// required.
/// - `name` (required): The struct name for the resulting system.
/// - `deps` (optional): Dependencies for the system. Specified
///    as they would be if they were specified for `SystemInfo`.
///    If not given then the default is to use `()`.
/// - `vis` (optional): Visibility of the generated structs.
///    By default this is `pub`.
///
/// # Example
/// ```ignore
/// # struct Foo;
/// # struct Bar;
///
/// #[system(name=MySystem, vis=pub(crate), deps=ThisOtherSystem)]
/// fn my_system<'a>(entities: Entities<'a>, foo: Read<'a, Foo>, bar: &Write<'a, Bar>) {
///     // Do stuff here ...
/// }
/// ```
#[proc_macro_attribute]
pub fn system(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    crate::system::system(attr, item)
}
