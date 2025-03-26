use crate::field::Fields;
use crate::parameterised::Parameterised;
use crate::path;
use crate::punctuated::punctuated;
use crate::util::{
    Receiver, call_method, mutable_reference, new_identifier, new_impl_fn, token, variable_named,
};
use crate::variant::Variant;
use quote::{ToTokens, quote};
use std::iter::{Once, once};
use syn::punctuated::Punctuated;
use syn::{Expr, ExprMacro, Generics, ImplItemFn, Macro, MacroDelimiter, Stmt, Type, TypePath};

fn core_fmt_result() -> Type {
    Type::Path(TypePath {
        qself: None,
        path: path::core(["fmt", "Result"]),
    })
}

fn core_write_f<T: ToTokens>(arguments: T) -> Stmt {
    let mac = Macro {
        path: path::core(["write"]),
        bang_token: token![!],
        delimiter: MacroDelimiter::Paren(token![()]),
        tokens: quote!(f, #arguments),
    };

    Stmt::Expr(
        Expr::Macro(ExprMacro {
            attrs: Vec::new(),
            mac,
        }),
        None,
    )
}

fn core_stringify<T: ToTokens>(value: T) -> Expr {
    let mac = Macro {
        path: path::core(["stringify"]),
        bang_token: token![!],
        delimiter: MacroDelimiter::Paren(token![()]),
        tokens: value.to_token_stream(),
    };

    Expr::Macro(ExprMacro {
        attrs: Vec::new(),
        mac,
    })
}

fn formatter_type() -> Type {
    mutable_reference(Type::Path(TypePath {
        qself: None,
        path: path::core(["fmt", "Formatter"]),
    }))
}

pub fn fmt(parameterised: &Parameterised) -> ImplItemFn {
    #[expect(clippy::never_loop, reason = "Attribute is temporarily empty")]
    for attribute in parameterised.item.attributes() {
        match *attribute {}
    }

    // TODO: read from attributes
    let non_exhaustive = false;

    let statements = parameterised
        .item
        .map_variants(|variant| debug_variant(variant, non_exhaustive));

    new_impl_fn(
        new_identifier("fmt"),
        Generics::default(),
        Receiver::Reference,
        [(new_identifier("f"), formatter_type())],
        core_fmt_result(),
        statements,
    )
}

fn debug_variant(variant: &Variant, non_exhaustive: bool) -> Once<Stmt> {
    let name_string = core_stringify(&variant.name);

    #[expect(clippy::never_loop, reason = "Attribute is temporarily empty")]
    for attribute in &variant.attributes {
        match *attribute {}
    }

    let mut expression = variable_named(new_identifier("f"));

    let debugger = match variant.fields {
        Fields::Named(_) => new_identifier("debug_struct"),
        Fields::Unnamed(_) => new_identifier("debug_tuple"),
        Fields::Unit => return once(core_write_f(name_string)),
    };

    expression = call_method(expression, debugger, punctuated![name_string]);

    for field in variant.fields.clone().into_named() {
        #[expect(clippy::never_loop, reason = "Attribute is temporarily empty")]
        for attribute in &field.attributes {
            match *attribute {}
        }

        let mut args = if matches!(variant.fields, Fields::Named(_)) {
            punctuated![core_stringify(&field.name)]
        } else {
            punctuated![]
        };

        args.push(variable_named(field.name));

        expression = call_method(expression, new_identifier("field"), args);
    }

    let finish = if non_exhaustive {
        new_identifier("finish_non_exhaustive")
    } else {
        new_identifier("finish")
    };

    expression = call_method(expression, finish, Punctuated::new());

    once(Stmt::Expr(expression, None))
}
