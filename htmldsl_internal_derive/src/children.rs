use proc_macro::TokenStream;
use quote;
use syn;

use super::util;

fn get_ast_data(ast: &syn::DeriveInput) -> &syn::DataStruct {
    match ast.data {
        syn::Data::Struct(ref s) => s,
        _ => panic!("derive children only works on structs"),
    }
}

fn get_named_fields(fields: &syn::Fields) -> &syn::FieldsNamed {
    match fields {
        syn::Fields::Named(ref n) => n,
        _ => panic!("derive children only works with named fields"),
    }
}

enum ChildrenGenType {
    Element,
}

pub fn impl_tag_renderable_children(ast: &syn::DeriveInput) -> TokenStream {
    let mut o_gen_type = None;
    for field in get_named_fields(&get_ast_data(&ast).fields).named.iter() {
        for attr in field.attrs.iter() {
            if o_gen_type.is_none() {
                if attr.path.get_ident().unwrap().to_string()
                    == "tag_renderable_children".to_string()
                {
                    let metas: Vec<syn::NestedMeta> = match attr.parse_meta() {
                        Ok(syn::Meta::List(meta)) => meta.nested.into_iter().collect(),
                        _ => panic!("must supply list of 'key = \"value\"'"),
                    };

                    if metas.len() != 1 {
                        panic!("must supply exactly one 'tag_renderable_children(type = \"...\")'");
                    }

                    let named_value = match metas[0] {
                        syn::NestedMeta::Meta(syn::Meta::NameValue(ref m)) => {
                            if m.path.get_ident().unwrap().to_string() == "type".to_string() {
                                util::get_string_from_lit(&m.lit)
                            } else {
                                panic!("must have 'tag_renderable_children(**type** = \"...\"'")
                            }
                        }
                        _ => panic!("must supply a named value pair"),
                    };

                    let field_name = match field.ident {
                        Some(ref v) => v,
                        None => panic!("must have named field for renderable children derive gen"),
                    };

                    o_gen_type = match named_value.as_str() {
                        "element" => Some((ChildrenGenType::Element, quote! { #field_name })),
                        _ => panic!("unsupported generation type"),
                    };
                } else {
                    panic!("only one field may have the 'tag_renderable_children' attr");
                }
            }
        }
    }

    let (_gen_type, children_name) = match o_gen_type {
        Some(s) => s,
        None => panic!("must provide the renderable children generation type"),
    };

    // Used in the quasi-quotation below as `#name`
    let name = &ast.ident;

    // Helper is provided for handling complex generic types correctly and effortlessly
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let ret = quote! {
        impl #impl_generics htmldsl_internal::element_traits::TagRenderableChildren for #name #ty_generics #where_clause {
            fn get_children(&self) -> Result<Vec<Renderable>, String> {
                Ok(self.#children_name
                    .iter()
                    .map(|m| m.into_renderable())
                    .collect())
            }
        }
    };
    ret.into()
}
