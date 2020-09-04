extern crate proc_macro;
use proc_macro::TokenStream;

mod deserialize;
mod serialize;

#[proc_macro_derive(Deserializable)]
pub fn deserializable(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    deserialize::impl_deser_macro(&ast)
}

#[proc_macro_derive(Serializable)]
pub fn serializable(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    serialize::impl_ser_macro(&ast)
}
