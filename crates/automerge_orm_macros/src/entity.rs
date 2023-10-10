use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, DeriveInput, Error, Expr, Lit, Meta, NestedMeta};

pub fn derive(input: DeriveInput) -> syn::Result<TokenStream> {
    let entity = input.ident;
    let mut table_name = entity.to_string().to_snake_case();
    let mut id_expr: Expr = parse_quote!(self.id);
    for attr in input.attrs {
        if attr.path.is_ident("automerge_orm") {
            let meta = attr.parse_meta()?;
            let Meta::List(meta) = meta else {
                return Err(Error::new_spanned(meta, "expected #[automerge_orm(...)]"));
            };
            for meta_item in meta.nested {
                match &meta_item {
                    NestedMeta::Meta(Meta::NameValue(m)) if m.path.is_ident("id") => {
                        let Lit::Str(s) = &m.lit else {
                            return Err(Error::new_spanned(&m.lit, "expected string literal"));
                        };
                        id_expr = syn::parse_str(&s.value())?;
                    },
                    NestedMeta::Meta(Meta::NameValue(m)) if m.path.is_ident("table_name") => {
                        let Lit::Str(s) = &m.lit else {
                            return Err(Error::new_spanned(&m.lit, "expected string literal"));
                        };
                        table_name = s.value();
                    },
                    NestedMeta::Meta(meta_item) => {
                        let path = meta_item
                            .path()
                            .into_token_stream()
                            .to_string()
                            .replace(' ', "");
                        return Err(Error::new_spanned(
                            meta_item.path(),
                            format!("unknown automerge_orm entity attribute `{path}`"),
                        ));
                    },
                    NestedMeta::Lit(lit) => {
                        return Err(Error::new_spanned(
                            lit,
                            "unexpected literal in automerge_orm entity attribute",
                        ));
                    },
                }
            }
        }
    }

    Ok(quote! {
        #[automatically_derived]
        impl ::automerge_orm::Entity for #entity {}

        #[automatically_derived]
        impl ::automerge_orm::Mapped for #entity {
            fn table_name() -> ::automerge_orm::__macro_support::String {
                ::automerge_orm::__macro_support::ToOwned::to_owned(#table_name)
            }
        }

        #[automatically_derived]
        impl ::automerge_orm::Keyed for #entity {
            type Entity = #entity;

            fn id(&self) -> ::automerge_orm::Key<Self::Entity> {
                ::automerge_orm::__macro_support::Into::into(#id_expr)
            }
        }
    })
}
