use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{token::Comma, Data, DeriveInput, Error, Fields, Type};

pub fn derive_conversions(input: TokenStream) -> TokenStream {
    match derive_impl(input) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
}

fn derive_impl(input: TokenStream) -> Result<TokenStream, Error> {
    let input: DeriveInput = syn::parse2(input)?;
    let krate = crate::crate_name("airmash-protocol");

    let data = match input.data {
        Data::Enum(data) => data,
        Data::Struct(_) | Data::Union(_) => {
            return Err(Error::new(
                input.ident.span(),
                "Conversions can only be derived for enums",
            ))
        }
    };

    let types = vec![
        quote! { u8 },
        quote! { u16 },
        quote! { u32 },
        quote! { u64 },
        quote! { u128 },
        quote! { i8 },
        quote! { i16 },
        quote! { i32 },
        quote! { i64 },
        quote! { i128 },
        quote! { usize },
        quote! { isize },
    ];

    let variants: Result<Vec<_>, _> = data
        .variants
        .iter()
        .map(|variant| -> Result<_, Error> {
            match &variant.fields {
                Fields::Unit => (),
                Fields::Named(_) | Fields::Unnamed(_) => {
                    return Err(Error::new(
                        variant.span(),
                        "Cannot derive conversions when there are variants with fields",
                    ));
                }
            }

            Ok(variant.ident.clone())
        })
        .collect();
    let variants = variants?;
    let (impl_generics, ty_generics, where_generics) = input.generics.split_for_impl();
    let name = &input.ident;
    let variants2 = &variants;

    let mut impls = Vec::new();

    for ty in &types {
        let tokens = quote! {
            #[allow(non_upper_case_globals)]
            impl #impl_generics ::core::convert::TryFrom<#ty> for #name #ty_generics
            #where_generics
            {
                type Error = #krate::error::EnumValueOutOfRangeError<#ty>;

                fn try_from(v: #ty) -> Result<Self, Self::Error> {
                    use ::core::convert::TryFrom;


                    #(
                        const #variants: #ty = #name::#variants2 as #ty;
                    )*

                    Ok(match v {
                        #(
                            #variants => Self::#variants2,
                        )*
                        _ => {
                            return Err(#krate::error::EnumValueOutOfRangeError(v))
                        }
                    })
                }
            }

            impl #impl_generics ::core::convert::From<#name #ty_generics> for #ty
            #where_generics
            {
                fn from(v: #name #ty_generics) -> #ty {
                    v as Self
                }
            }
        };

        impls.push(tokens);
    }

    Ok(quote! {
        #( #impls )*
    })
}

#[allow(dead_code)]
struct Attr {
    types: Punctuated<Type, Comma>,
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(Self {
            types: Punctuated::parse_separated_nonempty(input)?,
        })
    }
}
