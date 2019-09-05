extern crate proc_macro;

mod event_deps;
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
