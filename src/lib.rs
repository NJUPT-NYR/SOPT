extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(ToResponse)]
pub fn derive_to_response(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let mut token = format!("impl ToResponse for {} ", input.ident);
    token.push_str("{}\n");
    let token_vec = format!("impl ToResponse for Vec<{}> ", input.ident);
    token.push_str(token_vec.as_str());
    token.push_str("{}\n");
    token.parse().unwrap()
}