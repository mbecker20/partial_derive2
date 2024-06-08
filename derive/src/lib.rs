use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Data, DeriveInput, Expr, Field, Type, Visibility};

#[proc_macro_derive(
  Partial,
  attributes(partial, partial_derive, partial_attr, partial_default,)
)]
pub fn derive_partial(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let DeriveInput {
    attrs,
    vis,
    ident,
    data,
    ..
  } = syn::parse(input).unwrap();

  let mut skip_serializing = false;
  let mut derive_partial_from = false;
  let mut derive_partial_diff = false;

  attrs.iter().for_each(|attr| {
    if !attr.path().is_ident("partial") {
      return;
    }
    attr
      .parse_nested_meta(|meta| {
        if meta.path.is_ident("skip_serializing_none") {
          skip_serializing = true;
        }
        if meta.path.is_ident("from") {
          derive_partial_from = true;
        }
        if meta.path.is_ident("diff") {
          derive_partial_diff = true;
        }
        Ok(())
      })
      .expect("failed to get partial params");
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

  let derive_from_partial = derive_partial_from
    .then(|| {
      let fields = fields
        .iter()
        .map(|(_, ident, ty, def, _)| quote!{
          #ident: partial_derive2::value_maybe_as_option!(#ty, value.#ident.unwrap_or(#def), value.#ident)
        });
      quote! {
        impl From<#partial_ident> for #ident {
          fn from(value: #partial_ident) -> #ident {
            #ident {
              #(#fields),*
            }
          }
        }
      }
    });

  let derive_partial_diff = derive_partial_diff.then(|| {
    let diff_ident = Ident::new(&format!("{}Diff", ident), Span::call_site());

    let diff_struct_fields = if skip_serializing {
      fields
        .iter()
        .map(|(vis, ident, ty, _, _)| {
          quote! {
            #[serde(skip_serializing_if = "Option::is_none")]
            #vis #ident: Option<(#ty, #ty)>
          }
        })
        .collect::<Vec<_>>()
    } else {
      fields
        .iter()
        .map(|(vis, ident, ty, _, _)| {
          quote! {
            #vis #ident: Option<(#ty, #ty)>
          }
        })
        .collect::<Vec<_>>()
    };

    let diff_from_fields = fields.iter().map(|(_, ident, ty, _, _)| {
      quote! {
        #ident: partial_derive2::value_maybe_as_option!(
          #ty,
          value.#ident.map(|item| item.1),
          value.#ident.map(|item| item.1).flatten()
        )
      }
    });

    let diff_trait_fields = fields.iter().map(|(_, ident, ty, _, _)| {
      quote! {
        #ident: {
          match (
            partial_derive2::value_as_option!(#ty, &self.#ident),
            partial.#ident,
          ) {
            (None, Some(value)) => Some((
              partial_derive2::value_maybe_as_option!(#ty, self.#ident.clone(), None),
              partial_derive2::value_maybe_as_option!(#ty, value, Some(value))
            )),
            (Some(field), Some(value)) if &value != field => Some((
              partial_derive2::value_maybe_as_option!(#ty, field.clone(), Some(field.clone())),
              partial_derive2::value_maybe_as_option!(#ty, value, Some(value))
            )),
            _ => None,
          }
        }
      }
    });

    let diff_iter_items = fields.iter().map(|(_, ident, ty, _, _)| {
      quote! {
        self.#ident.as_ref().map(|(prev, curr)| partial_derive2::value_maybe_as_vec!(
          #ty,
          partial_derive2::FieldDiff {
            field: stringify!(#ident),
            from: format!("{prev:?}"),
            to: format!("{curr:?}"),
          },
          {
            let mut from = String::from("[");
            let mut to = String::from("[");
            let max = std::cmp::max(prev.len(), curr.len());
            let mut unchanged = 0;
            for i in 0..max {
              match (prev.get(i), curr.get(i)) {
                (Some(prev), Some(curr)) => {
                  if prev == curr {
                    unchanged += 1;
                    if i == max - 1 {
                      from.push_str(&format!("...{unchanged} unchanged"));
                      to.push_str(&format!("...{unchanged} unchanged"));
                    }
                    continue;
                  } else {
                    if unchanged > 0 {
                      from.push_str(&format!("...{unchanged} unchanged, "));
                      to.push_str(&format!("...{unchanged} unchanged, "));
                      unchanged = 0;
                    }
                    from.push_str(&format!("{i}: {prev:?}"));
                    to.push_str(&format!("{i}: {curr:?}"));
                  }
                },
                (Some(prev), None) => {
                  if unchanged > 0 {
                    from.push_str(&format!("...{unchanged} unchanged, "));
                    to.push_str(&format!("...{unchanged} unchanged, "));
                    unchanged = 0;
                  }
                  from.push_str(&format!("{i}: {prev:?}"));
                  to.push_str(&format!("{i}: None"));
                },
                (None, Some(curr)) => {
                  if unchanged > 0 {
                    from.push_str(&format!("...{unchanged} unchanged, "));
                    to.push_str(&format!("...{unchanged} unchanged, "));
                    unchanged = 0;
                  }
                  from.push_str(&format!("{i}: None"));
                  to.push_str(&format!("{i}: {curr:?}"));
                },
                (None, None) => unreachable!()
              }
              if i < max - 1 {
                from.push_str(", ");
                to.push_str(", ");
              }
            }
            from.push_str("]");
            to.push_str("]");
            partial_derive2::FieldDiff {
              field: stringify!(#ident),
              from,
              to,
            }
          }
        ))
      }
    });

    let is_none_fields = fields.iter().map(|(_, ident, _, _, _)| {
      quote! {
        self.#ident.is_none()
      }
    });

    quote! {
      #[derive(#derives)]
      #vis struct #diff_ident {
        #(#diff_struct_fields),*
      }

      impl partial_derive2::MaybeNone for #diff_ident {
        fn is_none(&self) -> bool {
          #(#is_none_fields) &&*
        }
      }

      impl From<#diff_ident> for #partial_ident {
        fn from(value: #diff_ident) -> #partial_ident {
          #partial_ident {
            #(#diff_from_fields),*
          }
        }
      }

      impl partial_derive2::PartialDiff<#partial_ident, #diff_ident> for #ident {
        fn partial_diff(&self, partial: #partial_ident) -> #diff_ident {
          #diff_ident {
            #(#diff_trait_fields),*
          }
        }
      }

      impl partial_derive2::Diff for #diff_ident {
        fn iter_field_diffs(&self) -> impl Iterator<Item = partial_derive2::FieldDiff> {
          [
            #(#diff_iter_items),*
          ]
            .into_iter()
            .flatten()
        }
      }
    }
  });

  let is_none_fields = fields.iter().map(|(_, ident, _, _, _)| {
    quote! {
      self.#ident.is_none()
    }
  });

  // ===============
  // The final quote
  // ===============
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

    impl partial_derive2::MaybeNone for #partial_ident {
      fn is_none(&self) -> bool {
        #(#is_none_fields) &&*
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

    #derive_partial_diff
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
