use proc_macro2::TokenStream;

use quote::quote;
use syn::parse::Parse;
use syn::{parse_macro_input, parse_quote, Expr, Ident};
use syn::{ItemFn, Result};

use crate::args::AttrArg;

struct MacroArgs {
  priority: Option<AttrArg<Expr>>,
}

impl Parse for MacroArgs {
  fn parse(input: syn::parse::ParseStream) -> Result<Self> {
    let mut priority = None;

    while !input.is_empty() {
      let next = match input.cursor().ident() {
        Some((ident, _)) => ident,
        None => {
          return Err(syn::Error::new(
            input.cursor().span(),
            "unexpected token in input",
          ))
        }
      };

      match &*format!("{}", next) {
        "priority" => priority = Some(input.parse()?),
        x => {
          return Err(syn::Error::new(
            next.span(),
            format!("Unknown argument `{}`", x),
          ))
        }
      }
    }

    Ok(Self { priority })
  }
}

pub fn handler(
  attr: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  let args = parse_macro_input!(attr as MacroArgs);
  let input = parse_macro_input!(item as ItemFn);

  match impl_handler(input, args) {
    Ok(tokens) => tokens.into(),
    Err(e) => e.to_compile_error().into(),
  }
}

fn impl_handler(item: ItemFn, args: MacroArgs) -> Result<TokenStream> {
  let const_name = Ident::new(
    &format!("__handler_{}", item.sig.ident),
    item.sig.ident.span(),
  );
  let name = &item.sig.ident;

  let priority = args
    .priority
    .map(|x| x.value.clone())
    .unwrap_or_else(|| parse_quote! { 0 });

  Ok(quote! {
    #item

    #[allow(non_upper_case_globals)]
    #[linkme::distributed_slice(airmash_server::HANDLERS)]
    static #const_name: fn(&airmash_server::EventDispatcher) = |dispatch| {
      dispatch.register_with_priority(#priority, #name);
    };
  })
}
