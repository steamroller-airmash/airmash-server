use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields, GenericParam};

use crate::*;

pub fn derive_event_deps(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(stream as DeriveInput);
    let crate_name =
        crate_name("airmash-server").expect("Expected airmash-server to be present in Cargo.toml");

    let typarams = input
        .generics
        .params
        .iter()
        .map(|generic| match generic {
            GenericParam::Type(ty) => {
                let ref ident = ty.ident;
                quote! { #ident, }
            }
            GenericParam::Lifetime(ltdef) => {
                let ref lt = ltdef.lifetime;
                quote! { #lt, }
            }
            GenericParam::Const(cst) => {
                let ref ident = cst.ident;
                quote! { { #ident }, }
            }
        })
        .collect::<Vec<_>>();
    let params = input.generics.params;
    let where_clause = input.generics.where_clause;
    let struct_name = input.ident;

    let field_types: Vec<_> = match input.data {
        Data::Struct(st) => match st.fields {
            Fields::Unit => Vec::new(),
            Fields::Named(fields) => fields.named.into_iter().map(|field| field.ty).collect(),
            Fields::Unnamed(fields) => fields.unnamed.into_iter().map(|field| field.ty).collect(),
        },
        Data::Enum(_) => {
            return Error::new(
                Span::call_site(),
                "Cannot automatically derive `EventDeps` for an enum",
            )
            .to_compile_error()
            .into()
        }
        Data::Union(_) => {
            return Error::new(
                Span::call_site(),
                "Cannot automatically derive `EventDeps` for a union",
            )
            .to_compile_error()
            .into()
        }
    };

    let res = quote! {
      #[automatically_derived]
      impl<#params> #crate_name::utils::EventDeps for #struct_name<#( #typarams )*>
      #where_clause
      {
        fn writes() -> Vec<::std::any::TypeId> {
          #[allow(unused_mut)]
          let mut res = vec![];
          #(
            res.append(&mut <#field_types as #crate_name::utils::EventDeps>::writes());
          )*
          res
        }
        fn reads() -> Vec<::std::any::TypeId> {
          #[allow(unused_mut)]
          let mut res = vec![];
          #(
            res.append(&mut <#field_types as #crate_name::utils::EventDeps>::reads());
          )*
          res
        }
      }
    };

    res.into()
}
