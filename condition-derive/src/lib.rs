extern crate proc_macro;

use proc_macro::TokenStream;
use ::quote::quote;
use syn::{
  spanned::Spanned,
  parse_macro_input,
  DeriveInput,
  Result,
  Error,
};

#[proc_macro_derive(Condition)]
pub fn condition_derive(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  conditional(&ast)
    .unwrap_or_else(|err| err.to_compile_error())
    .into()
}

fn conditional (ast: &syn::DeriveInput) -> Result<proc_macro2::TokenStream> {
  let name = &ast.ident;
  let data = match &ast.data {
    syn::Data::Enum(ref data) => &data.variants,
    _ => return Err(Error::new(ast.span(), "Only enums may derive a Condition"))
  };
  if data.len() != 2 {
    return Err(Error::new(ast.span(), "Condition enums must have exactly two fields"));
  }
  // TODO: Permit #[false] and #[true] fields so that the order doesn't really
  // matter
  // Unwrapping should be fine here, since we KNOW there are two fields.
  let falsehood = data.first().unwrap();
  let truth = data.last().unwrap();

  let gen = quote! {
    impl Condition for #name {
      #[inline]
      fn is (&self, value: bool) -> bool {
        match self {
          Self::#falsehood => value == false,
          Self::#truth => value == true,
        }
      }
    }
  };
  Ok(gen.into())
}
