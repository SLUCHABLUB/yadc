use crate::util::{
    Receiver, call_function, mutable_reference, new_identifier, new_impl_fn, reference,
    self_expression, token, type_named, unit_type, variable_named,
};
use crate::{AlgebraicItem, NamedField, Parameterised, Variant, VariantConfig, path, punctuated};
use itertools::chain;
use proc_macro2::Ident;
use syn::{
    Expr, GenericParam, Generics, ImplItemFn, Stmt, TraitBound, TraitBoundModifier, Type,
    TypeParam, TypeParamBound,
};

pub fn state_type() -> Type {
    mutable_reference(type_named(new_identifier("H")))
}

pub fn state_identifier() -> Ident {
    new_identifier("state")
}

pub fn state() -> Expr {
    variable_named(state_identifier())
}

pub fn hash(parameterised: &Parameterised) -> ImplItemFn {
    new_impl_fn(
        new_identifier("hash"),
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

fn maybe_hash_discriminant(item: &AlgebraicItem) -> Option<Stmt> {
    if matches!(item, AlgebraicItem::Struct { .. }) {
        return None;
    }

    let function = path::core(["mem", "discriminant"]);

    let discriminant = call_function(function, punctuated![self_expression()]);

    Some(hash_expression(reference(discriminant)))
}

fn hash_variant(variant: &Variant) -> Vec<Stmt> {
    let VariantConfig {} = variant.config;

    variant
        .fields
        .clone()
        .into_named()
        .into_iter()
        .map(hash_field)
        .collect()
}

fn hash_field(field: NamedField) -> Stmt {
    hash_expression(variable_named(field.name))
}

fn hash_expression(expression: Expr) -> Stmt {
    let function = path::core(["hash", "Hash", "hash"]);

    let expression = call_function(function, punctuated![expression, state()]);

    Stmt::Expr(expression, Some(token![;]))
}

fn generics() -> Generics {
    let bound = TypeParamBound::Trait(TraitBound {
        paren_token: None,
        modifier: TraitBoundModifier::None,
        lifetimes: None,
        path: path::core(["hash", "Hasher"]),
    });

    let parameter = GenericParam::Type(TypeParam {
        attrs: Vec::new(),
        ident: new_identifier("H"),
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
