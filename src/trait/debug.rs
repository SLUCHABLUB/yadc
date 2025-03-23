use crate::algebraic::AlgebraicItem;
use crate::util::field_names;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Fields, Result};

pub fn implement_debug(item: &AlgebraicItem) -> Result<TokenStream> {
    let name = item.name();
    let parameters = item.parameters();
    let arguments = item.arguments();
    let where_clause = item.where_clause();

    // TODO: read from attributes
    let non_exhaustive = false;

    let body = item.map_variants(|name, field| debug_variant(name, field, non_exhaustive));

    Ok(quote! {
        impl<#parameters> core::fmt::Debug for #name<#arguments> #where_clause {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                #body
            }
        }
    })
}

fn debug_variant(name: &Ident, fields: &Fields, non_exhaustive: bool) -> TokenStream {
    let finish = if non_exhaustive {
        quote!(finish_non_exhaustive)
    } else {
        quote!(finish)
    };
    let field_name = field_names(fields);

    match fields {
        Fields::Named(_) => quote! {
            f.debug_struct(stringify!(#name))
            #(
                .field(stringify!(#field_name), #field_name)
            )*
            .#finish()
        },
        Fields::Unnamed(_) => quote! {
            f.debug_tuple(stringify!(#name))
            #(
                .field(#field_name)
            )*
            .#finish()
        },
        Fields::Unit => quote! { core::write!(f, stringify!(#name)) },
    }
}
