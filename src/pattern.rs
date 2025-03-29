use crate::util::token;
use proc_macro2::Ident;
use syn::{Pat, PatIdent};

pub fn variable(name: Ident) -> Pat {
    Pat::Ident(PatIdent {
        attrs: Vec::new(),
        by_ref: None,
        mutability: None,
        ident: name,
        subpat: None,
    })
}

pub fn mutable(name: Ident) -> Pat {
    Pat::Ident(PatIdent {
        attrs: Vec::new(),
        by_ref: None,
        mutability: Some(token![mut]),
        ident: name,
        subpat: None,
    })
}
