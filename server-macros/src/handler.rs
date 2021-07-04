use proc_macro2::{Span, TokenStream};

use proc_macro_crate::FoundCrate;
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
  let name = &item.sig.ident;
  let krate = match proc_macro_crate::crate_name("airmash-server").unwrap_or(FoundCrate::Itself) {
    FoundCrate::Itself => Ident::new("airmash_server", Span::call_site()),
    FoundCrate::Name(name) => Ident::new(&name, Span::call_site()),
  };

  let priority = args
    .priority
    .map(|x| x.value.clone())
    .unwrap_or_else(|| parse_quote! { 0 });

  Ok(quote! {
    #item

    const _: () = {
      const PRIORITY: i32 = #priority;

      #[allow(non_upper_case_globals)]
      #[linkme::distributed_slice(#krate::_exports::AIRMASH_EVENT_HANDLERS)]
      #[linkme(crate = #krate::_exports::linkme)]
      static __: fn(&#krate::_exports::EventDispatcher) = |dispatch| {
        dispatch.register_with_priority(PRIORITY, #name);
      };
    };
  })
}
