mod my_derive;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Implements `MyDerive` trait
#[proc_macro_derive(MyDerive, attributes(my_derive))]
pub fn my_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    TokenStream::from(my_derive::impl_my_trait(ast))
}
