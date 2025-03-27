use crate::util::{to_argument, token, type_named};
use crate::{AlgebraicItem, TypeConfig, Variant, punctuated};
use itertools::Itertools;
use proc_macro2::Ident;
use std::mem::take;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    AngleBracketedGenericArguments, Error, GenericArgument, GenericParam, Generics, Item, Path,
    PathArguments, PathSegment, PredicateType, Token, TraitBound, TraitBoundModifier, Type,
    TypeParamBound, TypePath, WhereClause, WherePredicate,
};

const BAD_ITEM_KIND: &str = "yadc can only implement traits for enums and structs";
const NON_EXHAUSTIVE: &str = "item kind is not recognised, please open an issue";

/// An algebraic item with information about generics.
pub struct Parameterised {
    pub item: AlgebraicItem,
    pub parameters: Punctuated<GenericParam, Token![,]>,
    pub where_stem: Punctuated<WherePredicate, Token![,]>,
}

impl Parameterised {
    pub fn arguments(&self) -> Punctuated<GenericArgument, Token![,]> {
        self.parameters.iter().cloned().map(to_argument).collect()
    }

    pub fn bound_all(&self, trait_path: Path) -> Punctuated<WherePredicate, Token![,]> {
        let bound = TypeParamBound::Trait(TraitBound {
            paren_token: None,
            modifier: TraitBoundModifier::None,
            lifetimes: None,
            path: trait_path,
        });

        self.type_arguments()
            .map(|ty| {
                WherePredicate::Type(PredicateType {
                    lifetimes: None,
                    bounded_ty: type_named(ty),
                    colon_token: token![:],
                    bounds: punctuated![bound.clone()],
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
        bounds: Punctuated<WherePredicate, Token![,]>,
    ) -> WhereClause {
        let mut predicates = self.where_stem.clone();
        predicates.extend(bounds);
        WhereClause {
            where_token: token![where],
            predicates,
        }
    }

    pub fn to_type(&self) -> Type {
        let arguments = self.type_arguments().collect_vec();
        let arguments = if arguments.is_empty() {
            PathArguments::None
        } else {
            PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: token![<],
                args: self.arguments(),
                gt_token: token![>],
            })
        };

        Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: punctuated![PathSegment {
                    ident: self.item.name().clone(),
                    arguments,
                }],
            },
        })
    }
}

fn extract(
    generics: &mut Generics,
) -> (
    Punctuated<GenericParam, Token![,]>,
    Punctuated<WherePredicate, Token![,]>,
) {
    let generics = take(generics);

    let default = || WhereClause {
        where_token: token![where],
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
                let config = TypeConfig::try_from(item.attrs.clone())?;
                let variant = Variant::try_from(item)?;

                Ok(Parameterised {
                    item: AlgebraicItem::Struct { config, variant },
                    parameters,
                    where_stem,
                })
            }

            Item::Verbatim(_) | _ => Err(Error::new(item.span(), NON_EXHAUSTIVE)),
        }
    }
}
