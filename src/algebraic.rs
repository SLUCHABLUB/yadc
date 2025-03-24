use crate::util::{destruct, to_argument};
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Error, Fields, GenericArgument, Generics, Item, ItemEnum, ItemStruct, Result, Type, WhereClause};

const BAD_ITEM_KIND: &str = "yadc can only implement traits for enums and structs";
const NON_EXHAUSTIVE: &str = "item kind is not recognised, please open an issue";

pub enum AlgebraicItem {
    Enum(ItemEnum),
    Struct(ItemStruct),
}

impl AlgebraicItem {
    pub fn name(&self) -> &Ident {
        match self {
            AlgebraicItem::Enum(item) => &item.ident,
            AlgebraicItem::Struct(item) => &item.ident,
        }
    }

    fn generics(&self) -> &Generics {
        match self {
            AlgebraicItem::Enum(item) => &item.generics,
            AlgebraicItem::Struct(item) => &item.generics,
        }
    }

    pub fn parameters(&self) -> TokenStream {
        self.generics().params.to_token_stream()
    }

    pub fn arguments(&self) -> Punctuated<GenericArgument, Comma> {
        self.generics()
            .params
            .iter()
            .map(to_argument)
            .collect()
    }
    
    pub fn type_arguments(&self) -> Vec<Ident> {
        self.arguments().into_iter().filter_map(|argument| match argument {
            GenericArgument::Type(Type::Path(path)) => path.path.get_ident().cloned(),
            _ => None
        }).collect()
    }

    pub fn where_clause(&self) -> Option<&WhereClause> {
        self.generics().where_clause.as_ref()
    }

    pub fn map_variants<F>(&self, mut function: F) -> TokenStream
    where
        F: FnMut(&Ident, &Fields) -> TokenStream,
    {
        match self {
            AlgebraicItem::Enum(item) => {
                let enum_name = &item.ident;

                let arms = item
                    .variants
                    .iter()
                    .map(|variant| {
                        let pattern = destruct(&variant.ident, &variant.fields);
                        let expression = function(&variant.ident, &variant.fields);

                        quote!(#enum_name::#pattern => #expression)
                    })
                    .collect::<Punctuated<TokenStream, Comma>>();

                quote! {
                    match self {
                        #arms
                    }
                }
            }
            AlgebraicItem::Struct(item) => {
                let pattern = destruct(&item.ident, &item.fields);
                let expression = function(&item.ident, &item.fields);

                quote! {
                    let #pattern = self;
                    #expression
                }
            }
        }
    }
}

impl TryFrom<Item> for AlgebraicItem {
    type Error = Error;

    fn try_from(item: Item) -> Result<Self> {
        match item {
            Item::Const(_)
            | Item::ExternCrate(_)
            | Item::Fn(_)
            | Item::ForeignMod(_)
            | Item::Impl(_)
            | Item::Macro(_)
            | Item::Mod(_)
            | Item::Static(_)
            | Item::Trait(_)
            | Item::TraitAlias(_)
            | Item::Type(_)
            | Item::Union(_)
            | Item::Use(_) => Err(Error::new(item.span(), BAD_ITEM_KIND)),

            Item::Enum(item) => Ok(AlgebraicItem::Enum(item)),
            Item::Struct(item) => Ok(AlgebraicItem::Struct(item)),

            Item::Verbatim(_) | _ => Err(Error::new(item.span(), NON_EXHAUSTIVE)),
        }
    }
}

impl Parse for AlgebraicItem {
    fn parse(input: ParseStream) -> Result<Self> {
        Item::parse(input).and_then(AlgebraicItem::try_from)
    }
}

impl ToTokens for AlgebraicItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            AlgebraicItem::Enum(item) => item.to_tokens(tokens),
            AlgebraicItem::Struct(item) => item.to_tokens(tokens),
        }
    }
}
