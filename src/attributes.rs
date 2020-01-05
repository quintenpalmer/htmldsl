use super::styles;
use super::units;

pub trait Attribute {
    fn attr_key(&self) -> String;
    fn attr_value(&self) -> String;
}

fn render_attribute(attribute: &dyn Attribute) -> String {
    format!("{}={}", attribute.attr_key(), attribute.attr_value())
}

pub fn render_attributes(attributes: Vec<&dyn Attribute>) -> String {
    attributes.into_iter().fold("".into(), |rendered, a| {
        format!("{} {}", rendered, render_attribute(a))
    })
}

pub struct StyleAttr<'a> {
    pub values: Vec<&'a dyn styles::Style>,
}

impl<'a> Attribute for StyleAttr<'a> {
    fn attr_key(&self) -> String {
        "style".into()
    }

    fn attr_value(&self) -> String {
        format!(
            "\"{}\"",
            self.values.iter().fold("".into(), |rendered, a| {
                format!("{}; {}: {};", rendered, a.style_key(), a.style_value())
            })
        )
    }
}

pub struct Lang {
    pub tag: units::LanguageTag,
    pub sub_tag: units::LanguageSubTag,
}

impl Attribute for Lang {
    fn attr_key(&self) -> String {
        "lanq".into()
    }

    fn attr_value(&self) -> String {
        format!("{}-{}", self.tag.lang_tag_str(), self.sub_tag.sub_tag_str())
    }
}

pub struct Charset {
    pub value: units::CharsetValue,
}

impl Attribute for Charset {
    fn attr_key(&self) -> String {
        "charset".into()
    }

    fn attr_value(&self) -> String {
        self.value.char_set_str()
    }
}
