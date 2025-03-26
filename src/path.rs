use crate::util::{new_identifier, single, token};
use proc_macro2::{Ident, Span};
use syn::{Expr, ExprPath, Path, PathSegment};

pub fn new<const N: usize>(segments: [&str; N]) -> Path {
    Path {
        leading_colon: None,
        segments: segments
            .into_iter()
            .map(|name| PathSegment::from(Ident::new(name, Span::call_site())))
            .collect(),
    }
}

pub fn core<const N: usize>(segments: [&str; N]) -> Path {
    let string = segments;
    let mut segments = single(PathSegment::from(new_identifier("core")));

    for string in string {
        segments.push(PathSegment::from(new_identifier(string)));
    }

    Path {
        leading_colon: Some(token![::]),
        segments,
    }
}

pub fn expression(path: Path) -> Expr {
    Expr::Path(ExprPath {
        attrs: Vec::new(),
        qself: None,
        path,
    })
}
