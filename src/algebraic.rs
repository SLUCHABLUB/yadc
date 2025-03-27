use crate::util::{self_expression, token};
use crate::{TypeConfig, Variant};
use proc_macro2::Ident;
use syn::{Arm, Block, Error, Expr, ExprBlock, ExprMatch, ItemEnum, Result, Stmt};

pub enum AlgebraicItem {
    Enum {
        config: TypeConfig,
        name: Ident,
        variants: Vec<Variant>,
    },
    Struct {
        config: TypeConfig,
        variant: Variant,
    },
}

impl AlgebraicItem {
    pub fn name(&self) -> &Ident {
        match self {
            AlgebraicItem::Enum { name, .. } => name,
            AlgebraicItem::Struct { variant, .. } => &variant.name,
        }
    }

    /// The attributes that are applied to the type
    pub fn config(&self) -> &TypeConfig {
        let (AlgebraicItem::Enum { config, .. } | AlgebraicItem::Struct { config, .. }) = self;
        config
    }

    pub fn map_variants<F, Statements>(&self, mut function: F) -> Vec<Stmt>
    where
        F: FnMut(&Variant) -> Statements,
        Statements: IntoIterator<Item = Stmt>,
    {
        match self {
            AlgebraicItem::Enum {
                config: _,
                name,
                variants,
            } => {
                vec![Stmt::Expr(
                    Expr::Match(ExprMatch {
                        attrs: Vec::new(),
                        match_token: token![match],
                        expr: Box::new(self_expression()),
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
            AlgebraicItem::Struct { variant, .. } => {
                let destruct = variant.destruct_self();

                let mut statements = vec![destruct];
                statements.extend(function(variant));
                statements
            }
        }
    }
}

impl TryFrom<ItemEnum> for AlgebraicItem {
    type Error = Error;

    fn try_from(item: ItemEnum) -> Result<Self> {
        Ok(AlgebraicItem::Enum {
            config: TypeConfig::try_from(item.attrs)?,
            name: item.ident,
            variants: Variant::from_list(item.variants)?,
        })
    }
}
