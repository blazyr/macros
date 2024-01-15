extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn entry_point(attr: TokenStream, item: TokenStream) -> TokenStream {
    let tokens = item;
    let ast: ItemFn = syn::parse(tokens).unwrap();
    let ast2: ItemFn = ast.clone();

    let return_type = ast.sig.output;
    let function_name = ast.sig.ident;

    let result_function = quote!(
        #ast2

        #[no_mangle]
        pub fn __entry_point() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
            #function_name().map_err(std::boxed::Box::from)
        }
    );
    TokenStream::from(result_function)
}

#[proc_macro_attribute]
pub fn entities(attr: TokenStream, item: TokenStream) -> TokenStream {
    let tokens = item;
    let ast: ItemFn = syn::parse(tokens).unwrap();
    let ast2: ItemFn = ast.clone();

    let return_type = ast.sig.output;
    let function_name = ast.sig.ident;

    let result_function = quote!(
        #ast2

        #[no_mangle]
        pub fn __data_transfer() -> std::result::Result<Vec<spotlight_extension::Entity>, std::boxed::Box<dyn std::error::Error>> {
            #function_name().map_err(std::boxed::Box::from)
        }
    );
    TokenStream::from(result_function)
}

#[proc_macro_attribute]
pub fn dispose(attr: TokenStream, item: TokenStream) -> TokenStream {
    let tokens = item;
    let ast: ItemFn = syn::parse(tokens).unwrap();
    let ast2: ItemFn = ast.clone();

    let return_type = ast.sig.output;
    let function_name = ast.sig.ident;

    let result_function = quote!(
        #ast2

        #[no_mangle]
        pub fn __dispose() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
            #function_name().map_err(std::boxed::Box::from)
        }
    );
    TokenStream::from(result_function)
}

#[cfg(test)]
mod test {
    // necessary for the TokenStream::from_str() implementation
    use std::str::FromStr;

    use proc_macro2::TokenStream;
    use quote::{format_ident, quote};
    use syn::{ItemFn, ItemStruct};

    #[test]
    fn entry_point_test() {
        // struct sample
        let s = r#"pub fn buzz() -> Result<String>{
            Ok("coucou".to_string())
        }"#;

        // create a new token stream from our string
        let tokens = TokenStream::from_str(s).unwrap();

        // build the AST: note the syn::parse2() method rather than the syn::parse() one
        // which is meant for "real" procedural macros
        let ast: ItemFn = syn::parse2(tokens).unwrap();

        let return_type = ast.sig.output;
        let function_name = ast.sig.ident;

        let result_function = quote!(
            #[no_mangle]
            pub fn entry_point() -> Result<()> {
                #function_name()
            }
        );

        println!("{}", result_function);
    }
}
