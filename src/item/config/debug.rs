use syn::WherePredicate;
use crate::define_config;
use crate::List;

define_config! {
    bounds: List<WherePredicate> = List::new(), 
}
