extern crate proc_macro;

mod derive_enum;

#[proc_macro_derive(BSDieselEnum, attributes(bs_diesel))]
#[proc_macro_error::proc_macro_error]
pub fn derive_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  derive_enum::derive_enum(input).into()
}
