use super::units;

pub trait Style {
    fn style_key(&self) -> String;
    fn style_value(&self) -> String;
}

pub enum NumberLikeValue {
    Length(u32, units::Length),
    Percentage(u32, units::Percentage),
    Auto(units::Auto),
}

impl NumberLikeValue {
    fn style_value_helper(&self) -> String {
        match self {
            NumberLikeValue::Length(v, l) => format!("{}{}", v, l.unit_str()),
            NumberLikeValue::Percentage(v, p) => format!("{}{}", v, p.unit_str()),
            NumberLikeValue::Auto(a) => a.unit_str(),
        }
    }
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

pub struct BackgroundColor {
    pub color_value: String,
}

impl Style for BackgroundColor {
    fn style_key(&self) -> String {
        "background-color".into()
    }

    fn style_value(&self) -> String {
        self.color_value.clone()
    }
}

pub struct Height {
    pub value: NumberLikeValue,
}

impl Style for Height {
    fn style_key(&self) -> String {
        "height".into()
    }

    fn style_value(&self) -> String {
        self.value.style_value_helper()
    }
}
