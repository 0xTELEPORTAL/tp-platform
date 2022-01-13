use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::{quote, quote_spanned};
use syn::parse2;
use syn::parse_macro_input;
use syn::spanned::Spanned;
use syn::Error;
use syn::Result;

// channel and state proc macros are really similar, so to avoid boilerplate and
// keep it DRY we provide a template macro.

macro_rules! template {
    ($macro_name:ident, $handle_type:ty) => {
        #[proc_macro_attribute]
        pub fn $macro_name(
            _attr: proc_macro::TokenStream,
            item: proc_macro::TokenStream,
        ) -> proc_macro::TokenStream {
            // Get the AST of anything that can be derived (in this case, the struct)
            let item = parse_macro_input!(item as syn::DeriveInput);

            imp::$macro_name(item)
                .unwrap_or_else(|e| e.into_compile_error())
                .into()
        }
    };
}
macro_rules! template_impl {
    ($macro_name:ident, $handle_type:ty) => {
        pub fn $macro_name(mut item: syn::DeriveInput) -> Result<TokenStream> {
            // ---- Field Names ----
            let type_ids_ident = quote::format_ident!("type_ids");

            // Parse struct
            let s: &mut syn::DataStruct = match item.data {
                syn::Data::Struct(ref mut s) => s,
                syn::Data::Enum(ref mut e) => {
                    return Err(Error::new(
                        e.enum_token.span(),
                        "Only structs are supported",
                    ))
                }
                syn::Data::Union(ref mut u) => {
                    return Err(Error::new(
                        u.union_token.span(),
                        "Only structs are supported",
                    ))
                }
            };

            // Holds the contents of the generated impl block
            let mut impl_ts = TokenStream::new();
            // Get the named fields
            let fields = match s.fields {
                syn::Fields::Named(ref mut fields) => &mut fields.named,
                syn::Fields::Unit | syn::Fields::Unnamed(_) => {
                    return Err(Error::new(
                        s.fields.span(),
                        "Only named structs are supported",
                    ));
                }
            };

            // Holds the contents of the field initializer
            let mut field_init = TokenStream::new();
            // Holds the typeids of each field as the contents of vec![...]
            let mut typeids = Vec::new();
            let s_name = &item.ident;
            for (i, f) in fields.iter_mut().enumerate() {
                let inner_t = f.ty.clone();
                // won't panic because we already checked that the fields were named
                let f_name = f.ident.as_ref().unwrap();
                if f_name == &type_ids_ident {
                    return Err(Error::new(
                        f_name.span(),
                        "this field identifier is reserved",
                    ));
                }
                // Wrap field with handle type
                f.ty = parse2(quote_spanned! {inner_t.span()=>
                    $handle_type<#inner_t>
                })
                .unwrap();
                let wrapped_ty = &f.ty;

                // Add getter method
                impl_ts.extend(quote_spanned! {inner_t.span()=>
                    impl #s_name {
                        pub fn #f_name(&self) -> #wrapped_ty {
                            self.#f_name
                        }
                    }
                });

                // Add field initialization to the list
                field_init.extend(quote_spanned! {inner_t.span()=>
                    #f_name: $handle_type::new(#i, id),
                });

                // Add typeid of inner type
                typeids.push({
                    quote_spanned! {inner_t.span()=>
                        ::std::any::TypeId::of::<#inner_t>()
                    }
                });
            }

            // Field-agnostic impl block
            impl_ts.extend(quote! {
                impl #s_name {
                    pub fn new(id: tp_client::contract::ContractDataHandle) -> Self {
                        Self {
                            #field_init
                        }
                    }

                    pub fn #type_ids_ident() -> &'static [::std::any::TypeId] {
                        ::lazy_static::lazy_static! {
                            static ref TYPE_IDS: Vec<::std::any::TypeId> = vec![#(#typeids),*];
                        }
                        TYPE_IDS.as_slice()
                    }
                }
            });

            // Concatenate and return item tokens and impl tokens
            let mut result = item.into_token_stream();
            result.extend(impl_ts);
            Ok(result)
        }
    };
}

template!(states, tp_client::contract::properties::StateId);
template!(channels, tp_client::contract::properties::ChannelId);
pub(crate) mod imp {
    use super::*;
    template_impl!(states, tp_client::contract::properties::StateId);
    template_impl!(channels, tp_client::contract::properties::ChannelId);
}
