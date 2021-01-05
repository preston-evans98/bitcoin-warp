use proc_macro::TokenStream;
use quote::quote;
use syn;

pub fn payload(ast: &syn::DeriveInput) -> TokenStream {
    // let name = ast.ident.clone();
    // let default_impl = impl_default_ser_macro(&ast);
    // match ast.data {
    //     syn::DataStruct(&Data) => {
    //         let statements: Vec<quote::__private::TokenStream> =
    //             data.fields.iter().map(field_size).collect(); //.map(|field| &field.ty),

    //         quote! {
    //             impl shared::DefaultSerializable for #name {
    //                 fn default_serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    //                 where
    //                     W: std::io::Write,
    //                 {
    //                     #(#statements +)*
    //                     0;
    //                     Ok(())
    //                 }
    //             }
    //         }
    //     }
    // };

    TokenStream::from(quote! {})
}

fn field_size(field: &syn::Field) -> quote::__private::TokenStream {
    let ident = field
        .ident
        .clone()
        .expect("Can only get size of named fields");
    quote! { #ident.default_serialize(target)?; }
}
