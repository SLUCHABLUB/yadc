use proc_macro2::Span;
use crate::{define_config};
use syn::LitBool;

define_config! {
    skip: LitBool = LitBool::new(false, Span::call_site()),
}
