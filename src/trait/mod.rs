mod debug;

use crate::parameterised::Parameterised;
use crate::r#trait::debug::implement_debug;
use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::{IntoIter, Punctuated};
use syn::spanned::Spanned;
use syn::{Error, PathArguments, PathSegment, Result, Token};

pub enum Trait {
    Debug,
}

impl Trait {
    pub fn implement(self, item: &Parameterised) -> TokenStream {
        match self {
            Trait::Debug => implement_debug(item),
        }
    }
}

impl TryFrom<PathSegment> for Trait {
    type Error = Error;

    fn try_from(path: PathSegment) -> Result<Self> {
        let span = path.span();

        match (path.ident.to_string().as_str(), path.arguments) {
            ("Debug", PathArguments::None) => Ok(Trait::Debug),

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

pub struct List(Punctuated<Trait, Token![,]>);

impl Parse for List {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse_terminated(Trait::parse, Token![,]).map(List)
    }
}

impl IntoIterator for List {
    type Item = Trait;
    type IntoIter = IntoIter<Trait>;

    fn into_iter(self) -> IntoIter<Trait> {
        self.0.into_iter()
    }
}
