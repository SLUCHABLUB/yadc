use crate::algebraic::AlgebraicItem;
use crate::util::{single, to_argument, type_named};
use crate::variant::Variant;
use proc_macro2::Ident;
use std::mem::take;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Colon, Comma, Where};
use syn::{
    Error, GenericArgument, GenericParam, Generics, Item, Path, PredicateType, TraitBound,
    TraitBoundModifier, Type, TypeParamBound, WhereClause, WherePredicate,
};

const BAD_ITEM_KIND: &str = "yadc can only implement traits for enums and structs";
const NON_EXHAUSTIVE: &str = "item kind is not recognised, please open an issue";

/// An algebraic item with information about generics.
pub struct Parameterised {
    pub item: AlgebraicItem,
    pub parameters: Punctuated<GenericParam, Comma>,
    pub where_stem: Punctuated<WherePredicate, Comma>,
}

impl Parameterised {
    pub fn arguments(&self) -> Punctuated<GenericArgument, Comma> {
        self.parameters.iter().cloned().map(to_argument).collect()
    }

    pub fn bound_all(&self, trait_path: &Path) -> Punctuated<WherePredicate, Comma> {
        let bound = TypeParamBound::Trait(TraitBound {
            paren_token: None,
            modifier: TraitBoundModifier::None,
            lifetimes: None,
            path: trait_path.clone(),
        });

        self.type_arguments()
            .map(|ty| {
                WherePredicate::Type(PredicateType {
                    lifetimes: None,
                    bounded_ty: type_named(ty),
                    colon_token: Colon::default(),
                    bounds: single(bound.clone()),
                })
            })
            .collect()
    }

    fn type_arguments(&self) -> impl Iterator<Item = Ident> {
        self.arguments()
            .into_iter()
            .filter_map(|argument| match argument {
                GenericArgument::Type(Type::Path(path)) => path.path.get_ident().cloned(),
                _ => None,
            })
    }

    pub fn where_clause_with_bounds(
        &self,
        bounds: Punctuated<WherePredicate, Comma>,
    ) -> WhereClause {
        let mut predicates = self.where_stem.clone();
        predicates.extend(bounds);
        WhereClause {
            where_token: Where::default(),
            predicates,
        }
    }
}

fn extract(
    generics: &mut Generics,
) -> (
    Punctuated<GenericParam, Comma>,
    Punctuated<WherePredicate, Comma>,
) {
    let generics = take(generics);

    let default = || WhereClause {
        where_token: Where::default(),
        predicates: Punctuated::new(),
    };

    (
        generics.params,
        generics.where_clause.unwrap_or_else(default).predicates,
    )
}

impl TryFrom<Item> for Parameterised {
    type Error = Error;

    fn try_from(item: Item) -> syn::Result<Self> {
        match item {
            Item::Const(_)
            | Item::ExternCrate(_)
            | Item::Fn(_)
            | Item::ForeignMod(_)
            | Item::Impl(_)
            | Item::Macro(_)
            | Item::Mod(_)
            | Item::Static(_)
            | Item::Trait(_)
            | Item::TraitAlias(_)
            | Item::Type(_)
            | Item::Union(_)
            | Item::Use(_) => Err(Error::new(item.span(), BAD_ITEM_KIND)),

            Item::Enum(mut item) => {
                let (parameters, where_stem) = extract(&mut item.generics);
                Ok(Parameterised {
                    item: AlgebraicItem::try_from(item)?,
                    parameters,
                    where_stem,
                })
            }
            Item::Struct(mut item) => {
                let (parameters, where_stem) = extract(&mut item.generics);
                let item = Variant::try_from(item).map(AlgebraicItem::Struct)?;
                Ok(Parameterised {
                    item,
                    parameters,
                    where_stem,
                })
            }

            Item::Verbatim(_) | _ => Err(Error::new(item.span(), NON_EXHAUSTIVE)),
        }
    }
}
