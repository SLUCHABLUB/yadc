use proc_macro2::{Ident, Span};
use std::iter::once;
use syn::punctuated::Punctuated;
use syn::{Expr, ExprPath, GenericArgument, GenericParam, Path, PathSegment, Type, TypePath};

pub fn type_named(name: Ident) -> Type {
    Type::Path(TypePath {
        qself: None,
        path: Path::from(name),
    })
}

pub fn single<T, P: Default>(item: T) -> Punctuated<T, P> {
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
pub fn to_argument(parameter: GenericParam) -> GenericArgument {
    match parameter {
        GenericParam::Lifetime(parameter) => GenericArgument::Lifetime(parameter.lifetime),
        GenericParam::Type(ty) => GenericArgument::Type(type_named(ty.ident)),
        GenericParam::Const(constant) => GenericArgument::Const(Expr::Path(ExprPath {
            attrs: Vec::new(),
            qself: None,
            path: Path::from(constant.ident),
        })),
    }
}
