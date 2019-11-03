pub fn render_simple_html_page(html: Html) -> String {
    format!("<!DOCTYPE html>{}", render_tag_element(html))
}

trait TagRenderable {
    fn get_name() -> String;
}

fn render_tag_element<T: TagRenderable>(_tag_element: T) -> String {
    let name = T::get_name();
    format!("<{}></{}>", name, name)
}

pub struct Html {}

impl TagRenderable for Html {
    fn get_name() -> String {
        "html".into()
    }
}
