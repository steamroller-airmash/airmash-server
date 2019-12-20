use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::punctuated::Punctuated;
use syn::*;
use quote::quote;

use std::mem;

pub(crate) struct NamedArg<V> {
    pub name: Ident,
    pub eq: Token![=],
    pub value: V,
}

impl<V: Parse> Parse for NamedArg<V> {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(Self {
            name: input.parse()?,
            eq: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl<V: ToTokens> ToTokens for NamedArg<V> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens);
        self.eq.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}

pub fn strip_lifetimes(mut generics: Generics) -> Generics {
    let params = mem::replace(&mut generics.params, Punctuated::default())
        .into_pairs()
        .filter(|pair| match pair.value() {
            GenericParam::Lifetime(_) => false,
            _ => true,
        })
        .collect();

    generics.params = params;

    if let Some(mut where_clause) = generics.where_clause.take() {
        let preds = mem::replace(&mut where_clause.predicates, Punctuated::default())
            .into_pairs()
            .filter(|pred| match pred.value() {
                WherePredicate::Lifetime(_) => false,
                _ => true,
            })
            .collect();

        where_clause.predicates = preds;
        generics.where_clause = Some(where_clause);
    }

    generics
}

pub fn as_phantomdata(generics: &Generics) -> TokenStream {
    let types = generics.type_params()
        .map(|param| &param.ident);

    quote! {
        ::core::marker::PhantomData<( #( #types ),* )>
    }
}

macro_rules! parse_once {
    ($dest:expr, $input:expr) => {{
        use syn::spanned::Spanned;
        use syn::Error;

        let prev = std::mem::replace(&mut $dest, Some($input.parse()?));
        if prev.is_some() {
            let arg = $dest.as_ref().unwrap();
            return Err(Error::new(
                arg.span(),
                &format!("`{}` macro parameter specified multiple times", arg.name),
            ));
        }
    }};
}
