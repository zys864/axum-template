use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(WithTag, attributes(tag))]
pub fn derive(input: TokenStream) -> TokenStream {
    let token = parse_macro_input!(input as DeriveInput);
    let opts = TagOpts::from_derive_input(&token).unwrap();
    // let attrs:  Vec<syn::Attribute> = input.call(Attribute::parse_outer).unwrap()
    println!("{:?}",opts.attrs);
    opts.to_token().into()
}
#[derive(FromDeriveInput, Debug)]
#[darling(attributes(tag), supports(struct_any))]
struct TagOpts {
    ident: syn::Ident,
    vis: syn::Visibility,
    tag: Option<String>,
    attrs: Vec<syn::Attribute>
}
impl TagOpts {
    fn to_token(self) -> proc_macro2::TokenStream {
        let tag = syn::Ident::new(&self.tag.unwrap_or("inner".into()), self.ident.span());
        let wrapper_struct_ident =
            syn::Ident::new(&format!("Taged{}", self.ident), self.ident.span());
        let inner_struct = self.ident;
        let vis = self.vis;

        quote::quote!(
            #[allow(unused)]
            #[derive(Debug, serde::Serialize, serde::Deserialize)]
            #vis struct #wrapper_struct_ident {
                #vis #tag: #inner_struct
            }
            impl #inner_struct{
                pub fn into_tagged(self)->#wrapper_struct_ident{
                    #wrapper_struct_ident{
                        #tag: self
                    }
                }
            }
        )
    }
}
