use crate::Trait;
use syn::Token;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::{IntoIter, Punctuated};

pub struct List(Punctuated<Trait, Token![,]>);

impl Parse for List {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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
