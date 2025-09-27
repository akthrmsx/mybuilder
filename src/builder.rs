use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Data, DeriveInput, Fields, FieldsNamed, GenericArgument, PathArguments, PathSegment, Type,
};

pub fn implement(input: DeriveInput) -> TokenStream {
    let product_name = input.ident;
    let visibility = input.vis;
    let builder_name = format_ident!("{}Builder", product_name);
    let fields = extract_named_fields(&input.data);

    let builder_fields = fields.named.iter().map(|field| {
        let identifier = field.ident.clone().unwrap();
        let ty = field.ty.clone();

        if is_option(&ty) {
            quote! {
                #identifier: #ty
            }
        } else {
            quote! {
                #identifier: Option<#ty>
            }
        }
    });

    let initial_values = fields.named.iter().map(|field| {
        let identifier = field.ident.clone().unwrap();
        quote! {
            #identifier: None
        }
    });

    let set_values = fields.named.iter().map(|field| {
        let identifier = field.ident.clone().unwrap();
        let ty = {
            let ty = &field.ty;

            if is_option(ty) {
                unwrap_type_argument(ty).clone()
            } else {
                ty.clone()
            }
        };
        quote! {
            #visibility fn #identifier(&mut self, #identifier: #ty) -> &mut Self {
                self.#identifier = Some(#identifier);
                self
            }
        }
    });

    let validate_values = fields
        .named
        .iter()
        .filter(|field| !is_option(&field.ty))
        .map(|field| {
            let identifier = field.ident.clone().unwrap();
            let err = format!("required field '{}' is missing", identifier);
            quote! {
                if self.#identifier.is_none() {
                    return Err(#err.into());
                }
            }
        });

    let build_values = fields.named.iter().map(|field| {
        let identifier = field.ident.clone().unwrap();
        let ty = field.ty.clone();

        if is_option(&ty) {
            quote! {
                #identifier: self.#identifier.clone()
            }
        } else {
            quote! {
                #identifier: self.#identifier.clone().unwrap()
            }
        }
    });

    quote! {
        #visibility struct #builder_name {
            #(#builder_fields),*
        }

        impl #builder_name {
            #(#set_values)*

            #visibility fn build(&mut self) -> Result<#product_name, Box<dyn std::error::Error>> {
                #(#validate_values)*

                Ok(#product_name {
                    #(#build_values),*
                })
            }
        }

        impl #product_name {
            #visibility fn builder() -> #builder_name {
                #builder_name {
                    #(#initial_values),*
                }
            }
        }
    }
}

fn extract_named_fields(data: &Data) -> &FieldsNamed {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields,
            _ => panic!("expects named fields"),
        },
        _ => panic!("expects struct"),
    }
}

fn extract_last_path_segment(ty: &Type) -> Option<&PathSegment> {
    match ty {
        Type::Path(ty) => ty.path.segments.last(),
        _ => None,
    }
}

fn extract_first_generic_argument(segment: &PathSegment) -> Option<&GenericArgument> {
    match &segment.arguments {
        PathArguments::AngleBracketed(arguments) => arguments.args.first(),
        _ => None,
    }
}

fn is_option(ty: &Type) -> bool {
    match extract_last_path_segment(ty) {
        Some(segment) => segment.ident == "Option",
        _ => false,
    }
}

fn unwrap_type_argument(ty: &Type) -> &Type {
    match extract_last_path_segment(ty) {
        Some(segment) => match extract_first_generic_argument(segment) {
            Some(GenericArgument::Type(ty)) => ty,
            _ => panic!("expects type argument"),
        },
        _ => panic!("expects segment"),
    }
}
