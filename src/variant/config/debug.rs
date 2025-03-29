use crate::{define_config};
use syn::Expr;
use crate::expression::false_;

define_config! {
    non_exhaustive: Expr = false_(),
}
