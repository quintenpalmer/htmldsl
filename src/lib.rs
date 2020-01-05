mod attributes;
mod element;
mod styles;
mod traits;
mod units;

pub use attributes::Charset;
pub use attributes::Lang;
pub use attributes::StyleAttr;
pub use element::Body;
pub use element::Code;
pub use element::Div;
pub use element::Head;
pub use element::Html;
pub use element::Meta;
pub use element::Pre;
pub use element::Table;
pub use element::Tbody;
pub use element::Td;
pub use element::Th;
pub use element::Thead;
pub use element::Thr;
pub use element::Tr;
pub use element::H1;
pub use element::P;
pub use styles::BackgroundColor;
pub use styles::Border;
pub use styles::Height;
pub use styles::Margin;
pub use styles::MaxWidth;
pub use styles::Padding;
pub use traits::element_traits::Element;
pub use traits::style_traits::Style;
pub use units::Auto;
pub use units::BorderStyle;
pub use units::CharsetValue;
pub use units::LanguageSubTag;
pub use units::LanguageTag;
pub use units::Length;
pub use units::Number;
pub use units::NumberOrAuto;
pub use units::Percentage;

pub fn render_simple_html_page(html: Html) -> String {
    format!(
        "<!DOCTYPE html>{}",
        traits::element_traits::Renderable::Tag(&html).render()
    )
}
