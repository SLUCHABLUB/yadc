use crate::{define_config};
use syn::{parse_quote, LitBool};

define_config! {
    skip: LitBool = parse_quote!(false),
}
