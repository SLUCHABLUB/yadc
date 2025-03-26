use crate::attribute::Attribute;
use crate::util::{self_expression, token};
use crate::variant::Variant;
use proc_macro2::Ident;
use syn::{Arm, Block, Error, Expr, ExprBlock, ExprMatch, ItemEnum, Result, Stmt};

pub enum AlgebraicItem {
    Enum {
        attributes: Vec<Attribute>,
        name: Ident,
        variants: Vec<Variant>,
    },
    Struct(Variant),
}

impl AlgebraicItem {
    pub fn name(&self) -> &Ident {
        match self {
            AlgebraicItem::Enum { name, .. } => name,
            AlgebraicItem::Struct(variant) => &variant.name,
        }
    }

    /// The attributes that are applied to the type
    pub fn attributes(&self) -> &[Attribute] {
        match self {
            AlgebraicItem::Enum { attributes, .. } => attributes,
            AlgebraicItem::Struct(variant) => &variant.attributes,
        }
    }

    pub fn map_variants<F, Statements>(&self, mut function: F) -> Vec<Stmt>
    where
        F: FnMut(&Variant) -> Statements,
        Statements: IntoIterator<Item = Stmt>,
    {
        match self {
            AlgebraicItem::Enum {
                attributes: _,
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
            AlgebraicItem::Struct(variant) => {
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
            attributes: Attribute::from_vec(item.attrs)?,
            name: item.ident,
            variants: Variant::from_list(item.variants)?,
        })
    }
}
