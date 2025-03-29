macro_rules! token {
    [()] => { syn::token::Paren::default() };
    [[]] => { syn::token::Bracket::default() };
    [{}] => { syn::token::Brace::default() };
    [$token:tt] => { <syn::Token![$token]>::default() };
}

macro_rules! identifier {
    ($identifier:ident) => {
        syn::Ident::new(stringify!($identifier), proc_macro2::Span::call_site())
    };
}

macro_rules! core_path {
    ($($segment:ident)::*) => {
        syn::Path {
            leading_colon: Some(token![::]),
            segments: crate::punctuated![
                syn::PathSegment::from(crate::identifier!(core)),
                $(
                    syn::PathSegment::from(crate::identifier!($segment))
                ),*
            ],
        }
    };
}

macro_rules! punctuated {
    () => (
        ::syn::punctuated::Punctuated::new()
    );
    ($elem:expr; $n:expr) => (
        ::std::iter::repeat_n($elem, $n).collect()
    );
    ($($x:expr),+ $(,)?) => (
        [$($x),+].into_iter().collect::<::syn::punctuated::Punctuated<_, _>>()
    );
}

pub(crate) use {core_path, identifier, punctuated, token};
