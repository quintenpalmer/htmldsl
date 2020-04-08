use proc_macro::TokenStream;
use quote;
use syn;

pub fn impl_tag_renderable(ast: &syn::DeriveInput) -> TokenStream {
    // Used in the quasi-quotation below as `#name`
    let name = &ast.ident;

    // Helper is provided for handling complex generic types correctly and effortlessly
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let ret = quote! {
        impl #impl_generics htmldsl_internal::element_traits::TagRenderable for #name #ty_generics #where_clause {}
    };
    ret.into()
}
