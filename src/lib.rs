pub mod attributes;
pub mod element;
pub mod styles;
mod traits;
pub mod units;

pub use traits::element_traits::Element;

pub fn render_simple_html_page(html: element::Html) -> String {
    format!(
        "<!DOCTYPE html>{}",
        traits::element_traits::Renderable::Tag(&html).render()
    )
}
