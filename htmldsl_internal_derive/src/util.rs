use syn;

pub fn get_string_from_lit(lit: &syn::Lit) -> String {
    if let syn::Lit::Str(ref s) = *lit {
        s.value()
    } else {
        panic!(format!(
            "expected tag_renderable_name attribute to be a string: `... = \"...\"`",
        ));
    }
}
