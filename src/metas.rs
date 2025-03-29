use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Meta, Result, Token, parse2};

pub fn parse(tokens: TokenStream) -> Result<Punctuated<Meta, Token![,]>> {
    let Metas(metas) = parse2(tokens)?;
    Ok(metas)
}

struct Metas(Punctuated<Meta, Token![,]>);

impl Parse for Metas {
    fn parse(input: ParseStream) -> Result<Metas> {
        input.parse_terminated(Meta::parse, Token![,]).map(Metas)
    }
}
