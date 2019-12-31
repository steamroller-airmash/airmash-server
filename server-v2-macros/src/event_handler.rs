use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::spanned::Spanned;
use syn::{parse_quote, Error, FnArg, Ident, ItemFn, Token, Type, Visibility};

use crate::util::*;

pub fn event_handler(attr: TokenStream, input: TokenStream) -> TokenStream {
    match event_handler_impl(input, attr) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
}

fn event_handler_impl(input: TokenStream, attr: TokenStream) -> Result<TokenStream, Error> {
    let func: ItemFn = syn::parse2(input)?;
    let args: MacroArgs = syn::parse2(attr)?;

    let krate = crate::crate_name("server-v2");

    let func_name = func.sig.ident.clone();
    let sys_name = args
        .name
        .map(|arg| arg.value)
        .unwrap_or_else(|| func_name.clone());
    let sys_deps = args
        .deps
        .map(|arg| arg.value)
        .unwrap_or_else(|| parse_quote! { () });
    let vis = args
        .vis
        .map(|arg| arg.value)
        .unwrap_or_else(|| parse_quote! { pub });

    let lifetime = match func.sig.generics.lifetimes().next() {
        Some(lifetime) => lifetime,
        None => {
            return Err(Error::new(
                Span::call_site(),
                "Must have at least 1 lifetime parameter",
            ))
        }
    };

    let generics = &func.sig.generics;
    let (impl_generics, ty_generics, where_clause) = func.sig.generics.split_for_impl();

    let sys_generics = strip_lifetimes(func.sig.generics.clone());
    let (sys_impl, sys_ty, sys_where) = sys_generics.split_for_impl();
    let phantom_ty = as_phantomdata(&sys_generics);

    let skipnum = match &args.state {
        Some(_) => 2,
        None => 1,
    };

    let arg_names: Vec<_> = func
        .sig
        .inputs
        .iter()
        .enumerate()
        .skip(skipnum)
        .map(|(idx, arg)| Ident::new(&format!("__field{}", idx), arg.span()))
        .collect();
    let (arg_tys, opt_refs): (Vec<Type>, Vec<TokenStream>) = func
        .sig
        .inputs
        .iter()
        .skip(skipnum)
        .map(|arg: &FnArg| match arg {
            FnArg::Typed(pat) => (dereference(&pat.ty), opt_ref(&pat.ty)),
            _ => unreachable!(),
        })
        .fold(Default::default(), |mut acc, x| {
            acc.0.push(x.0);
            acc.1.push(x.1);
            acc
        });

    let event_ty = match &args.state {
        Some(_) => func.sig.inputs.iter().skip(1).next(),
        None => func.sig.inputs.first(),
    };

    let event_ty = match event_ty {
        Some(arg) => match arg {
            FnArg::Typed(pat) => dereference(&pat.ty),
            _ => unreachable!(),
        },
        None => {
            return Err(Error::new(
                Span::call_site(),
                "must have at least one argument to specify the event type",
            ))
        }
    };

    let data_name = Ident::new(
        &format!("{}_{}Data_{}", func.sig.ident, sys_name, hash_fn(&func)),
        func.sig.ident.span(),
    );

    let decls = if let Some(state) = &args.state {
        let state_ty = &state.value;
        quote! {
            #[derive(Default)]
            #[allow(non_camel_case_types)]
            #vis struct #sys_name #sys_generics {
                reader: Option<#krate::__export::shrev::ReaderId<#event_ty>>,
                state: #state_ty,
                _marker: #phantom_ty
            }

            impl #impl_generics #krate::ecs::System<#lifetime> for #sys_name #sys_ty
            #where_clause
            {
                type SystemData = (
                    #krate::ecs::Read<#lifetime, #krate::__export::shrev::EventChannel<#event_ty>>,
                    #data_name #ty_generics
                );

                fn setup(&mut self, world: &mut #krate::ecs::World) {
                    <Self::SystemData as #krate::ecs::SystemData>::setup(world);

                    self.state.setup(world);

                    let mut channel = world.fetch_resource_mut::<#krate::__export::shrev::EventChannel<#event_ty>>();
                    self.reader = Some(channel.register_reader());
                }

                fn run(&mut self, (channel, mut data): Self::SystemData) {
                    let reader = self.reader.as_mut().unwrap();

                    for evt in channel.read(reader) {
                        #func_name(
                            &mut self.state,
                            evt,
                            #( #opt_refs data.#arg_names ),*
                        );
                    }
                }
            }
        }
    } else {
        quote! {
            #[derive(Default)]
            #[allow(non_camel_case_types)]
            #vis struct #sys_name #sys_generics {
                reader: Option<#krate::__export::shrev::ReaderId<#event_ty>>,
                _marker: #phantom_ty
            }

            impl #impl_generics #krate::ecs::System<#lifetime> for #sys_name #sys_ty
            #where_clause
            {
                type SystemData = (
                    #krate::ecs::Read<#lifetime, #krate::__export::shrev::EventChannel<#event_ty>>,
                    #data_name #ty_generics
                );

                fn setup(&mut self, world: &mut #krate::ecs::World) {
                    <Self::SystemData as #krate::ecs::SystemData>::setup(world);

                    let mut channel = world.fetch_resource_mut::<#krate::__export::shrev::EventChannel<#event_ty>>();
                    self.reader = Some(channel.register_reader());
                }

                fn run(&mut self, (channel, mut data): Self::SystemData) {
                    let reader = self.reader.as_mut().unwrap();

                    for evt in channel.read(reader) {
                        #func_name(
                            evt,
                            #( #opt_refs data.#arg_names ),*
                        );
                    }
                }
            }
        }
    };

    let res = quote! {
        #decls

        #[allow(non_camel_case_types)]
        #[derive(SystemData)]
        #vis struct #data_name #generics {
            #[allow(unused)]
            _dummy: ::core::marker::PhantomData<&#lifetime ()>,
            #(
                #arg_names: #arg_tys,
            )*
        }

        impl #sys_impl #krate::ecs::SystemBuilder for #sys_name #sys_ty
        #sys_where
        {
            type System = Self;
            type Dependencies = #sys_deps;

            fn build(self) -> Self { self }
        }

        #func
    };

    Ok(res)
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

fn hash_fn(func: &syn::ItemFn) -> u64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = fxhash::FxHasher::default();
    func.hash(&mut hasher);

    hasher.finish()
}

#[derive(Default)]
struct MacroArgs {
    name: Option<NamedArg<Ident>>,
    deps: Option<NamedArg<Type>>,
    vis: Option<NamedArg<Visibility>>,
    state: Option<NamedArg<Type>>,
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut args = Self::default();

        while input.peek(Ident) {
            let name: Ident = input.fork().parse()?;

            match &*name.to_string() {
                "name" => parse_once!(args.name, input),
                "deps" => parse_once!(args.deps, input),
                "vis" => parse_once!(args.vis, input),
                "state" => parse_once!(args.state, input),
                _ => {
                    return Err(Error::new(
                        name.span(),
                        format!("unknown argument `{}`", name),
                    ))
                }
            }

            if input.peek2(Ident) {
                let _: Token![,] = input.parse()?;
            }
        }

        Ok(args)
    }
}
