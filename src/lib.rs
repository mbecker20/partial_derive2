use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{DeriveInput, Fields, Type, Visibility};

#[proc_macro_derive(Partial, attributes(partial_derive, partial_attr))]
pub fn derive_partial(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs,
        vis,
        ident,
        data,
        ..
    } = syn::parse(input).unwrap();

    let partial_attrs = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("partial_attr"))
        .map(|attr| {
            attr.parse_args::<proc_macro2::TokenStream>()
                .expect("failed to parse partial_attr")
        });

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

    let fields = filter_fields(match data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => panic!("Field can only be derived for structs"),
    });

    let _field_var = fields.iter().map(|(vis, ident, ty)| {
        quote! {
            #vis #ident: make_option::make_option!(#ty)
        }
    });

    quote! {
        #(#partial_attrs)*
        #[derive(#derives)]
        #vis struct #partial_ident {
            #(#_field_var),*
        }
    }
    .into()
}

fn filter_fields(fields: &Fields) -> Vec<(Visibility, Ident, Type)> {
    fields
        .iter()
        .filter_map(|field| {
            if field.ident.is_some() {
                let field_vis = field.vis.clone();
                let field_ident = field.ident.as_ref().unwrap().clone();
                let field_ty = field.ty.clone();
                Some((field_vis, field_ident, field_ty))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}
