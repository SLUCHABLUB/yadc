use crate::algebraic::AlgebraicItem;
use crate::field::NamedField;
use crate::parameterised::Parameterised;
use crate::util::{
    Receiver, call_method, expression_path, new_identifier, new_impl_fn, new_path, self_expression,
    single, token, type_named, unit_type, variable_named,
};
use crate::variant::Variant;
use itertools::chain;
use syn::{
    Expr, ExprCall, GenericParam, Generics, ImplItemFn, Stmt, TraitBound, TraitBoundModifier, Type,
    TypeParam, TypeParamBound, TypeReference,
};

pub fn state_type() -> Type {
    Type::Reference(TypeReference {
        and_token: token![&],
        lifetime: None,
        mutability: Some(token![mut]),
        elem: Box::new(type_named(new_identifier("H"))),
    })
}

pub fn hash(parameterised: &Parameterised) -> ImplItemFn {
    new_impl_fn(
        new_identifier("hash"),
        generics(),
        Receiver::Reference,
        [(new_identifier("state"), state_type())],
        unit_type(),
        chain(
            maybe_hash_discriminant(&parameterised.item),
            parameterised.item.map_variants(hash_variant),
        ),
    )
}

fn maybe_hash_discriminant(item: &AlgebraicItem) -> Option<Stmt> {
    if matches!(item, AlgebraicItem::Struct(_)) {
        return None;
    }

    let function = expression_path(new_path(["core", "mem", "discriminant"]));

    let discriminant = Expr::Call(ExprCall {
        attrs: Vec::new(),
        func: Box::new(function),
        paren_token: token![()],
        args: single(self_expression()),
    });

    // TODO: don't call method
    let expression = call_method(
        discriminant,
        new_identifier("hash"),
        single(variable_named(new_identifier("state"))),
    );

    Some(Stmt::Expr(expression, Some(token![;])))
}

fn hash_variant(variant: &Variant) -> Vec<Stmt> {
    #[expect(clippy::never_loop, reason = "Attribute is temporarily empty")]
    for attribute in &variant.attributes {
        match *attribute {}
    }

    variant
        .fields
        .clone()
        .into_named()
        .into_iter()
        .map(hash_field)
        .collect()
}

fn hash_field(field: NamedField) -> Stmt {
    // TODO: don't call method
    let expression = call_method(
        variable_named(field.name),
        new_identifier("hash"),
        single(variable_named(new_identifier("state"))),
    );

    Stmt::Expr(expression, Some(token![;]))
}

fn generics() -> Generics {
    Generics {
        lt_token: Some(token![<]),
        params: single(GenericParam::Type(TypeParam {
            attrs: Vec::new(),
            ident: new_identifier("H"),
            colon_token: Some(token![:]),
            bounds: single(TypeParamBound::Trait(TraitBound {
                paren_token: None,
                modifier: TraitBoundModifier::None,
                lifetimes: None,
                path: new_path(["core", "hash", "Hasher"]),
            })),
            eq_token: None,
            default: None,
        })),
        gt_token: Some(token![>]),
        where_clause: None,
    }
}
