macro_rules! define_config {
    // empty trait-level config
    () => {
        #[derive(Clone, Default)]
        pub struct Config;

        #[derive(Default)]
        pub struct Builder;

        impl From<Builder> for Config {
            fn from(_: Builder) -> Self {
                Config
            }
        }

        impl std::ops::BitOr for Builder {
            type Output = syn::Result<Builder>;

            fn bitor(self, _: Builder) -> syn::Result<Builder> {
                Ok(Builder)
            }
        }

        impl TryFrom<(syn::Ident, proc_macro2::TokenStream)> for Builder {
            type Error = syn::Error;

            fn try_from((option, _): (syn::Ident, proc_macro2::TokenStream)) -> syn::Result<Builder> {
                Err(syn::Error::new(option.span(), "unrecognized config option"))
            }
        }
    };
    // trait-level config
    ($($field:ident : $ty:ty = $default:expr,)*) => {
        #[derive(Clone)]
        pub struct Config {
            $(
                pub $field: $ty,
            )*
        }

        impl Default for Config {
            fn default() -> Self {
                Config::from(Builder::default())
            }
        }

        #[derive(Default)]
        pub(super) struct Builder {
            $(
                pub $field: Option<$ty>,
            )*
        }

        impl From<Builder> for Config {
            fn from(builder: Builder) -> Self {
                Config {
                    $(
                        $field: builder.$field.unwrap_or_else(|| $default),
                    )*
                }
            }
        }

        impl std::ops::BitOr for Builder {
            type Output = syn::Result<Builder>;

            fn bitor(self, rhs: Builder) -> syn::Result<Builder> {
                Ok(Builder {
                    $(
                        $field: match (self.$field, rhs.$field) {
                            (Some(lhs), Some(rhs)) => return Err(
                                crate::error::key_set_twice(stringify!($field), &lhs, &rhs)
                            ),
                            (Some($field), None) | (None, Some($field)) => Some($field),
                            (None, None) => None,
                        },
                    )*
                })
            }
        }

        impl TryFrom<(syn::Ident, proc_macro2::TokenStream)> for Builder {
            type Error = syn::Error;

            fn try_from((option, tokens): (syn::Ident, proc_macro2::TokenStream)) -> syn::Result<Builder> {
                let mut builder = Builder::default();
                $(
                    if option == stringify!($field) {
                        builder.$field = Some(syn::parse2::<$ty>(tokens)?);
                        return Ok(builder);
                    }
                )*

                Err(syn::Error::new(option.span(), "unrecognized config option"))
            }
        }
    };

    // top-level config
    ($($field:ident,)*) => {
        use quote::ToTokens as _;
        use syn::spanned::Spanned as _;

        $(
            pub mod $field;
        )*

        #[derive(Clone, Default)]
        pub struct Config {
            $(
                pub $field: $field::Config,
            )*
        }

        #[derive(Default)]
        struct Builder {
            $(
                pub $field: $field::Builder,
            )*
        }

        impl From<Builder> for Config {
            fn from(builder: Builder) -> Self {
                Config {
                    $(
                        $field: $field::Config::from(builder.$field),
                    )*
                }
            }
        }

        impl std::ops::BitOr for Builder {
            type Output = syn::Result<Builder>;

            fn bitor(self, rhs: Builder) -> syn::Result<Builder> {
                Ok(Builder {
                    $(
                        $field: (self.$field | rhs.$field)?,
                    )*
                })
            }
        }

        impl TryFrom<(syn::Ident, syn::punctuated::Punctuated<syn::Meta, syn::Token![,]>)> for Builder {
            type Error = syn::Error;

            fn try_from((trait_name, metas): (syn::Ident, syn::punctuated::Punctuated<syn::Meta, syn::Token![,]>)) -> syn::Result<Builder> {
                let pairs: Vec<_> = metas
                    .into_iter()
                    .map(|meta| match meta {
                        syn::Meta::Path(path) => (path, crate::expression::true_().into_token_stream()),
                        syn::Meta::List(meta) => (meta.path, meta.tokens),
                        syn::Meta::NameValue(meta) => (meta.path, meta.value.into_token_stream())
                    })
                    .map(|(path, tokens)| {
                        let span = path.span();
                        let error = || crate::error::path_too_long(span);
                        let identifier = crate::path::into_identifier(path).ok_or_else(error)?;
                        Ok((identifier, tokens))
                    })
                    .collect::<syn::Result<_>>()?;

                $(
                    if trait_name == stringify!($field) {
                        let mut builder = Builder::default();
                        builder.$field = pairs
                            .into_iter()
                            .try_fold($field::Builder::default(), |builder, pair| {
                                builder | $field::Builder::try_from(pair)?
                            })?;
                        return Ok(builder);
                    }
                )*

                return Ok(Builder::default())
            }
        }

        impl TryFrom<syn::MetaList> for Builder {
            type Error = syn::Error;

            fn try_from(meta: syn::MetaList) -> syn::Result<Builder> {
                if meta.path.leading_colon.is_some() {
                    return Ok(Builder::default())
                }

                if let Some(identifiier) = crate::path::into_identifier(meta.path.clone()) {
                    let metas = crate::metas::parse(meta.tokens)?;

                    return Builder::try_from((identifiier, metas))
                }

                let Some((trait_name, path)) = crate::path::split_off_first(meta.path) else {
                    return Ok(Builder::default())
                };

                let metas = crate::punctuated![syn::Meta::List(syn::MetaList {
                    path,
                    delimiter: meta.delimiter,
                    tokens: meta.tokens
                })];

                Builder::try_from((trait_name, metas))
            }
        }

        impl TryFrom<syn::Meta> for Builder {
            type Error = syn::Error;

            fn try_from(meta: syn::Meta) -> syn::Result<Builder> {
                let (mut path, expression) = match meta {
                    syn::Meta::Path(path) => (path, crate::expression::true_()),
                    syn::Meta::List(meta) => return Builder::try_from(meta),
                    syn::Meta::NameValue(meta) => (meta.path, meta.value)
                };

                if path.segments.len() == 1 {
                    path.segments.push(syn::PathSegment::from(crate::identifier!(value)));
                }

                Builder::try_from(syn::MetaList {
                    path,
                    delimiter: syn::MacroDelimiter::Paren(crate::token![()]),
                    tokens: expression.into_token_stream()
                })
            }
        }

        impl TryFrom<Vec<syn::Attribute>> for Config {
            type Error = syn::Error;

            fn try_from(attributes: Vec<syn::Attribute>) -> syn::Result<Config> {
                attributes
                    .into_iter()
                    .try_fold(
                        Builder::default(),
                        |accumulator, attribute| {
                            accumulator | Builder::try_from(attribute.meta)?
                        }
                    )
                    .map(Config::from)
            }
        }
    }
}

pub(super) use define_config;
