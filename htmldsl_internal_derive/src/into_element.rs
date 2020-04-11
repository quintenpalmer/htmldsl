use proc_macro::TokenStream;
use quote;
use syn;

pub fn impl_tag_renderable_into_element(ast: &syn::DeriveInput) -> TokenStream {
    // Used in the quasi-quotation below as `#name`
    let name = &ast.ident;

    // Helper is provided for handling complex generic types correctly and effortlessly
    let (_, _, where_clause) = ast.generics.split_for_impl();

    let ret = quote! {
        impl htmldsl_internal::element_traits::TagRenderableIntoElement for #name <'static> #where_clause {
            fn into_element(self) -> htmldsl_internal::element_traits::Element {
                htmldsl_internal::element_traits::Element::Tag(
                    Box::new(
                        self
                    )
                )
            }
        }
    };
    ret.into()
}
