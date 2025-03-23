extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn implement(attribute: TokenStream, item: TokenStream) -> TokenStream {
    drop(attribute);
    item
}
