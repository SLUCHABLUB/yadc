use crate::FieldConfig;
use proc_macro2::Ident;
use syn::spanned::Spanned;
use syn::{Error, Field, Type};

#[derive(Clone)]
pub struct NamedField {
    pub config: FieldConfig,
    pub name: Ident,
    #[expect(
        unused,
        reason = "no implementations care for the types of fields (yet)"
    )]
    pub ty: Type,
}

const EXPECTED_FIELD_NAME: &str = "expected a field name";

impl TryFrom<Field> for NamedField {
    type Error = Error;

    fn try_from(field: Field) -> syn::Result<Self> {
        let error = Error::new(field.span(), EXPECTED_FIELD_NAME);

        Ok(NamedField {
            config: FieldConfig::try_from(field.attrs)?,
            name: field.ident.ok_or(error)?,
            ty: field.ty,
        })
    }
}
