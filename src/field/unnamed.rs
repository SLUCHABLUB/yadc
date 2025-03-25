use crate::attribute::Attribute;
use crate::field::named::NamedField;
use quote::format_ident;
use syn::{Error, Field, Type};

#[derive(Clone)]
pub struct UnnamedField {
    pub attributes: Vec<Attribute>,
    pub ty: Type,
}

impl UnnamedField {
    pub fn into_named(self, index: usize) -> NamedField {
        NamedField {
            attributes: self.attributes,
            name: format_ident!("value_{index}"),
            ty: self.ty,
        }
    }
}

impl TryFrom<Field> for UnnamedField {
    type Error = Error;

    fn try_from(field: Field) -> syn::Result<Self> {
        Ok(UnnamedField {
            attributes: Attribute::from_vec(field.attrs)?,
            ty: field.ty,
        })
    }
}
