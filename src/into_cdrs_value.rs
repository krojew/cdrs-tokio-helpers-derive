use syn;
use quote;

use common::get_ident_string;

pub fn impl_into_cdrs_value(ast: &syn::DeriveInput) -> quote::Tokens {
  let name = &ast.ident;
  if let syn::Body::Struct(syn::VariantData::Struct(ref fields)) = ast.body {
    let conver_into_bytes = fields.iter().map(|field| {
      let field_ident = field.ident.clone().unwrap();
      if get_ident_string(field.ty.clone()).as_str() == "Option" {
        return quote!{
          match self.#field_ident {
            Some(ref val) => {
              let field_bytes: cdrs_tokio::types::value::Bytes = val.clone().into();
              bytes.append(&mut cdrs_tokio::types::value::Value::new_normal(field_bytes).as_bytes());
            },
            None => {
              bytes.append(&mut cdrs_tokio::types::value::Value::new_not_set().as_bytes());
            }
          }
        };
      } else {
        return quote! {
          let field_bytes: cdrs_tokio::types::value::Bytes = self.#field_ident.into();
          bytes.append(&mut cdrs_tokio::types::value::Value::new_normal(field_bytes).as_bytes());
        };
      }
    });
    // As Value has following implementation impl<T: Into<Bytes>> From<T> for Value
    // for a struct it's enough to implement Into<Bytes> in order to be convertable into Value
    // wich is used for making queries
    quote! {
        #[allow(clippy::from_over_into)]
        impl Into<cdrs_tokio::types::value::Bytes> for #name {
          fn into(self) -> cdrs_tokio::types::value::Bytes {
            let mut bytes: Vec<u8> = vec![];
            #(#conver_into_bytes)*
            cdrs_tokio::types::value::Bytes::new(bytes)
          }
        }
    }
  } else {
    panic!("#[derive(IntoCDRSValue)] is only defined for structs, not for enums!");
  }
}
