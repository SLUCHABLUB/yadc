use crate::expression::{call_method, if_else, variable};
use crate::item::Algebraic;
use crate::statement::{implicit_return, let_mut};
use crate::traits::Trait;
use crate::util::{Receiver, bound_type, mutable_reference, new_impl_fn};
use crate::{
    Fields, NamedField, Parameterised, Variant, core_path, field, identifier, item, punctuated,
    statement, token, variant,
};
use proc_macro2::Ident;
use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::{
    Expr, ExprMacro, Generics, ImplItemFn, Macro, MacroDelimiter, Stmt, Token, Type, TypePath,
    WherePredicate,
};

/// `core::fmt::Result`
fn core_fmt_result() -> Type {
    Type::Path(TypePath {
        qself: None,
        path: core_path!(fmt::Result),
    })
}

/// `core::write!(f, ...)`
fn core_write_f<T: ToTokens>(arguments: T) -> Stmt {
    let mac = Macro {
        path: core_path!(write),
        bang_token: token![!],
        delimiter: MacroDelimiter::Paren(token![()]),
        tokens: quote!(f, #arguments),
    };

    implicit_return(Expr::Macro(ExprMacro {
        attrs: Vec::new(),
        mac,
    }))
}

/// `core::stringify!(...)`
fn core_stringify<T: ToTokens>(value: T) -> Expr {
    let mac = Macro {
        path: core_path!(stringify),
        bang_token: token![!],
        delimiter: MacroDelimiter::Paren(token![()]),
        tokens: value.into_token_stream(),
    };

    Expr::Macro(ExprMacro {
        attrs: Vec::new(),
        mac,
    })
}

fn formatter_identifier() -> Ident {
    identifier!(f)
}

fn formatter_type() -> Type {
    mutable_reference(Type::Path(TypePath {
        qself: None,
        path: core_path!(fmt::Formatter),
    }))
}

fn formatter_expression() -> Expr {
    variable(formatter_identifier())
}

pub fn fmt(parameterised: &Parameterised) -> ImplItemFn {
    let item::debug::Config {} = parameterised.item.config().debug;

    let statements = parameterised.item.map_variants(debug_variant);

    new_impl_fn(
        identifier!(fmt),
        Generics::default(),
        Receiver::Reference,
        [(formatter_identifier(), formatter_type())],
        core_fmt_result(),
        statements,
    )
}

fn debug_variant(variant: &Variant) -> Vec<Stmt> {
    let name_string = core_stringify(&variant.name);

    let variant::debug::Config { non_exhaustive } = &variant.config.debug;

    let debugger = match variant.fields {
        Fields::Named(_) => identifier!(debug_struct),
        Fields::Unnamed(_) => identifier!(debug_tuple),
        Fields::Unit => return vec![core_write_f(name_string)],
    };
    let is_named = matches!(variant.fields, Fields::Named(_));

    let mut statements = Vec::new();

    let builder = call_method(formatter_expression(), debugger, punctuated![name_string]);

    // create the debug builder
    statements.push(let_mut(formatter_identifier(), builder));

    for field in variant.fields.clone().into_named() {
        statements.extend(debug_field(field, is_named));
    }

    // finish the builder
    statements.push(implicit_return(if_else(
        non_exhaustive.clone(),
        call_method(
            formatter_expression(),
            identifier!(finish_non_exhaustive),
            punctuated![],
        ),
        call_method(formatter_expression(), identifier!(finish), punctuated![]),
    )));

    statements
}

fn debug_field(field: NamedField, is_named: bool) -> Option<Stmt> {
    let field::debug::Config { skip } = field.config.debug;

    let mut args = if is_named {
        punctuated![core_stringify(&field.name)]
    } else {
        punctuated![]
    };

    args.push(variable(field.name));

    (!skip.value).then(|| {
        statement::new(call_method(
            formatter_expression(),
            identifier!(field),
            args,
        ))
    })
}

pub fn bounds(item: &Algebraic) -> Punctuated<WherePredicate, Token![,]> {
    let item::debug::Config {} = item.config().debug;

    let mut bounds = Punctuated::new();

    for variant in item.variants() {
        let variant::debug::Config { non_exhaustive: _ } = variant.config.debug;

        for field in variant.fields.clone().into_named() {
            let field::debug::Config { skip } = field.config.debug;

            if !skip.value {
                bounds.push(bound_type(field.ty, Trait::Debug.path()));
            }
        }
    }

    bounds
}
