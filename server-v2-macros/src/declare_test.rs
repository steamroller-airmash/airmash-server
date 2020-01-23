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
        #[tokio::test]
        #vis #sig {
            #( #attrs )*
            #old { #block }

            #krate::run_test(#ident).await
        }
    })
}
