use crate::{expression, pattern};
use proc_macro2::{Ident, Span, TokenStream};
use syn::punctuated::Punctuated;
use syn::{
    Block, Expr, ExprCall, ExprMethodCall, ExprPath, ExprReference, FnArg, GenericArgument,
    GenericParam, Generics, ImplItemFn, PatType, Path, PredicateType, ReturnType, Signature, Stmt,
    Token, TraitBound, TraitBoundModifier, Type, TypeParamBound, TypePath, TypeReference,
    TypeTuple, Visibility, WherePredicate,
};

macro_rules! token {
    [()] => { syn::token::Paren::default() };
    [[]] => { syn::token::Bracket::default() };
    [{}] => { syn::token::Brace::default() };
    [$token:tt] => { <syn::Token![$token]>::default() };
}

// TODO: remove
use crate::punctuated::punctuated;
pub(crate) use token;

pub fn new_identifier(string: &str) -> Ident {
    Ident::new(string, Span::call_site())
}

#[derive(Copy, Clone)]
pub enum Receiver {
    Reference,
}

/// Constructs an [`ImplItemFn`]
pub fn new_impl_fn<const PARAMETERS: usize, Statements>(
    name: Ident,
    generics: Generics,
    receiver: Receiver,
    parameters: [(Ident, Type); PARAMETERS],
    return_type: Type,
    statements: Statements,
) -> ImplItemFn
where
    Statements: IntoIterator<Item = Stmt>,
{
    let mut inputs = Punctuated::new();

    match receiver {
        Receiver::Reference => {
            inputs.push(FnArg::Receiver(syn::Receiver {
                attrs: Vec::new(),
                reference: Some((token![&], None)),
                mutability: None,
                self_token: token![self],
                colon_token: None,
                ty: Box::new(Type::Verbatim(TokenStream::new())),
            }));
        }
    }

    for (parameter, ty) in parameters {
        inputs.push(FnArg::Typed(PatType {
            attrs: Vec::new(),
            pat: Box::new(pattern::variable(parameter)),
            colon_token: token![:],
            ty: Box::new(ty),
        }));
    }

    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: token![fn],
        ident: name,
        generics,
        paren_token: token![()],
        inputs,
        variadic: None,
        output: ReturnType::Type(token![->], Box::new(return_type)),
    };

    ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig,
        block: Block {
            brace_token: token![{}],
            stmts: statements.into_iter().collect(),
        },
    }
}

pub fn bound_type(ty: Type, trait_bound: Path) -> WherePredicate {
    WherePredicate::Type(PredicateType {
        lifetimes: None,
        bounded_ty: ty,
        colon_token: token![:],
        bounds: punctuated![TypeParamBound::Trait(TraitBound {
            paren_token: None,
            modifier: TraitBoundModifier::None,
            lifetimes: None,
            path: trait_bound,
        })],
    })
}

pub fn type_named(name: Ident) -> Type {
    Type::Path(TypePath {
        qself: None,
        path: Path::from(name),
    })
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

pub fn unit_type() -> Type {
    Type::Tuple(TypeTuple {
        paren_token: token![()],
        elems: Punctuated::new(),
    })
}

pub fn call_method(receiver: Expr, method: Ident, args: Punctuated<Expr, Token![,]>) -> Expr {
    Expr::MethodCall(ExprMethodCall {
        attrs: Vec::new(),
        receiver: Box::new(receiver),
        dot_token: token![.],
        method,
        turbofish: None,
        paren_token: token![()],
        args,
    })
}

pub fn mutable_reference(referend: Type) -> Type {
    Type::Reference(TypeReference {
        and_token: token![&],
        lifetime: None,
        mutability: Some(token![mut]),
        elem: Box::new(referend),
    })
}

pub fn call_function(function: Path, arguments: Punctuated<Expr, Token![,]>) -> Expr {
    Expr::Call(ExprCall {
        attrs: Vec::new(),
        func: Box::new(expression::path(function)),
        paren_token: token![()],
        args: arguments,
    })
}

pub fn reference(referend: Expr) -> Expr {
    Expr::Reference(ExprReference {
        attrs: Vec::new(),
        and_token: token![&],
        mutability: None,
        expr: Box::new(referend),
    })
}
