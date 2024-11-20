use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let builder_struct = Ident::new(&format!("{}Builder", ident), Span::call_site());

    quote! {
        impl #ident {
            pub fn builder() -> #builder_struct {
                #builder_struct {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }

            }
        }

        struct #builder_struct {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        impl #builder_struct {
            pub fn build(&mut self) -> Result<#ident, Box<dyn std::error::Error>> {
                let executable = if let Some(e) = self.executable.take() {
                    e
                } else {
                     return Err("missing executable".into());
                };

                let args = if let Some(a) = self.args.take() {
                    a
                } else {
                     return Err("missing args".into());
                };

                let env = if let Some(e) = self.env.take() {
                    e
                } else {
                     return Err("missing env".into());
                };

                let current_dir = if let Some(c) = self.current_dir.take() {
                    c
                } else {
                     return Err("missing current_dir".into());
                };

                Ok(#ident {
                    executable,
                    args,
                    env,
                    current_dir,
                })
            }

            fn executable(&mut self, executable: String) -> &mut Self {
                self.executable = Some(executable);
                self
            }

            fn args(&mut self, args: Vec<String>) -> &mut Self {
                self.args = Some(args);
                self
            }

            fn env(&mut self, env: Vec<String>) -> &mut Self {
                self.env = Some(env);
                self
            }


            fn current_dir(&mut self, current_dir: String) -> &mut Self {
                self.current_dir = Some(current_dir);
                self
            }
        }
    }
    .into()
}
