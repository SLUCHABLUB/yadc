mod config;
mod remove_attributes;

pub use config::*;
pub use remove_attributes::remove_attributes;

use crate::Variant;
use crate::expression::self_;
use crate::util::token;
use proc_macro2::Ident;
use std::iter::from_fn;
use syn::{Arm, Block, Error, Expr, ExprBlock, ExprMatch, ItemEnum, Result, Stmt};

pub enum Algebraic {
    Enum {
        config: Config,
        name: Ident,
        variants: Vec<Variant>,
    },
    Struct {
        config: Config,
        variant: Variant,
    },
}

impl Algebraic {
    pub fn name(&self) -> &Ident {
        match self {
            Algebraic::Enum { name, .. } => name,
            Algebraic::Struct { variant, .. } => &variant.name,
        }
    }

    pub fn config(&self) -> &Config {
        let (Algebraic::Enum { config, .. } | Algebraic::Struct { config, .. }) = self;
        config
    }

    pub fn variants(&self) -> impl Iterator<Item = &Variant> {
        let mut index = Some(0);

        from_fn(move || match self {
            Algebraic::Enum { variants, .. } => {
                let variant = variants.get(index?);
                index = index?.checked_add(1);
                variant
            }
            Algebraic::Struct { variant, .. } => {
                let variant = index.map(|_| variant);
                index = None;
                variant
            }
        })
    }

    pub fn map_variants<F, Statements>(&self, mut function: F) -> Vec<Stmt>
    where
        F: FnMut(&Variant) -> Statements,
        Statements: IntoIterator<Item = Stmt>,
    {
        match self {
            Algebraic::Enum {
                config: _,
                name,
                variants,
            } => {
                vec![Stmt::Expr(
                    Expr::Match(ExprMatch {
                        attrs: Vec::new(),
                        match_token: token![match],
                        expr: Box::new(self_()),
                        brace_token: token![{}],
                        arms: variants
                            .iter()
                            .map(|variant| Arm {
                                attrs: Vec::new(),
                                pat: variant.pattern(Some(name.clone())),
                                guard: None,
                                fat_arrow_token: token![=>],
                                body: Box::new(Expr::Block(ExprBlock {
                                    attrs: Vec::new(),
                                    label: None,
                                    block: Block {
                                        brace_token: token![{}],
                                        stmts: function(variant).into_iter().collect(),
                                    },
                                })),
                                comma: None,
                            })
                            .collect(),
                    }),
                    None,
                )]
            }
            Algebraic::Struct { variant, .. } => {
                let destruct = variant.destruct_self();

                let mut statements = vec![destruct];
                statements.extend(function(variant));
                statements
            }
        }
    }
}

impl TryFrom<ItemEnum> for Algebraic {
    type Error = Error;

    fn try_from(item: ItemEnum) -> Result<Self> {
        Ok(Algebraic::Enum {
            config: Config::try_from(item.attrs)?,
            name: item.ident,
            variants: Variant::from_list(item.variants)?,
        })
    }
}
