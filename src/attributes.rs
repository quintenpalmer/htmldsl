use super::traits::attr_traits::Attribute;
use super::traits::style_traits;
use super::units;

pub struct StyleAttr<'a> {
    pub values: Vec<&'a dyn style_traits::Style>,
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
