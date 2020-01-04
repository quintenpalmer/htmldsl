use std::fmt;

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
    pub values: Vec<&'a dyn Style>,
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

pub struct Charset {
    pub value: CharsetValue,
}

impl Attribute for Charset {
    fn attr_key(&self) -> String {
        "charset".into()
    }

    fn attr_value(&self) -> String {
        format!("{}", self.value)
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

pub enum CharsetValue {
    Utf8,
}

impl fmt::Display for CharsetValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_to_formatter(
            f,
            match self {
                CharsetValue::Utf8 => "utf-8",
            },
        )
    }
}

pub trait Style {
    fn style_key(&self) -> String;
    fn style_value(&self) -> String;
}

pub struct Margin {
    pub value: u32,
}

impl Style for Margin {
    fn style_key(&self) -> String {
        "margin".into()
    }

    fn style_value(&self) -> String {
        format!("{} auto", self.value)
    }
}

pub struct MaxWidth {
    pub value: u32,
}

impl Style for MaxWidth {
    fn style_key(&self) -> String {
        "max-width".into()
    }

    fn style_value(&self) -> String {
        format!("{}cm", self.value)
    }
}

fn write_to_formatter(f: &mut fmt::Formatter<'_>, message: &'static str) -> fmt::Result {
    write!(f, "{}", message)
}
