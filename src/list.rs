use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::{IntoIter, Punctuated};
use syn::{Result, Token};

type Comma = Token![,];

#[derive(Clone, Default)]
pub struct List<T>(Punctuated<T, Comma>);

impl<T> List<T> {
    pub const fn new() -> List<T> {
        List(Punctuated::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: Parse> Parse for List<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse_terminated(T::parse, Token![,]).map(List)
    }
}

impl<T: ToTokens> ToTokens for List<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> IntoIter<T> {
        self.0.into_iter()
    }
}

impl<T> From<List<T>> for Punctuated<T, Token![,]> {
    fn from(list: List<T>) -> Self {
        list.0
    }
}
