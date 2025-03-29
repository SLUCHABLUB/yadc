use crate::statement::implicit_return;
use crate::{path, token};
use proc_macro2::{Ident, Span};
use syn::{Block, Expr, ExprIf, ExprLit, ExprPath, Lit, LitBool, Path};

pub fn variable(name: Ident) -> Expr {
    path(Path::from(name))
}

pub fn false_() -> Expr {
    Expr::Lit(ExprLit {
        attrs: Vec::new(),
        lit: Lit::Bool(LitBool::new(false, Span::call_site())),
    })
}

pub fn true_() -> Expr {
    Expr::Lit(ExprLit {
        attrs: Vec::new(),
        lit: Lit::Bool(LitBool::new(true, Span::call_site())),
    })
}

pub fn self_() -> Expr {
    path(path::new(["self"]))
}

pub fn if_else(condition: Expr, then: Expr, otherwise: Expr) -> Expr {
    Expr::If(ExprIf {
        attrs: Vec::new(),
        if_token: token![if],
        cond: Box::new(condition),
        then_branch: Block {
            brace_token: token![{}],
            stmts: vec![implicit_return(then)],
        },
        else_branch: Some((token![else], Box::new(otherwise))),
    })
}

pub fn path(path: Path) -> Expr {
    Expr::Path(ExprPath {
        attrs: Vec::new(),
        qself: None,
        path,
    })
}
