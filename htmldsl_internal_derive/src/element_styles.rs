use proc_macro::TokenStream;
use quote;
use syn;

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

pub fn impl_tag_renderable_style_setter(ast: &syn::DeriveInput) -> TokenStream {
    let mut o_field_name = None;
    for field in get_named_fields(&get_ast_data(&ast).fields).named.iter() {
        for attr in field.attrs.iter() {
            if attr.path.get_ident().unwrap().to_string() == "tag_renderable_style".to_string() {
                if o_field_name.is_none() {
                    o_field_name = match field.ident {
                        Some(ref v) => Some(quote! { #v }),
                        None => {
                            panic!("must have named field for renderable style setter derive gen")
                        }
                    };
                } else {
                    panic!("only one field may have the 'tag_renderable_style' attr");
                }
            }
        }
    }

    let field_name = match o_field_name {
        Some(s) => s,
        None => panic!("must provide the renderable style field"),
    };

    // Used in the quasi-quotation below as `#name`
    let name = &ast.ident;

    // Helper is provided for handling complex generic types correctly and effortlessly
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    // TODO the `&'a dyn ...` should actually be the lifetime from the impl_generics, but I don't
    // know how to extract that out.
    let ret = quote! {
        impl #impl_generics htmldsl_internal::element_traits::TagRenderableStyleSetter#impl_generics for #name #ty_generics #where_clause {
            fn add_style(mut self, styles: Vec<&'a dyn htmldsl_internal::style_traits::Style>) -> Self {
                self.#field_name.values = self.#field_name.values
                    .into_iter()
                    .chain(styles.into_iter())
                    .collect();
                self
            }
        }
    };
    ret.into()
}
