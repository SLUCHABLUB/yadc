use crate::attribute::Attribute;
use crate::field::Fields;
use proc_macro2::Ident;
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma, Paren};
use syn::{
    Error, FieldPat, ItemStruct, Member, Pat, PatIdent, PatPath, PatStruct, PatTupleStruct, Path,
    Result,
};

/// A `struct` or an `enum` variant.
pub struct Variant {
    pub attributes: Vec<Attribute>,
    pub name: Ident,
    pub fields: Fields,
}

impl Variant {
    pub fn from_list(list: Punctuated<syn::Variant, Comma>) -> Result<Vec<Variant>> {
        list.into_iter().map(Variant::try_from).collect()
    }

    /// The pattern to destruct this variant
    pub fn pattern(&self) -> Pat {
        let path = Path::from(self.name.clone());
        let field_names = self.fields.names().into_iter();

        match self.fields {
            Fields::Named(_) => Pat::Struct(PatStruct {
                attrs: Vec::new(),
                qself: None,
                path,
                brace_token: Brace::default(),
                fields: field_names
                    .map(|ident| FieldPat {
                        attrs: vec![],
                        member: Member::Named(ident.clone()),
                        colon_token: None,
                        pat: Box::new(pattern(ident)),
                    })
                    .collect(),
                rest: None,
            }),
            Fields::Unnamed(_) => Pat::TupleStruct(PatTupleStruct {
                attrs: Vec::new(),
                qself: None,
                path,
                paren_token: Paren::default(),
                elems: field_names.map(pattern).collect(),
            }),
            Fields::Unit => Pat::Path(PatPath {
                attrs: Vec::new(),
                qself: None,
                path,
            }),
        }
    }
}

fn pattern(ident: Ident) -> Pat {
    Pat::Ident(PatIdent {
        attrs: Vec::new(),
        by_ref: None,
        mutability: None,
        ident,
        subpat: None,
    })
}

impl TryFrom<ItemStruct> for Variant {
    type Error = Error;

    fn try_from(item: ItemStruct) -> Result<Self> {
        Ok(Variant {
            attributes: Attribute::from_vec(item.attrs)?,
            name: item.ident,
            fields: Fields::try_from(item.fields)?,
        })
    }
}

impl TryFrom<syn::Variant> for Variant {
    type Error = Error;

    fn try_from(variant: syn::Variant) -> Result<Self> {
        Ok(Variant {
            attributes: Attribute::from_vec(variant.attrs)?,
            name: variant.ident,
            fields: Fields::try_from(variant.fields)?,
        })
    }
}
