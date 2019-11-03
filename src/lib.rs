use std::fmt;

pub fn render_simple_html_page(html: Html) -> String {
    format!("<!DOCTYPE html>{}", render_tag_element(&html))
}

trait TagRenderable {
    fn get_name(&self) -> String;
    fn get_attributes(&self) -> Vec<&dyn Attribute>;
}

fn render_tag_element(tag_element: &dyn TagRenderable) -> String {
    let name = tag_element.get_name();
    let attrs = tag_element.get_attributes();
    format!("<{}{}></{}>", name, render_attributes(attrs), name)
}

pub struct Html {
    pub lang: Lang,
}

impl TagRenderable for Html {
    fn get_name(&self) -> String {
        "html".into()
    }

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        vec![&self.lang]
    }
}

trait Attribute {
    fn attr_key(&self) -> String;
    fn attr_value(&self) -> String;
}

fn render_attribute(attribute: &dyn Attribute) -> String {
    format!("{}={}", attribute.attr_key(), attribute.attr_value())
}

fn render_attributes(attributes: Vec<&dyn Attribute>) -> String {
    attributes.into_iter().fold("".into(), |rendered, a| {
        format!("{} {}", rendered, render_attribute(a))
    })
}

pub struct Lang {
    pub tag: LanguageTag,
    pub sub_tag: LanguageSubTag,
}

impl Attribute for Lang {
    fn attr_key(&self) -> String {
        "lanq".into()
    }

    fn attr_value(&self) -> String {
        format!("{}-{}", self.tag, self.sub_tag)
    }
}

pub enum LanguageTag {
    En,
}

impl fmt::Display for LanguageTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_to_formatter(
            f,
            match self {
                LanguageTag::En => "en",
            },
        )
    }
}

pub enum LanguageSubTag {
    Us,
}

impl fmt::Display for LanguageSubTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_to_formatter(
            f,
            match self {
                LanguageSubTag::Us => "US",
            },
        )
    }
}

fn write_to_formatter(f: &mut fmt::Formatter<'_>, message: &'static str) -> fmt::Result {
    write!(f, "{}", message)
}
