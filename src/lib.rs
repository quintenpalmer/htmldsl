mod attributes;
mod element;
mod styles;
mod traits;
mod units;

pub use attributes::*;
pub use element::*;
pub use styles::*;
pub use traits::element_traits::Element;
pub use traits::style_traits::Style;
pub use units::*;

pub fn render_simple_html_page(html: Html) -> String {
    format!(
        "<!DOCTYPE html>{}",
        traits::element_traits::Renderable::Tag(&html).render()
    )
}
