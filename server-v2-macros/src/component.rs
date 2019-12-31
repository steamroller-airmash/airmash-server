use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::spanned::Spanned;
use syn::*;

pub fn derive_component(input: TokenStream) -> TokenStream {
    match derive_impl(input) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
}

fn derive_impl(input: TokenStream) -> Result<TokenStream> {
    let input: DeriveInput = syn::parse2(input)?;
    let krate = crate::crate_name("server-v2");

    let storage_ty = match parse_args(&input.attrs)? {
        Some(ty) => ty,
        None => parse_quote! { #krate::ecs::DenseVecStorage },
    };

    let name = input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl#impl_generics #krate::ecs::Component for #name #ty_generics
        #where_clause
        {
            type Storage = #storage_ty<Self>;
        }
    })
}

fn parse_args(attrs: &[Attribute]) -> Result<Option<Type>> {
    let mut result = None;

    for attr in attrs {
        let ident = match attr.path.get_ident() {
            Some(ident) => ident,
            None => continue,
        };

        if ident == "storage" {
            if result.is_some() {
                return Err(Error::new(
                    attr.span(),
                    "Attribute `storage` specified multiple times",
                ));
            }

            let arg: MacroArg = syn::parse2(attr.tokens.clone())?;
            result = Some(arg.storage);
        }
    }

    Ok(result)
}

#[allow(dead_code)]
struct MacroArg {
    paren: syn::token::Paren,
    storage: Type,
}

impl Parse for MacroArg {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let content;

        Ok(Self {
            paren: parenthesized!(content in input),
            storage: content.parse()?,
        })
    }
}
