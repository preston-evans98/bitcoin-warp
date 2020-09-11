use proc_macro::TokenStream;
use quote::quote;
use syn;

pub fn impl_ser_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = ast.ident.clone();
    let statements: Vec<quote::__private::TokenStream> = match ast.data {
        syn::Data::Struct(ref data) => data
            .fields
            .iter()
            .map(|field| serialize_field(field))
            .collect(), //.map(|field| &field.ty),
        _ => unimplemented!(),
    };

    let expanded = quote! {
        impl shared::Serializable for #name {
            fn serialize(&self, target: &mut Vec<u8>) {
                #(#statements)*
            }
        }
    };
    TokenStream::from(expanded)
}

fn serialize_field(field: &syn::Field) -> quote::__private::TokenStream {
    let ident = field.ident.clone().expect("Can only serialize named fields");
    quote! { self.#ident.serialize(target); }
}
