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
        syn::Data::Enum(ref data) => {
            let variants: Vec<quote::__private::TokenStream> = data
                .variants
                .iter()
                .map(|variant| deserialize_variant(variant, &name))
                .collect();

            let expanded: quote::__private::TokenStream = quote! {
                impl shared::Deserializable for #name {
                    fn deserialize(&self, target: &mut BytesMut) -> Result<#name, std::io::Error>
                    {
                        match *self {
                            #(#variants)*
                        }
                        Ok(())
                    }
                }
            };
            return TokenStream::from(expanded);
        }
        _ => unimplemented!(),
    };

    let expanded = quote! {
        impl shared::Deserializable for #name {
            fn deserialize(target: &mut BytesMut) -> Result<Self, shared::DeserializationError>
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

fn deserialize_variant(variant: &syn::Variant, name: &syn::Ident) -> quote::__private::TokenStream {
    let ident = variant.ident.clone();

    // let subfields: Vec<quote::__private::TokenStream> = variant
    //     .fields
    //     .iter()
    //     .map(|field| {
    //         let ident = field
    //             .ident
    //             .clone()
    //             .expect("Can only derive serialize for named variant fields");
    //         quote! { ref #ident , }
    //     })
    //     .collect();

    let statements: Vec<quote::__private::TokenStream> = variant
        .fields
        .iter()
        .map(|field| {
            let ty = field.ty.clone();
            // let ident = field
            //     .ident
            //     .clone()
            //     .expect("Can only derive serialize for named variant fields");
            quote! { #ty::deserialize(&mut target)?; }
        })
        .collect();

    quote! { #name::#ident {
        #(#statements)*
    } }
}
