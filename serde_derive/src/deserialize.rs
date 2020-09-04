use proc_macro::TokenStream;
use quote::quote;
use syn;
pub fn impl_deser_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = ast.ident.clone();
    let statements: Vec<quote::__private::TokenStream> = match ast.data {
        // syn::Data::Struct(ref data) => &data.fields, //.map(|field| &field.ty),
        syn::Data::Struct(ref data) => data
            .fields
            .iter()
            .map(|field| deserialize_field(field))
            .collect(), //.map(|field| &field.ty),
        _ => unimplemented!(),
    };

    let expanded = quote! {
        impl shared::Deserializable for #name {
            fn deserialize<R>(target: &mut R) -> Result<Self, shared::DeserializationError>
            where
                R: std::io::Read,
            {
                Ok(#name {
                    #(#statements)*
                })
            }
        }
    };
    TokenStream::from(expanded)
}

fn deserialize_field(field: &syn::Field) -> quote::__private::TokenStream {
    let name = field.ident.clone().expect("Missing identifier for field");
    let ty = field.ty.clone();
    quote! { #name: <#ty as shared::Deserializable>::deserialize(target)?, }
    // quote! { #name: 0, }
    // quote! { #name: format!("shared::<{}>::deserialize(target),", #ty)  }
}
