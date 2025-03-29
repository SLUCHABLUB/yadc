mod debug;
mod hash;
mod list;

pub use list::List;

use crate::item::Algebraic;
use crate::{Parameterised, core_path, token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    Error, Generics, ItemImpl, Path, PathArguments, PathSegment, Result, Token, WherePredicate,
    parse_quote,
};

#[derive(Clone, Eq, PartialEq)]
pub enum Trait {
    Debug,
    Hash,
}

impl Trait {
    pub fn implement(self, parameterised: &Parameterised) -> ItemImpl {
        let items = match self {
            Trait::Debug => vec![debug::fmt(parameterised).into()],
            Trait::Hash => vec![hash::hash(parameterised).into()],
        };

        let bounds = self.bounds(&parameterised.item);
        let where_clause = parameterised.where_clause_with_bounds(bounds);

        ItemImpl {
            attrs: vec![parse_quote!(#[automatically_derived])],
            defaultness: None,
            unsafety: None,
            impl_token: token![impl],
            generics: Generics {
                lt_token: Some(token![<]),
                params: parameterised.parameters.clone(),
                gt_token: Some(token![>]),
                where_clause: Some(where_clause),
            },
            trait_: Some((None, self.path(), token![for])),
            self_ty: Box::new(parameterised.to_type()),
            brace_token: token![{}],
            items,
        }
    }

    fn path(&self) -> Path {
        match self {
            Trait::Debug => core_path!(fmt::Debug),
            Trait::Hash => core_path!(hash::Hash),
        }
    }

    fn bounds(&self, item: &Algebraic) -> Punctuated<WherePredicate, Token![,]> {
        match self {
            Trait::Debug => debug::bounds(item),
            Trait::Hash => hash::bounds(item),
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
