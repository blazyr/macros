extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn entry_point(attr: TokenStream, item: TokenStream) -> TokenStream {
    let tokens = item;
    let ast: ItemFn = syn::parse(tokens).unwrap();

    let return_type = ast.sig.output;
    let function_name = ast.sig.ident;

    let result_function = quote!({
        #[no_mangle]
        pub fn entry_point() -> Result<()> {
            #function_name()
        }
    });
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

        let result_function = quote!({
            #[no_mangle]
            pub fn entry_point() -> Result<()> {
                #function_name()
            }
        });

        println!("{}", result_function);
    }

    #[test]
    fn entities_transfer_test() {
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

        let result_function = quote!({
            #[no_mangle]
            pub fn entities_transfer() -> Result<Vec<Entity>> {
                #function_name()
            }
        });

        println!("{}", result_function);
    }

    #[test]
    fn dispose_test() {
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

        let result_function = quote!({
            #[no_mangle]
            pub fn dispose() -> Result<()> {
                #function_name()
            }
        });

        println!("{}", result_function);
    }
}
