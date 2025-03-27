#![deny(clippy::pedantic)]

extern crate proc_macro;

mod algebraic;
mod config;
mod field;
mod parameterised;
mod path;
mod punctuated;
mod traits;
mod util;
mod variant;

pub(crate) use algebraic::AlgebraicItem;
pub(crate) use config::{FieldConfig, TypeConfig, VariantConfig};
pub(crate) use field::{Fields, NamedField};
pub(crate) use parameterised::Parameterised;
pub(crate) use punctuated::punctuated;
pub(crate) use traits::{List, Trait};
pub(crate) use variant::Variant;

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
