use crate::field;
use proc_macro2::Ident;
use syn::spanned::Spanned;
use syn::{Error, Field, Type};

#[derive(Clone)]
pub struct NamedField {
    pub config: field::Config,
    pub name: Ident,
    pub ty: Type,
}

const EXPECTED_FIELD_NAME: &str = "expected a field name";

impl TryFrom<Field> for NamedField {
    type Error = Error;

    fn try_from(field: Field) -> syn::Result<Self> {
        let error = Error::new(field.span(), EXPECTED_FIELD_NAME);

        Ok(NamedField {
            config: field::Config::try_from(field.attrs)?,
            name: field.ident.ok_or(error)?,
            ty: field.ty,
        })
    }
}
