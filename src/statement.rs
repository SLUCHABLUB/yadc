use crate::token;
use syn::{Expr, Stmt};

pub fn new(expression: Expr) -> Stmt {
    Stmt::Expr(expression, Some(token![;]))
}

pub fn implicit_return(expression: Expr) -> Stmt {
    Stmt::Expr(expression, None)
}
