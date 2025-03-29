use crate::statement::implicit_return;
use crate::{expression, identifier, token};
use proc_macro2::{Ident, Span};
use syn::punctuated::Punctuated;
use syn::{
    Block, Expr, ExprCall, ExprIf, ExprLit, ExprMethodCall, ExprPath, ExprReference, Lit, LitBool,
    Path, Token,
};

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
    path(Path::from(identifier!(self)))
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

pub fn reference(referend: Expr) -> Expr {
    Expr::Reference(ExprReference {
        attrs: Vec::new(),
        and_token: token![&],
        mutability: None,
        expr: Box::new(referend),
    })
}

pub fn call(function: Path, arguments: Punctuated<Expr, Token![,]>) -> Expr {
    Expr::Call(ExprCall {
        attrs: Vec::new(),
        func: Box::new(expression::path(function)),
        paren_token: token![()],
        args: arguments,
    })
}

pub fn call_method(receiver: Expr, method: Ident, args: Punctuated<Expr, Token![,]>) -> Expr {
    Expr::MethodCall(ExprMethodCall {
        attrs: Vec::new(),
        receiver: Box::new(receiver),
        dot_token: token![.],
        method,
        turbofish: None,
        paren_token: token![()],
        args,
    })
}
