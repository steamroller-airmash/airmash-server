
mod handler;

#[proc_macro_attribute]
pub fn handler(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  handler::handler(attr, item)
}
