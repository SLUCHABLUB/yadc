use syn::{Meta, Result};

/// A helper attribute.
#[derive(Copy, Clone)]
pub enum Attribute {}

impl Attribute {
    pub fn from_vec(attributes: Vec<syn::Attribute>) -> Result<Vec<Attribute>> {
        attributes
            .into_iter()
            .map(Attribute::from_attribute)
            .filter_map(Result::transpose)
            .collect()
    }

    fn from_attribute(attribute: syn::Attribute) -> Result<Option<Attribute>> {
        Attribute::from_meta(attribute.meta)
    }

    #[expect(clippy::unnecessary_wraps, reason = "Attribute is temporarily empty")]
    fn from_meta(meta: Meta) -> Result<Option<Attribute>> {
        drop(meta);
        Ok(None)
    }
}
