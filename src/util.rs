use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Fields, GenericParam};

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

/// Extract the name from a generic parameter (converts it to an argument).
///
/// | parameter kind | input example | output |
/// | :------------- | :------------ | :----- |
/// | lifetime       | `'a: 'b`      | `'a`   |
/// | type           | `T: Trait`    | `T`    |
/// | constant       | `const N: u8` | `N`    |
pub fn to_argument(parameter: &GenericParam) -> TokenStream {
    match parameter {
        GenericParam::Lifetime(parameter) => {
            let lifetime = &parameter.lifetime;
            quote!(#lifetime)
        }
        GenericParam::Type(ty) => {
            let name = &ty.ident;
            quote!(#name)
        }
        GenericParam::Const(constant) => {
            let name = &constant.ident;
            quote!(#name)
        }
    }
}
