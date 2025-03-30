use crate::expression::variable;
use crate::macros::token;
use crate::pattern;
use proc_macro2::Ident;
use syn::{Expr, Local, LocalInit, Stmt, Type};

#[derive(Clone)]
pub struct Value {
    name: fn() -> Ident,
    ty: fn() -> Type,
}

impl Value {
    pub const fn __new(name: fn() -> Ident, ty: fn() -> Type) -> Value {
        Value { name, ty }
    }

    pub fn name(&self) -> Ident {
        (self.name)()
    }

    pub fn ty(&self) -> Type {
        (self.ty)()
    }

    pub fn expression(&self) -> Expr {
        variable(self.name())
    }

    pub fn let_mut(&self, initialiser: Expr) -> Stmt {
        Stmt::Local(Local {
            attrs: Vec::new(),
            let_token: token![let],
            pat: pattern::mutable(self.name()),
            init: Some(LocalInit {
                eq_token: token![=],
                expr: Box::new(initialiser),
                diverge: None,
            }),
            semi_token: token![;],
        })
    }
}

macro_rules! value {
    ($name:ident : $ty:expr) => {
        crate::Value::__new(|| identifier!($name), || $ty)
    };
}

pub(crate) use value;
