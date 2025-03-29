use crate::expression::{call, reference, self_, variable};
use crate::traits::Trait;
use crate::util::{Receiver, bound_type, mutable_reference, new_impl_fn, type_named, unit_type};
use crate::{
    Algebraic, NamedField, Parameterised, Variant, core_path, field, identifier, item, punctuated,
    token, variant,
};
use itertools::chain;
use proc_macro2::Ident;
use syn::punctuated::Punctuated;
use syn::{
    Expr, GenericParam, Generics, ImplItemFn, Stmt, Token, TraitBound, TraitBoundModifier, Type,
    TypeParam, TypeParamBound, WherePredicate,
};

fn state_type() -> Type {
    mutable_reference(type_named(identifier!(H)))
}

fn state_identifier() -> Ident {
    identifier!(state)
}

fn state_expression() -> Expr {
    variable(state_identifier())
}

pub fn hash(parameterised: &Parameterised) -> ImplItemFn {
    let item::hash::Config {} = parameterised.item.config().hash;

    new_impl_fn(
        identifier!(hash),
        generics(),
        Receiver::Reference,
        [(state_identifier(), state_type())],
        unit_type(),
        chain(
            maybe_hash_discriminant(&parameterised.item),
            parameterised.item.map_variants(hash_variant),
        ),
    )
}

fn maybe_hash_discriminant(item: &Algebraic) -> Option<Stmt> {
    let item::hash::Config {} = item.config().hash;

    if matches!(item, Algebraic::Struct { .. }) {
        return None;
    }

    let function = core_path!(mem::discriminant);

    let discriminant = call(function, punctuated![self_()]);

    Some(hash_expression(reference(discriminant)))
}

fn hash_variant(variant: &Variant) -> Vec<Stmt> {
    let variant::hash::Config {} = variant.config.hash;

    variant
        .fields
        .clone()
        .into_named()
        .into_iter()
        .map(hash_field)
        .collect()
}

fn hash_field(field: NamedField) -> Stmt {
    let field::hash::Config {} = field.config.hash;

    hash_expression(variable(field.name))
}

fn hash_expression(expression: Expr) -> Stmt {
    let function = core_path!(hash::Hash::hash);

    let expression = call(function, punctuated![expression, state_expression()]);

    Stmt::Expr(expression, Some(token![;]))
}

/// The generics for the `Hash::hash` function.
fn generics() -> Generics {
    let bound = TypeParamBound::Trait(TraitBound {
        paren_token: None,
        modifier: TraitBoundModifier::None,
        lifetimes: None,
        path: core_path!(hash::Hasher),
    });

    let parameter = GenericParam::Type(TypeParam {
        attrs: Vec::new(),
        ident: identifier!(H),
        colon_token: Some(token![:]),
        bounds: punctuated![bound],
        eq_token: None,
        default: None,
    });

    Generics {
        lt_token: Some(token![<]),
        params: punctuated![parameter],
        gt_token: Some(token![>]),
        where_clause: None,
    }
}

pub fn bounds(item: &Algebraic) -> Punctuated<WherePredicate, Token![,]> {
    let item::hash::Config {} = item.config().hash;

    let mut bounds = Punctuated::new();

    for variant in item.variants() {
        let variant::hash::Config {} = variant.config.hash;

        for field in variant.fields.clone().into_named() {
            let field::hash::Config {} = field.config.hash;

            bounds.push(bound_type(field.ty, Trait::Hash.path()));
        }
    }

    bounds
}
