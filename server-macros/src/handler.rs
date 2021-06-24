use proc_macro2::TokenStream;

use quote::quote;
use syn::{parse_macro_input, Ident};
use syn::{ItemFn, Result};

pub fn handler(
  _attr: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  let input = parse_macro_input!(item as ItemFn);

  match impl_handler(input) {
    Ok(tokens) => tokens.into(),
    Err(e) => e.to_compile_error().into(),
  }
}

fn impl_handler(item: ItemFn) -> Result<TokenStream> {
  let const_name = Ident::new(
    &format!("__handler_{}", item.sig.ident),
    item.sig.ident.span(),
  );
  let name = &item.sig.ident;

  Ok(quote! {
    #item

    #[allow(non_upper_case_globals)]
    #[linkme::distributed_slice(airmash_server::HANDLERS)]
    static #const_name: fn(&airmash_server::EventDispatcher) = |dispatch| {
      dispatch.register(#name);
    };
  })
}
