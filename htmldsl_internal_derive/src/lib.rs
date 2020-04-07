#![recursion_limit = "192"]

#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(TagRenderableName, attributes(tag_renderable_name))]
pub fn derive_tag_renderable_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    // Build the output, possibly using quasi-quotation
    let expanded = impl_tag_renderable_name(&input);

    // Parse back to a token stream and return it
    expanded
}

fn impl_tag_renderable_name(ast: &syn::DeriveInput) -> TokenStream {
    let tag_renderable_name_usage = "#[tag_renderable_name(name = \"html_element_name\")]";

    let mut o_name = None;
    for meta_items in ast.attrs.iter().filter_map(get_renderable_name_meta_items) {
        for meta_item in meta_items {
            match meta_item {
                // Parse `#[tag_renderable_name(name = "foo")]`
                syn::NestedMeta::Meta(syn::Meta::NameValue(ref m))
                    if m.path.get_ident().unwrap().to_string() == "name" =>
                {
                    let s = get_string_from_lit(&m.lit);
                    o_name = Some(s);
                }
                _ => panic!(format!(
                    "incorrect usage of custom attribute, use: {}",
                    tag_renderable_name_usage
                )),
            }
        }
    }

    let renderable_name = match o_name {
        Some(s) => s,
        None => panic!(format!(
            "must provide the renderable name: {}",
            tag_renderable_name_usage
        )),
    };

    // Used in the quasi-quotation below as `#name`
    let name = &ast.ident;

    // Helper is provided for handling complex generic types correctly and effortlessly
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let ret = quote! {
        impl #impl_generics htmldsl_internal::element_traits::TagRenderableName for #name #ty_generics #where_clause {
            fn get_name(&self) -> String {
                return #renderable_name.to_string();
            }
        }
    };
    ret.into()
}

fn get_renderable_name_meta_items(attr: &syn::Attribute) -> Option<Vec<syn::NestedMeta>> {
    if attr.path.get_ident().unwrap().to_string() != "tag_renderable_name" {
        return Some(Vec::new());
    }

    match attr.parse_meta() {
        Ok(syn::Meta::List(meta)) => Some(meta.nested.into_iter().collect()),
        _ => None,
    }
}

fn get_string_from_lit(lit: &syn::Lit) -> String {
    if let syn::Lit::Str(ref s) = *lit {
        s.value()
    } else {
        panic!(format!(
            "expected tag_renderable_name attribute to be a string: `... = \"...\"`",
        ));
    }
}
