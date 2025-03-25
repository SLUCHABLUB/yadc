use crate::attribute::Attribute;
use crate::variant::Variant;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Error, ItemEnum, Result};

pub enum AlgebraicItem {
    Enum {
        attributes: Vec<Attribute>,
        name: Ident,
        variants: Vec<Variant>,
    },
    Struct(Variant),
}

impl AlgebraicItem {
    pub fn name(&self) -> &Ident {
        match self {
            AlgebraicItem::Enum { name, .. } => name,
            AlgebraicItem::Struct(variant) => &variant.name,
        }
    }

    /// The attributes that are applied to the type
    pub fn attributes(&self) -> &[Attribute] {
        match self {
            AlgebraicItem::Enum { attributes, .. } => attributes,
            AlgebraicItem::Struct(variant) => &variant.attributes,
        }
    }

    pub fn map_variants<F>(&self, mut function: F) -> TokenStream
    where
        F: FnMut(&Variant) -> TokenStream,
    {
        match self {
            AlgebraicItem::Enum {
                attributes: _,
                name,
                variants,
            } => {
                let arms = variants
                    .iter()
                    .map(|variant| {
                        let pattern = variant.pattern();
                        let expression = function(variant);

                        quote!(#name::#pattern => #expression)
                    })
                    .collect::<Punctuated<TokenStream, Comma>>();

                quote! {
                    match self {
                        #arms
                    }
                }
            }
            AlgebraicItem::Struct(variant) => {
                let pattern = variant.pattern();
                let expression = function(variant);

                quote! {
                    let #pattern = self;
                    #expression
                }
            }
        }
    }
}

impl TryFrom<ItemEnum> for AlgebraicItem {
    type Error = Error;

    fn try_from(item: ItemEnum) -> Result<Self> {
        Ok(AlgebraicItem::Enum {
            attributes: Attribute::from_vec(item.attrs)?,
            name: item.ident,
            variants: Variant::from_list(item.variants)?,
        })
    }
}
