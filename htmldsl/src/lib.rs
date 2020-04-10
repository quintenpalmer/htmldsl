pub mod attributes;
pub mod elements;
pub mod style_sheet;
pub mod styles;
pub mod units;

pub use htmldsl_internal::element_traits::*;

#[macro_use]
extern crate htmldsl_internal_derive;

pub fn render_simple_html_page(pretty: bool, html: elements::Html) -> String {
    let tag = htmldsl_internal::element_traits::Renderable::Tag(&html);
    format!(
        "<!DOCTYPE html>{}",
        match pretty {
            true => tag.render_pretty(),
            false => tag.render(),
        }
    )
}
