use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, Error, ItemFn, Signature};

pub fn test(_: TokenStream, input: TokenStream) -> TokenStream {
    match test_impl(input) {
        Ok(tokens) => tokens,
        Err(e) => e.to_compile_error(),
    }
}

fn test_impl(input: TokenStream) -> Result<TokenStream, Error> {
    let func: ItemFn = syn::parse2(input)?;
    let krate = crate::crate_name("server-tests");

    let ItemFn {
        attrs,
        vis,
        mut sig,
        block,
    } = func;

    let old: Signature = sig.clone();
    let ident = sig.ident.clone();

    sig.inputs = Punctuated::new();
    sig.asyncness = None;

    if old.inputs.len() != 1 {
        return Err(Error::new(
            ident.span(),
            "Must have exactly one argument of type TestRunner",
        ));
    }

    if old.asyncness.is_none() {
        return Err(Error::new(
            ident.span(),
            "Client test function must be async",
        ));
    }

    Ok(quote! {
        #[test]
        #vis #sig {
            #( #attrs )*
            #old { #block }

            tokio::runtime::Builder::new()
                .basic_scheduler()
                .enable_all()
                .build()
                .expect("Failed to create runtime")
                .block_on(async { #krate::run_test(#ident).await })
        }
    })
}
