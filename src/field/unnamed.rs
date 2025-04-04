use crate::{NamedField, field};
use quote::format_ident;
use syn::{Error, Field, Result, Type};

#[derive(Clone)]
pub struct UnnamedField {
    pub config: field::Config,
    pub ty: Type,
}

impl UnnamedField {
    pub fn into_named(self, index: usize) -> NamedField {
        NamedField {
            config: self.config,
            name: format_ident!("value_{index}"),
            ty: self.ty,
        }
    }
}

impl TryFrom<Field> for UnnamedField {
    type Error = Error;

    fn try_from(field: Field) -> Result<Self> {
        Ok(UnnamedField {
            config: field::Config::try_from(field.attrs)?,
            ty: field.ty,
        })
    }
}
