macro_rules! token {
    [()] => { syn::token::Paren::default() };
    [[]] => { syn::token::Bracket::default() };
    [{}] => { syn::token::Brace::default() };
    [$token:tt] => { <syn::Token![$token]>::default() };
}

pub(crate) use token;
