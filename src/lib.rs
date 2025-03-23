mod algebraic;
mod r#trait;
mod util;

extern crate proc_macro;

use crate::algebraic::AlgebraicItem;
use crate::r#trait::List;
use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::spanned::Spanned;
use syn::{Result, parse2};

#[proc_macro_attribute]
pub fn implement(
    attribute: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attribute = TokenStream::from(attribute);
    let item = TokenStream::from(item);

    implement_2(attribute, item)
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

fn implement_2(attribute: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let item_span = item.span();

    let traits = parse2::<List>(attribute)?;
    let item = parse2::<AlgebraicItem>(item)?;

    let mut output = quote_spanned! { item_span => #item };

    for r#trait in traits {
        output.extend(r#trait.implement(&item)?);
    }

    Ok(output)
}
