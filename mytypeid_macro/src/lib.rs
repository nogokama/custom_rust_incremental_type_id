// mytypeid_macro/src/lib.rs
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(DerMyTypeId)]
pub fn my_type_id_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl mytypeid_trait::MyTypeIdStatic for #name {
            fn get_type_id_static() -> u16 {
                lazy_static! {
                    static ref TYPE_ID: u16 =
                        mytypeid_trait::COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
                TYPE_ID.clone()
            }
        }

        impl mytypeid_trait::MyTypeId for #name {
            fn get_type_id(&self) -> u16 {
                #name::get_type_id_static()
            }
        }
    };

    TokenStream::from(expanded)
}
