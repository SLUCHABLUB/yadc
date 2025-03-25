use crate::field::Fields;
use crate::parameterised::Parameterised;
use crate::util::new_path;
use crate::variant::Variant;
use proc_macro2::TokenStream;
use quote::quote;

pub fn implement_debug(parameterised: &Parameterised) -> TokenStream {
    let trait_path = new_path(["core", "fmt", "Debug"]);

    #[expect(clippy::never_loop, reason = "Attribute is temporarily empty")]
    for attribute in parameterised.item.attributes() {
        match *attribute {}
    }

    let item = &parameterised.item;
    let name = item.name();
    let parameters = &parameterised.parameters;
    let arguments = parameterised.arguments();

    let default_bound = parameterised.bound_all(&trait_path);
    let where_clause = parameterised.where_clause_with_bounds(default_bound);

    // TODO: read from attributes
    let non_exhaustive = false;

    let body = item.map_variants(|variant| debug_variant(variant, non_exhaustive));

    quote! {
        #[automatically_derived]
        impl<#parameters> #trait_path for #name<#arguments> #where_clause {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                #body
            }
        }
    }
}

fn debug_variant(variant: &Variant, non_exhaustive: bool) -> TokenStream {
    let name = &variant.name;

    #[expect(clippy::never_loop, reason = "Attribute is temporarily empty")]
    for attribute in &variant.attributes {
        match *attribute {}
    }

    let finish = if non_exhaustive {
        quote!(finish_non_exhaustive)
    } else {
        quote!(finish)
    };

    let named;

    let mut output = match variant.fields {
        Fields::Named(_) => {
            named = true;
            quote!(f.debug_struct(stringify!(#name)))
        }
        Fields::Unnamed(_) => {
            named = false;
            quote!(f.debug_tuple(stringify!(#name)))
        }
        Fields::Unit => return quote! { core::write!(f, stringify!(#name)) },
    };

    for field in variant.fields.clone().into_named() {
        #[expect(clippy::never_loop, reason = "Attribute is temporarily empty")]
        for attribute in &field.attributes {
            match *attribute {}
        }

        let name = field.name;

        output.extend(if named {
            quote!(.field(stringify!(#name), #name))
        } else {
            quote!(.field(#name))
        });
    }

    output.extend(quote!(.#finish()));

    output
}
