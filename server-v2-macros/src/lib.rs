extern crate proc_macro;

use proc_macro::TokenStream;

#[macro_use]
mod util;

mod conversions;
mod event_handler;
mod system;
mod systemdata;

/// Derive macro for `SystemData`
#[proc_macro_derive(SystemData)]
pub fn derive_system_data(input: TokenStream) -> TokenStream {
    self::systemdata::derive_system_data(input.into()).into()
}

/// Derive conversions for an enum with discriminants.
///
/// This is mainly meant for use in airmash-protocol.
#[proc_macro_derive(Conversions)]
pub fn derive_conversions(input: TokenStream) -> TokenStream {
    self::conversions::derive_conversions(input.into()).into()
}

#[proc_macro_attribute]
pub fn system(attr: TokenStream, input: TokenStream) -> TokenStream {
    self::system::system(attr.into(), input.into()).into()
}

#[proc_macro_attribute]
pub fn event_handler(attr: TokenStream, input: TokenStream) -> TokenStream {
    self::event_handler::event_handler(attr.into(), input.into()).into()
}

fn crate_name(krate: &str) -> proc_macro2::TokenStream {
    use proc_macro2::Span;
    use quote::quote;
    use syn::Ident;

    match proc_macro_crate::crate_name(krate) {
        Ok(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote! { ::#ident }
        }
        Err(_) => {
            let ident = Ident::new(&krate.replace("-", "_"), Span::call_site());
            quote! { ::#ident }
        }
    }
}
