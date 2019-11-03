use std::fmt;

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

trait Attribute {
    fn attr_key(&self) -> String;
    fn attr_value(&self) -> String;
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
        write!(
            f,
            "{}",
            match self {
                LanguageTag::En => "en",
            }
        )
    }
}

pub enum LanguageSubTag {
    Us,
}

impl fmt::Display for LanguageSubTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LanguageSubTag::Us => "US",
            }
        )
    }
}
