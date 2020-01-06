pub mod attributes;
pub mod elements;
pub mod style_sheet;
pub mod styles;
mod traits;
pub mod units;

pub use traits::element_traits::tag;
pub use traits::element_traits::text;
pub use traits::element_traits::Element;

pub fn render_simple_html_page(html: elements::Html) -> String {
    format!(
        "<!DOCTYPE html>{}",
        traits::element_traits::Renderable::Tag(&html).render()
    )
}
