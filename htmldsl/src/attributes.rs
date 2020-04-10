use super::units;
use htmldsl_internal::attr_traits::Attribute;
use htmldsl_internal::style_traits;

pub struct StyleAttr<'a> {
    pub values: Vec<&'a dyn style_traits::Style>,
}

impl<'a> StyleAttr<'a> {
    pub fn empty() -> Self {
        StyleAttr { values: Vec::new() }
    }

    pub fn new(styles: Vec<&'a dyn style_traits::Style>) -> Self {
        StyleAttr { values: styles }
    }
}

impl<'a> Attribute for StyleAttr<'a> {
    fn attr_key(&self) -> String {
        "style".into()
    }

    fn attr_value(&self) -> String {
        style_traits::render_styles(&self.values)
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

pub struct Src {
    pub value: units::SourceValue,
}

impl Attribute for Src {
    fn attr_key(&self) -> String {
        "src".into()
    }

    fn attr_value(&self) -> String {
        self.value.source_value_str()
    }
}

pub struct Href {
    pub value: units::SourceValue,
}

impl Attribute for Href {
    fn attr_key(&self) -> String {
        "href".into()
    }

    fn attr_value(&self) -> String {
        self.value.source_value_str()
    }
}

pub struct Action {
    pub value: units::SourceValue,
}

impl Attribute for Action {
    fn attr_key(&self) -> String {
        "action".into()
    }

    fn attr_value(&self) -> String {
        self.value.source_value_str()
    }
}

pub struct InputType {
    pub inner: units::InputTypeValue,
}

impl Attribute for InputType {
    fn attr_key(&self) -> String {
        "type".into()
    }

    fn attr_value(&self) -> String {
        self.inner.value_string()
    }
}

pub struct Value {
    pub inner: units::ValueString,
}

impl Attribute for Value {
    fn attr_key(&self) -> String {
        "value".into()
    }

    fn attr_value(&self) -> String {
        self.inner.value_string()
    }
}

pub struct Formmethod {
    pub inner: units::FormmethodValue,
}

impl Attribute for Formmethod {
    fn attr_key(&self) -> String {
        "method".into()
    }

    fn attr_value(&self) -> String {
        self.inner.value_string()
    }
}

pub struct Name {
    pub inner: units::ValueString,
}

impl Attribute for Name {
    fn attr_key(&self) -> String {
        "name".into()
    }

    fn attr_value(&self) -> String {
        self.inner.value_string()
    }
}
