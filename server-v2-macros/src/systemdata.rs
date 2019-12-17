use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields};

use super::crate_name;

pub fn derive_system_data(input: TokenStream) -> TokenStream {
    match derive_impl(input) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
}

fn derive_impl(input: TokenStream) -> Result<TokenStream, Error> {
    let input: DeriveInput = syn::parse2(input)?;

    let krate = crate_name();
    let lifetime = match input.generics.lifetimes().next() {
        Some(lt) => lt,
        None => {
            return Err(Error::new_spanned(
                input.ident,
                "Must have at least 1 lifetime to derive SystemData",
            ))
        }
    };

    let bounds: Vec<_> = input
        .generics
        .type_params()
        .map(|param| {
            quote! {
                #param: #krate::ecs::SystemData<#lifetime>,
            }
        })
        .collect();
    let name = input.ident;
    let mut generics = input.generics.clone();
    generics.make_where_clause();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let data = match input.data {
        Data::Struct(data) => data,
        Data::Enum(_) | Data::Union(_) => {
            return Err(Error::new(
                name.span(),
                "SystemData can only be derived for structs",
            ))
        }
    };

    let field_names: Vec<_> = data
        .fields
        .iter()
        .enumerate()
        .map(|(i, field)| match &field.ident {
            Some(ident) => quote! { #ident },
            None => quote! { #i },
        })
        .collect();

    let field_types: Vec<_> = data.fields.iter().map(|field| &field.ty).collect();

    let initializers = match &data.fields {
        Fields::Named(_) => quote! {
            Self {
                #(
                    #field_names: <#field_types as #krate::ecs::SystemData>::fetch(world),
                )*
            }
        },
        Fields::Unnamed(_) => quote! {
            Self(#( <#field_types as #krate::ecs::SystemData>::fetch(world) ),*)
        },
        Fields::Unit => unreachable!(),
    };

    Ok(quote! {
        impl #impl_generics #krate::ecs::SystemData<#lifetime> for #name #ty_generics
        #where_clause #( #bounds, )*
        {
            fn fetch(world: & #lifetime #krate::ecs::World) -> Self {
                #initializers
            }

            fn setup(world: &mut World) {
                #( <#field_types as #krate::ecs::SystemData>::setup(world); )*
            }

            fn reads(types: &mut #krate::__export::std::vec::Vec<#krate::__export::std::any::TypeId>) {
                #( <#field_types as #krate::ecs::SystemData>::reads(types); )*
            }

            fn writes(types: &mut #krate::__export::std::vec::Vec<#krate::__export::std::any::TypeId>) {
                #( <#field_types as #krate::ecs::SystemData>::writes(types); )*
            }
        }
    })
}
