use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derived_input = parse_macro_input!(input as DeriveInput);

    let target_struct_name = derived_input.ident;
    let builder_struct_name =
        Ident::new(&format!("{}Builder", target_struct_name), Span::call_site());

    let fields = &builder_fields(&derived_input.data);

    let init_builder_fields = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        quote! {
            #ident: None
        }
    });

    let builder_function = quote! {
        impl #target_struct_name {
            pub fn builder() -> #builder_struct_name {
                #builder_struct_name {
                    #(#init_builder_fields,)*
                }
            }
        }
    };

    let builder_fields = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        quote! {
            #ident: ::std::option::Option<#ty>
        }
    });

    let builder_struct = quote! {
        struct #builder_struct_name {
            #(#builder_fields,)*
        }
    };

    let build_fn_setters = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        quote! {
            let #ident = self.#ident.take().unwrap();
        }
    });

    let build_fn_fields = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        quote! {
            #ident
        }
    });

    let build_fn = quote! {
        pub fn build(&mut self) -> ::core::result::Result<#target_struct_name, Box<dyn std::error::Error>> {
            #(#build_fn_setters)*

            Ok(#target_struct_name {
                #(#build_fn_fields,)*
            })
        }
    };

    let builder_fns = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;

        quote! {
            fn #ident(&mut self, #ident: #ty) -> &mut Self {
                self.#ident = Some(#ident);
                self
            }
        }
    });

    let ts = quote! {
        #builder_function

        #builder_struct

        impl #builder_struct_name {
            #build_fn

            #(#builder_fns)*
        }
    };

    eprintln!("TOKENS: {:#?}", ts);

    proc_macro::TokenStream::from(ts)
}

fn builder_fields(data: &Data) -> Vec<&Field> {
    match *data {
        Data::Struct(ref ds) => match ds.fields {
            syn::Fields::Named(ref f) => f.named.iter().collect(),
            syn::Fields::Unnamed(_) | syn::Fields::Unit => {
                unimplemented!("#[derive(Builder)] can only be used on structs")
            }
        },
        Data::Enum(_) | Data::Union(_) => {
            unimplemented!("#[derive(Builder)] can only be used on structs")
        }
    }
}

fn is_optional_field(data: Data, name: &str) -> bool {
    match data {
        Data::Struct(ds) => {
            let fields = ds.fields;
            match fields {
                syn::Fields::Named(f) => {
                    for field in f.named {
                        let fname = quote! {#field.ident}.to_string();
                        let ty = quote! {#field.ty}.to_string();
                        if fname == name && ty.starts_with("Option") {
                            return true;
                        }
                    }
                    false
                }
                syn::Fields::Unnamed(_) | syn::Fields::Unit => false,
            }
        }
        Data::Enum(_) | Data::Union(_) => false,
    }
}
