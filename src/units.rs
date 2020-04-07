pub enum LanguageTag {
    En,
}

impl LanguageTag {
    pub fn lang_tag_str(&self) -> String {
        match self {
            LanguageTag::En => "en".into(),
        }
    }
}

pub enum LanguageSubTag {
    Us,
}

impl LanguageSubTag {
    pub fn sub_tag_str(&self) -> String {
        match self {
            LanguageSubTag::Us => "US".into(),
        }
    }
}

pub enum CharsetValue {
    Utf8,
}

impl CharsetValue {
    pub fn char_set_str(&self) -> String {
        match self {
            CharsetValue::Utf8 => "utf-8".into(),
        }
    }
}

pub struct SourceValue {
    inner_string: String,
}

impl SourceValue {
    pub fn new(s: String) -> Self {
        SourceValue { inner_string: s }
    }

    pub fn source_value_str(&self) -> String {
        self.inner_string.clone()
    }
}

pub enum InputTypeValue {
    Submit,
}

impl InputTypeValue {
    pub fn value_string(&self) -> String {
        match self {
            InputTypeValue::Submit => "submit".into(),
        }
    }
}

pub struct ValueString {
    inner_string: String,
}

impl ValueString {
    pub fn new(s: String) -> Self {
        ValueString { inner_string: s }
    }

    pub fn value_string(&self) -> String {
        self.inner_string.clone()
    }
}

pub enum FormmethodValue {
    Get,
    Post,
    Dialog,
}

impl FormmethodValue {
    pub fn value_string(&self) -> String {
        match self {
            FormmethodValue::Get => "get".into(),
            FormmethodValue::Post => "post".into(),
            FormmethodValue::Dialog => "dialog".into(),
        }
    }
}

pub enum Length {
    Pixel,
    Centimeter,
    ViewportHeight,
}

impl Length {
    pub fn unit_str(&self) -> String {
        match self {
            Length::Pixel => "px".into(),
            Length::Centimeter => "cm".into(),
            Length::ViewportHeight => "vh".into(),
        }
    }
}

pub struct Percentage {}

impl Percentage {
    pub fn unit_str(&self) -> String {
        "%".into()
    }
}

pub struct Auto {}

impl Auto {
    pub fn unit_str(&self) -> String {
        "auto".into()
    }
}

pub enum BorderStyle {
    Solid,
    None,
}

impl BorderStyle {
    pub fn unit_str(&self) -> String {
        match self {
            BorderStyle::Solid => "solid".into(),
            BorderStyle::None => "0px".into(),
        }
    }
}

pub enum Number {
    Length(u32, Length),
    Percentage(u32),
}

impl Number {
    pub fn style_value_helper(&self) -> String {
        match self {
            Number::Length(v, l) => format!("{}{}", v, l.unit_str()),
            Number::Percentage(v) => format!("{}{}", v, Percentage {}.unit_str()),
        }
    }
}

pub enum NumberOrAuto {
    Number(Number),
    Auto,
}

impl NumberOrAuto {
    pub fn style_value_helper(&self) -> String {
        match self {
            NumberOrAuto::Number(n) => n.style_value_helper(),
            NumberOrAuto::Auto => Auto {}.unit_str(),
        }
    }
}

pub enum VisibilityValue {
    Visible,
    Hidden,
    Collapse,
}

impl VisibilityValue {
    pub fn style_value_helper(&self) -> String {
        match self {
            VisibilityValue::Visible => "visible",
            VisibilityValue::Hidden => "hidden",
            VisibilityValue::Collapse => "collapse",
        }
        .into()
    }
}
