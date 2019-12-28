extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn inject(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    let output: TokenStream2 = {
        unimplemented!()
    };
    TokenStream::from(output)
}
