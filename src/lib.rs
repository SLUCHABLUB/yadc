#![deny(clippy::pedantic)]

extern crate proc_macro;

mod config;
mod error;
mod expression;
mod field;
mod item;
mod macros;
mod metas;
mod parameterised;
mod path;
mod pattern;
mod statement;
mod traits;
mod util;
mod value;
mod variant;

pub(crate) use config::define_config;
pub(crate) use field::{Fields, NamedField};
pub(crate) use item::Algebraic;
pub(crate) use macros::{core_path, identifier, punctuated, token};
pub(crate) use parameterised::Parameterised;
pub(crate) use traits::{List, Trait};
pub(crate) use value::{Value, value};
pub(crate) use variant::Variant;

use proc_macro2::TokenStream;
use quote::ToTokens;
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
    let traits = parse2::<List>(attribute)?;
    let item = parse2::<Item>(item)?;

    let mut output_item = item.clone();

    item::remove_attributes(&mut output_item);

    // TODO: remove helper attributes
    let mut output = output_item.into_token_stream();

    let parameterised = Parameterised::try_from(item)?;

    for r#trait in traits {
        output.extend(r#trait.implement(&parameterised).into_token_stream());
    }

    Ok(output)
}
