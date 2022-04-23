use proc_macro::TokenStream;
use syn::{DeriveInput, Meta, NestedMeta, parse_macro_input};
use quote::quote;


#[proc_macro_derive(Index, attributes(index_type))]
pub fn index(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;
    let attrs = &input.attrs;

    let mut res = Vec::new();
    
    // struct is annotated with [index_type(A, B, C)]
    // find A B C and impl Index and IndexMut for each and for Option<*>
    for attr in attrs.iter().filter(|attr| attr.path.is_ident("index_type")) {
        if let Ok(Meta::List(list)) = attr.parse_meta() {
            for element in list.nested {
                match element {
                    NestedMeta::Meta(Meta::Path(path)) => {
                        // this is one Identifier, e.g. A
                        let arg = path.get_ident().expect("no valid identifier");
                        //println!("arg: {:#?}", arg);

                        // impl Index and IndexMut
                        let q = quote! {
                            #[automatically_derived]
                            impl std::ops::Index<#ident> for Vec<#arg> {
                                type Output = #arg;
                                fn index(&self, index: #ident) -> &Self::Output {
                                    self.index(index.0)
                                }
                            }
                    
                            #[automatically_derived]
                            impl std::ops::IndexMut<#ident> for Vec<#arg> {
                                fn index_mut(&mut self, index: #ident) -> &mut Self::Output {
                                    self.index_mut(index.0)
                                }
                            }

                            #[automatically_derived]
                            impl std::ops::Index<#ident> for Vec<Option<#arg>> {
                                type Output = Option<#arg>;
                                fn index(&self, index: #ident) -> &Self::Output {
                                    self.index(index.0)
                                }
                            }
                    
                            #[automatically_derived]
                            impl std::ops::IndexMut<#ident> for Vec<Option<#arg>> {
                                fn index_mut(&mut self, index: #ident) -> &mut Self::Output {
                                    self.index_mut(index.0)
                                }
                            }
                        };
                        res.push(q);
                    },
                    _ => {},
                }
            }
        }
    }

    // concat
    let r = quote! {
        #(#res)*
    };

    r.into()
}
