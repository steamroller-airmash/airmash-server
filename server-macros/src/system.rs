use crate::macro_args::MacroArgs;
use crate::*;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Error, FnArg, Ident, ItemFn, Result, Type,
};

pub fn system(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let macro_args: MacroArgs = parse_macro_input!(attr as MacroArgs);
    let func: ItemFn = parse_macro_input!(item as ItemFn);

    match system_impl(macro_args, func) {
        Ok(x) => x.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn system_impl(macro_args: MacroArgs, func: ItemFn) -> Result<TokenStream> {
    let specs_name = crate_name("specs").map_err(|e| Error::new(Span::call_site(), e))?;
    let crate_name = crate_name("airmash-server").map_err(|e| Error::new(Span::call_site(), e))?;

    let func_name = func.sig.ident.clone();
    let sys_name = macro_args.name;
    let sys_deps = macro_args.deps.unwrap_or(parse_quote! { () });
    let vis = macro_args.vis.unwrap_or_else(|| parse_quote! { pub });
    let attrs = func.attrs.clone();

    let lifetime = match func.sig.generics.lifetimes().next() {
        Some(x) => x,
        None => {
            return Err(Error::new(
                Span::call_site(),
                "Functional systems must have exactly 1 lifetime parameter",
            ))
        }
    };

    let arg_names: Vec<_> = func
        .sig
        .inputs
        .iter()
        .enumerate()
        .map(|(idx, arg)| Ident::new(&format!("__field{}", idx), arg.span()))
        .collect();
    let (arg_tys, opt_refs): (Vec<Type>, Vec<TokenStream>) = func
        .sig
        .inputs
        .iter()
        .map(|arg: &FnArg| match arg {
            FnArg::Typed(pat) => (dereference(&pat.ty), opt_ref(&pat.ty)),
            _ => unreachable!(),
        })
        .fold(Default::default(), |mut acc, x| {
            acc.0.push(x.0);
            acc.1.push(x.1);
            acc
        });

    let data_name = Ident::new(
        &format!("{}_{}Data_{}", func.sig.ident, sys_name, hash_fn(&func)),
        func.sig.ident.span(),
    );

    Ok(quote! {
        #( #attrs )*
        #[derive(Default)]
        #vis struct #sys_name;

        #[allow(non_camel_case_types)]
        #[derive(SystemDataCustom)]
        #vis struct #data_name<#lifetime> {
            #[allow(unused)]
            _dummy: #specs_name::prelude::Entities<#lifetime>,
            #(
                #arg_names: #arg_tys,
            )*
        }

        impl<#lifetime> #specs_name::prelude::System<#lifetime> for #sys_name {
            type SystemData = #data_name<'a>;

            fn run(&mut self, mut data: Self::SystemData) {
                #func_name(
                    #( #opt_refs data.#arg_names, )*
                )
            }
        }

        impl #crate_name::SystemInfo for #sys_name {
            type Dependencies = #sys_deps;

            fn name() -> &'static str {
                concat!(
                    module_path!(),
                    stringify!(#func_name)
                )
            }

            fn new() -> Self {
                Self::default()
            }
        }

        #func
    })
}

fn dereference(ty: &Type) -> Type {
    match ty {
        Type::Reference(refty) => (*refty.elem).clone(),
        ty => ty.clone(),
    }
}

fn opt_ref(ty: &Type) -> TokenStream {
    match ty {
        Type::Reference(_) => quote! { &mut },
        _ => TokenStream::new(),
    }
}
