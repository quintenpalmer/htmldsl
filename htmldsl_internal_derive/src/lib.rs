#![recursion_limit = "192"]

#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

mod generic;
mod name;
mod tag;

use proc_macro::TokenStream;

#[proc_macro_derive(TagRenderable)]
pub fn derive_tag_renderable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    // Build the output, possibly using quasi-quotation
    let expanded = tag::impl_tag_renderable(&input);

    // Parse back to a token stream and return it
    expanded
}

#[proc_macro_derive(GenericRenderable)]
pub fn derive_generic_renderable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    // Build the output, possibly using quasi-quotation
    let expanded = generic::impl_generic_renderable(&input);

    // Parse back to a token stream and return it
    expanded
}

#[proc_macro_derive(TagRenderableName, attributes(tag_renderable_name))]
pub fn derive_tag_renderable_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    // Build the output, possibly using quasi-quotation
    let expanded = name::impl_tag_renderable_name(&input);

    // Parse back to a token stream and return it
    expanded
}
