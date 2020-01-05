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
