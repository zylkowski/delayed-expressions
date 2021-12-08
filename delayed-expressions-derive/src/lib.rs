extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{ItemType, Type};

#[proc_macro_attribute]
pub fn default_execute(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemType = syn::parse(item).unwrap();
    let expanded = match *input.ty {
        Type::Path(ref path) => match &path.path.segments.first().unwrap().arguments {
            syn::PathArguments::AngleBracketed(brack) => {
                let generic_type = brack.args.first().unwrap();
                let alias_type = &input.ident;
                let expanded = quote! {
                    impl ExpressionExecute<#generic_type> for #alias_type
                    {
                        fn execute(self) -> #generic_type {
                            match self.op {
                                NodeType::Operation(operation) => {
                                    operation.execute(self.left.unwrap().execute(), self.right.unwrap().execute())
                                }
                                NodeType::Val(value) => Operand::get_value(value),
                            }
                        }
                    }
                };
                TokenStream::from(expanded)
            }
            _ => panic!("No generic!"),
        },
        _ => panic!("No input type!"),
    };

    let mut output = TokenStream::from(input.into_token_stream());
    output.extend(expanded);
    output
}
