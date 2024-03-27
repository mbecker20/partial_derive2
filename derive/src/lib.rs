use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Data, DeriveInput, Expr, Field, Type, Visibility};

#[proc_macro_derive(
    Partial,
    attributes(
        partial_derive,
        partial_from,
        skip_serializing_none,
        partial_attr,
        partial_default,
    )
)]
pub fn derive_partial(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs,
        vis,
        ident,
        data,
        ..
    } = syn::parse(input).unwrap();

    let skip_serializing = attrs
        .iter()
        .any(|attr| attr.path().is_ident("skip_serializing_none"));

    let derives = attrs
        .iter()
        .find(|attr| attr.path().is_ident("partial_derive"));

    let derives = if let Some(derives) = derives {
        derives
            .parse_args()
            .expect("failed to parse partial_derive")
    } else {
        proc_macro2::TokenStream::new()
    };

    let partial_ident = Ident::new(&format!("Partial{}", ident), Span::call_site());

    let fields = get_fields(data);

    let partial_fields: Vec<_> = if skip_serializing {
        fields
            .iter()
            .map(|(vis, ident, ty, _, partial_attr)| {
                quote! {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    #(#[#partial_attr])*
                    #vis #ident: partial_derive2::make_option!(#ty)
                }
            })
            .collect()
    } else {
        fields
            .iter()
            .map(|(vis, ident, ty, _, partial_attr)| {
                quote! {
                    #(#[#partial_attr])*
                    #vis #ident: partial_derive2::make_option!(#ty)
                }
            })
            .collect()
    };

    let merge_fields = fields.iter().map(|(_, ident, ty, _, _)| {
        quote!(#ident: partial_derive2::value_maybe_as_option!(#ty, partial.#ident.unwrap_or(self.#ident), partial.#ident))
    });

    let partial_from_fields = fields.iter().map(
        |(_, ident, ty, _, _)| quote!(#ident: partial_derive2::value_as_option!(#ty, value.#ident)),
    );

    let derive_from_partial = attrs
        .iter()
        .find(|a| a.path().is_ident("partial_from"))
        .map(|_| {
            let partial_to_fields = fields
                .iter()
                .map(|(_, ident, ty, def, _)| quote!{
                    #ident: partial_derive2::value_maybe_as_option!(#ty, value.#ident.unwrap_or(#def), value.#ident)
                });
            quote! {
                impl From<#partial_ident> for #ident {
                    fn from(value: #partial_ident) -> #ident {
                        #ident {
                            #(#partial_to_fields),*
                        }
                    }
                }
            }
        });

    quote! {
        #[derive(#derives)]
        #vis struct #partial_ident {
            #(#partial_fields),*
        }

        impl partial_derive2::HasPartial for #ident {
            type Partial = #partial_ident;
            fn merge_partial(self, partial: Self::Partial) -> #ident {
                #ident {
                    #(#merge_fields),*
                }
            }
        }

        impl From<#ident> for #partial_ident {
            fn from(value: #ident) -> #partial_ident {
                #partial_ident {
                    #(#partial_from_fields),*
                }
            }
        }

        #derive_from_partial
    }
    .into()
}

fn get_fields(
    data: Data,
) -> Vec<(
    Visibility,
    Ident,
    Type,
    proc_macro2::TokenStream,
    Vec<proc_macro2::TokenStream>,
)> {
    let fields = if let Data::Struct(s) = data {
        s.fields
    } else {
        panic!("Partial can only be derived for structs")
    };
    fields
        .into_iter()
        .filter_map(
            |Field {
                 vis,
                 attrs,
                 ident,
                 ty,
                 ..
             }| {
                let partial_default = attrs
                    .iter()
                    .find(|a| a.path().is_ident("partial_default"))
                    .map(|partial_default| {
                        let partial_default: Expr = partial_default
                            .parse_args()
                            .expect("failed to parse partial_default argument");
                        quote!(#partial_default)
                    })
                    .unwrap_or_else(|| quote!(Default::default()));
                let partial_attr = attrs
                    .iter()
                    .filter(|a| a.path().is_ident("partial_attr"))
                    .map(|partial_attr| {
                        partial_attr
                            .parse_args()
                            .expect("failed to parse partial_attr argument")
                    })
                    .collect();
                ident.map(|ident| (vis, ident, ty, partial_default, partial_attr))
            },
        )
        .collect::<Vec<_>>()
}
