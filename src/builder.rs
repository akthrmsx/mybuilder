use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, FieldsNamed};

pub fn implement(input: DeriveInput) -> TokenStream {
    let product_name = input.ident;
    let visibility = input.vis;
    let builder_name = format_ident!("{}Builder", product_name);
    let fields_named = extract_fields_named(&input.data);

    let builder_fields = fields_named.named.iter().map(|field| {
        let identifier = field.ident.clone().unwrap();
        let ty = field.ty.clone();
        quote! {
            #identifier: Option<#ty>
        }
    });

    let initial_values = fields_named.named.iter().map(|field| {
        let identifier = field.ident.clone().unwrap();
        quote! {
            #identifier: None
        }
    });

    quote! {
        #visibility struct #builder_name {
            #(#builder_fields),*
        }

        impl #product_name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#initial_values),*
                }
            }
        }
    }
}

fn extract_fields_named(data: &Data) -> FieldsNamed {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields.clone(),
            _ => panic!("expects named fields"),
        },
        _ => panic!("expects struct"),
    }
}
