extern crate proc_macro;

use proc_macro::TokenStream;

mod systemdata;

#[proc_macro_derive(SystemData)]
pub fn derive_system_data(input: TokenStream) -> TokenStream {
    self::systemdata::derive_system_data(input.into()).into()
}

fn crate_name() -> proc_macro2::TokenStream {
    use proc_macro2::Span;
    use quote::quote;
    use syn::Ident;

    match proc_macro_crate::crate_name("server-v2") {
        Ok(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote! { ::#ident }
        }
        Err(_) => {
            quote! { ::server_v2 }
        }
    }
}
