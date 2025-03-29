use proc_macro2::Span;
use syn::Error;
use syn::spanned::Spanned;

fn combine(mut first: Error, second: Error) -> Error {
    first.combine(second);
    first
}

pub fn key_set_twice<A: Spanned, B: Spanned>(key: &str, first: &A, second: &B) -> Error {
    let message = format!("configuration key `{key}` has been set multiple times");

    combine(
        Error::new(first.span(), &message),
        Error::new(second.span(), message),
    )
}

pub fn path_too_long(span: Span) -> Error {
    Error::new(span, "configuration-key paths must be 2 or 1 segments long")
}
