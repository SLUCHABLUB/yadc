use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use std::iter::once;
use syn::punctuated::Punctuated;
use syn::token::{Colon, Comma, Where};
use syn::{
    Expr, ExprPath, Fields, GenericArgument, GenericParam, Path, PathSegment, PredicateType,
    TraitBound, TraitBoundModifier, Type, TypeParamBound, TypePath, WhereClause, WherePredicate,
};

/// Creates a pattern for destructing a struct / variant.
///
/// For example, `struct Foo { bar: u8, baz: u8 }` becomes `Foo { bar, baz }`
pub fn destruct(name: &Ident, fields: &Fields) -> TokenStream {
    let field_name = field_names(fields);

    match fields {
        Fields::Named(_) => quote! {
            #name {
                #(#field_name),*
            }
        },
        Fields::Unnamed(_) => quote! {
            #name(
                #(#field_name),*
            )
        },
        Fields::Unit => quote!(#name),
    }
}

pub fn field_names(fields: &Fields) -> Vec<Ident> {
    fields
        .iter()
        .enumerate()
        .map(|(index, field)| {
            field
                .ident
                .clone()
                .unwrap_or(format_ident!("value_{index}"))
        })
        .collect()
}

pub fn with_bound(
    where_clause: Option<WhereClause>,
    types: &[Ident],
    trait_path: &Path,
) -> Option<WhereClause> {
    if types.is_empty() {
        return where_clause;
    }

    let mut predicates = where_clause
        .map(|clause| clause.predicates)
        .unwrap_or_default();

    predicates.extend(bound(types, trait_path));

    Some(WhereClause {
        where_token: Where::default(),
        predicates,
    })
}

fn bound(types: &[Ident], trait_path: &Path) -> Punctuated<WherePredicate, Comma> {
    let bound = TypeParamBound::Trait(TraitBound {
        paren_token: None,
        modifier: TraitBoundModifier::None,
        lifetimes: None,
        path: trait_path.clone(),
    });

    types
        .iter()
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

fn type_named(name: &Ident) -> Type {
    Type::Path(TypePath {
        qself: None,
        path: Path::from(name.clone()),
    })
}

fn single<T, P: Default>(item: T) -> Punctuated<T, P> {
    once(item).collect()
}

pub fn new_path<const N: usize>(segments: [&str; N]) -> Path {
    Path {
        leading_colon: None,
        segments: segments
            .into_iter()
            .map(|name| PathSegment::from(Ident::new(name, Span::call_site())))
            .collect(),
    }
}

/// Extract the name from a generic parameter (converts it to an argument).
///
/// | parameter kind | input example | output |
/// | :------------- | :------------ | :----- |
/// | lifetime       | `'a: 'b`      | `'a`   |
/// | type           | `T: Trait`    | `T`    |
/// | constant       | `const N: u8` | `N`    |
pub fn to_argument(parameter: &GenericParam) -> GenericArgument {
    match parameter {
        GenericParam::Lifetime(parameter) => GenericArgument::Lifetime(parameter.lifetime.clone()),
        GenericParam::Type(ty) => GenericArgument::Type(type_named(&ty.ident)),
        GenericParam::Const(constant) => GenericArgument::Const(Expr::Path(ExprPath {
            attrs: Vec::new(),
            qself: None,
            path: Path::from(constant.ident.clone()),
        })),
    }
}
