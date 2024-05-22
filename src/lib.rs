// proc macro crate

use proc_macro::TokenStream;
use quote::quote;

// for enum, w'd like to generate From impls for enum variants
// impl From<DirectionUp> for Direction {
//     fn from(up: DirectionUp) -> Self {
//         Direction::Up(up)
//     }
// }
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    // println!("{:#?}", input);

    // get the ident
    let ident = input.ident;

    // get enum variants
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom derive macro only works on enums"),
    };

    // for each variant, get the ident and fields
    let form_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                // only support one field
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = &fields.unnamed.first().expect("should have one field");
                    let ty = &field.ty;
                    quote! {
                        impl From<#ty> for #ident {
                            fn from(v: #ty) -> Self {
                                #ident::#var(v)
                            }
                        }
                    }
                }
            }
            syn::Fields::Named(_fields) => quote! {},
            syn::Fields::Unit => quote! {},
        }
    });

    quote! {
        #(#form_impls)*
    }
    .into()
}
