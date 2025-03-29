use crate::pattern;
use crate::util::token;
use proc_macro2::Ident;
use syn::{Expr, Local, LocalInit, Stmt};

pub fn new(expression: Expr) -> Stmt {
    Stmt::Expr(expression, Some(token![;]))
}

pub fn implicit_return(expression: Expr) -> Stmt {
    Stmt::Expr(expression, None)
}

pub fn let_mut(identifier: Ident, initialiser: Expr) -> Stmt {
    Stmt::Local(Local {
        attrs: Vec::new(),
        let_token: token![let],
        pat: pattern::mutable(identifier),
        init: Some(LocalInit {
            eq_token: token![=],
            expr: Box::new(initialiser),
            diverge: None,
        }),
        semi_token: token![;],
    })
}
