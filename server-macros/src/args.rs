use quote::ToTokens;
use syn::parse::Parse;
use syn::{Ident, Token};

pub struct AttrArg<P> {
  pub ident: Ident,
  pub equals_token: Token![=],
  pub value: P,
}

impl<P: Parse> Parse for AttrArg<P> {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    Ok(Self {
      ident: input.parse()?,
      equals_token: input.parse()?,
      value: input.parse()?,
    })
  }
}

impl<P: ToTokens> ToTokens for AttrArg<P> {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    self.ident.to_tokens(tokens);
    self.equals_token.to_tokens(tokens);
    self.value.to_tokens(tokens);
  }
}
