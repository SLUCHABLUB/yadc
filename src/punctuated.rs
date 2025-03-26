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

pub(crate) use punctuated;
