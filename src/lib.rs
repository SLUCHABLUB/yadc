#![deny(clippy::pedantic)]

mod algebraic;
mod attribute;
mod field;
mod parameterised;
mod path;
mod punctuated;
mod r#trait;
mod util;
mod variant;

extern crate proc_macro;

use crate::parameterised::Parameterised;
use crate::r#trait::List;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote_spanned};
use syn::spanned::Spanned;
use syn::{Item, Result, parse2};

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
    let item = parse2::<Item>(item)?;

    // TODO: remove helper attributes
    let mut output = quote_spanned! { item_span => #item };

    let parameterised = Parameterised::try_from(item)?;

    for r#trait in traits {
        output.extend(r#trait.implement(&parameterised).to_token_stream());
    }

    Ok(output)
}
