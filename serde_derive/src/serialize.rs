use proc_macro::TokenStream;
use quote::quote;
use syn;

pub fn impl_ser_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = ast.ident.clone();
    match ast.data {
        syn::Data::Struct(ref data) => {
            let statements: Vec<quote::__private::TokenStream> = data
                .fields
                .iter()
                .map(|field| serialize_field(field))
                .collect(); //.map(|field| &field.ty),

            let expanded = quote! {
                impl shared::Serializable for #name {
                    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
                    where
                        W: std::io::Write,
                    {
                        #(#statements)*
                        Ok(())
                    }
                }
            };
            return TokenStream::from(expanded);
        }
        syn::Data::Enum(ref data) => {
            let variants: Vec<quote::__private::TokenStream> = data
                .variants
                .iter()
                .map(|variant| serialize_variant(variant, &name))
                .collect();
            // vec![quoted]

            let expanded: quote::__private::TokenStream = quote! {
                impl shared::Serializable for #name {
                    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
                    where
                        W: std::io::Write,
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
    }
}

fn serialize_field(field: &syn::Field) -> quote::__private::TokenStream {
    let ident = field
        .ident
        .clone()
        .expect("Can only serialize named fields");
    quote! { self.#ident.serialize(target)?; }
}

// fn serialize_ref(field: &syn::Field) -> quote::__private::TokenStream {
//     let ident = field
//         .ident
//         .clone()
//         .expect("Can only serialize named fields");
//     quote! { #ident.serialize(target)?; }
// }

fn serialize_variant(variant: &syn::Variant, name: &syn::Ident) -> quote::__private::TokenStream {
    let ident = variant.ident.clone();

    let subfields: Vec<quote::__private::TokenStream> = variant
        .fields
        .iter()
        .map(|field| {
            let ident = field
                .ident
                .clone()
                .expect("Can only derive serialize for named variant fields");
            quote! { ref #ident , }
        })
        .collect();

    let statements: Vec<quote::__private::TokenStream> = variant
        .fields
        .iter()
        .map(|field| {
            let ident = field
                .ident
                .clone()
                .expect("Can only derive serialize for named variant fields");
            quote! { #ident.serialize(target)?; }
        })
        .collect();

    quote! { #name::#ident { #(#subfields)* } => {
        #(#statements)*
    } }
}
