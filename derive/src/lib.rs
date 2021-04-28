extern crate proc_macro;
use proc_macro::TokenStream;

/// A proc macro used to derive `ToResponse`
/// trait for data struct and its vector.
#[proc_macro_derive(ToResponse)]
pub fn derive_to_response(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    format!(
        "impl ToResponse for {} {{}}\n \
             impl ToResponse for Vec<{0}> {{}}\n",
        input.ident
    )
    .parse()
    .unwrap()
}
