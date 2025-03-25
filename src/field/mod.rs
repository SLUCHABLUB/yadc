mod named;
mod unnamed;

use crate::field::named::NamedField;
use crate::field::unnamed::UnnamedField;
use itertools::Itertools;
use proc_macro2::Ident;
use quote::format_ident;
use syn::{Error, Result};

#[derive(Clone)]
pub enum Fields {
    Named(Vec<NamedField>),
    Unnamed(Vec<UnnamedField>),
    Unit,
}

impl Fields {
    pub fn into_named(self) -> Vec<NamedField> {
        match self {
            Fields::Named(named) => named,
            Fields::Unnamed(unnamed) => unnamed
                .into_iter()
                .enumerate()
                .map(|(index, field)| field.into_named(index))
                .collect(),
            Fields::Unit => Vec::new(),
        }
    }

    pub fn names(&self) -> Vec<Ident> {
        match self {
            Fields::Named(fields) => fields.iter().map(|field| field.name.clone()).collect(),
            Fields::Unnamed(fields) => (0..fields.len())
                .map(|index| format_ident!("value_{index}"))
                .collect(),
            Fields::Unit => Vec::new(),
        }
    }
}

impl TryFrom<syn::Fields> for Fields {
    type Error = Error;

    fn try_from(fields: syn::Fields) -> Result<Self> {
        Ok(match fields {
            syn::Fields::Named(fields) => Fields::Named(
                fields
                    .named
                    .into_iter()
                    .map(NamedField::try_from)
                    .try_collect()?,
            ),
            syn::Fields::Unnamed(fields) => Fields::Unnamed(
                fields
                    .unnamed
                    .into_iter()
                    .map(UnnamedField::try_from)
                    .try_collect()?,
            ),
            syn::Fields::Unit => Fields::Unit,
        })
    }
}
