extern crate proc_macro2;
extern crate proc_macro;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{DeriveInput};

#[proc_macro_derive(Describable)]
pub fn describable(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let code = impl_describable(&ast);
    TokenStream::from(code)
}

fn impl_describable(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    quote! {
        impl<'a> Describable for &'a #name {
            fn get_full(&self) -> Option<String> {
                self.description.clone().map(|d| d.full)
            }

            fn get_summary(&self) -> Option<String> {
                self.description.clone().map(|d| d.summary)
            }
        }
    }
}
