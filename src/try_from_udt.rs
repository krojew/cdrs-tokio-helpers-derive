use common::get_struct_fields;
use quote;
use syn;

pub fn impl_try_from_udt(ast: &syn::DeriveInput) -> quote::Tokens {
  let name = &ast.ident;
  let fields = get_struct_fields(ast);
  quote! {
      impl cdrs_tokio::frame::TryFromUDT for #name {
        fn try_from_udt(cdrs: cdrs_tokio::types::udt::UDT) -> cdrs_tokio::Result<Self> {
        use cdrs_tokio::frame::TryFromUDT;
        use cdrs_tokio::types::from_cdrs::FromCDRSByName;

          Ok(#name {
            #(#fields),*
          })
        }
      }
  }
}
