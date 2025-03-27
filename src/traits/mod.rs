mod debug;
mod hash;
mod list;

pub use list::List;

use crate::util::{path_attribute, token};
use crate::{Parameterised, path};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Error, Generics, ItemImpl, Path, PathArguments, PathSegment, Result};

fn automatically_derived() -> syn::Attribute {
    path_attribute(path::new(["automatically_derived"]))
}

#[derive(Clone, Eq, PartialEq)]
pub enum Trait {
    Debug,
    Hash,
}

impl Trait {
    pub fn implement(self, item: &Parameterised) -> ItemImpl {
        let items = match self {
            Trait::Debug => vec![debug::fmt(item).into()],
            Trait::Hash => vec![hash::hash(item).into()],
        };

        let bounds = item.bound_all(self.path());
        let where_clause = item.where_clause_with_bounds(bounds);

        ItemImpl {
            attrs: vec![automatically_derived()],
            defaultness: None,
            unsafety: None,
            impl_token: token![impl],
            generics: Generics {
                lt_token: Some(token![<]),
                params: item.parameters.clone(),
                gt_token: Some(token![>]),
                where_clause: Some(where_clause),
            },
            trait_: Some((None, self.path(), token![for])),
            self_ty: Box::new(item.to_type()),
            brace_token: token![{}],
            items,
        }
    }

    pub fn path(&self) -> Path {
        match self {
            Trait::Debug => path::core(["fmt", "Debug"]),
            Trait::Hash => path::core(["hash", "Hash"]),
        }
    }
}

impl TryFrom<PathSegment> for Trait {
    type Error = Error;

    fn try_from(path: PathSegment) -> Result<Self> {
        let span = path.span();

        match (path.ident.to_string().as_str(), path.arguments) {
            ("Debug", PathArguments::None) => Ok(Trait::Debug),
            ("Hash", PathArguments::None) => Ok(Trait::Hash),

            _ => Err(Error::new(
                span,
                format!("`{}` cannot (yet) be derived via yadc", path.ident),
            )),
        }
    }
}

impl Parse for Trait {
    fn parse(input: ParseStream) -> Result<Self> {
        Trait::try_from(PathSegment::parse(input)?)
    }
}
