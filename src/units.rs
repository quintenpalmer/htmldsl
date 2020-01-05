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
