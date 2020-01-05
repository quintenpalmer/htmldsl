use super::units;

pub trait Style {
    fn style_key(&self) -> String;
    fn style_value(&self) -> String;
}

pub enum Number {
    Length(u32, units::Length),
    Percentage(u32),
}

impl Number {
    fn style_value_helper(&self) -> String {
        match self {
            Number::Length(v, l) => format!("{}{}", v, l.unit_str()),
            Number::Percentage(v) => format!("{}{}", v, units::Percentage {}.unit_str()),
        }
    }
}

pub enum NumberOrAuto {
    Number(Number),
    Auto,
}

impl NumberOrAuto {
    fn style_value_helper(&self) -> String {
        match self {
            NumberOrAuto::Number(n) => n.style_value_helper(),
            NumberOrAuto::Auto => units::Auto {}.unit_str(),
        }
    }
}

pub enum Margin {
    AllFour(NumberOrAuto),
    VerticalHorizontal(NumberOrAuto, NumberOrAuto),
    TopHorizontalBotton(NumberOrAuto, NumberOrAuto, NumberOrAuto),
    TopRightBottonLeft(NumberOrAuto, NumberOrAuto, NumberOrAuto, NumberOrAuto),
}

impl Style for Margin {
    fn style_key(&self) -> String {
        "margin".into()
    }

    fn style_value(&self) -> String {
        match self {
            Margin::AllFour(v) => v.style_value_helper(),
            Margin::VerticalHorizontal(v, h) => {
                format!("{} {}", v.style_value_helper(), h.style_value_helper())
            }
            Margin::TopHorizontalBotton(t, h, b) => format!(
                "{} {} {}",
                t.style_value_helper(),
                h.style_value_helper(),
                b.style_value_helper()
            ),
            Margin::TopRightBottonLeft(t, r, b, l) => format!(
                "{} {} {} {}",
                t.style_value_helper(),
                r.style_value_helper(),
                b.style_value_helper(),
                l.style_value_helper()
            ),
        }
    }
}

pub enum Padding {
    AllFour(Number),
    VerticalHorizontal(Number, Number),
    TopHorizontalBotton(Number, Number, Number),
    TopRightBottonLeft(Number, Number, Number, Number),
}

impl Style for Padding {
    fn style_key(&self) -> String {
        "padding".into()
    }

    fn style_value(&self) -> String {
        match self {
            Padding::AllFour(v) => v.style_value_helper(),
            Padding::VerticalHorizontal(v, h) => {
                format!("{} {}", v.style_value_helper(), h.style_value_helper())
            }
            Padding::TopHorizontalBotton(t, h, b) => format!(
                "{} {} {}",
                t.style_value_helper(),
                h.style_value_helper(),
                b.style_value_helper()
            ),
            Padding::TopRightBottonLeft(t, r, b, l) => format!(
                "{} {} {} {}",
                t.style_value_helper(),
                r.style_value_helper(),
                b.style_value_helper(),
                l.style_value_helper()
            ),
        }
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
    pub value: NumberOrAuto,
}

impl Style for Height {
    fn style_key(&self) -> String {
        "height".into()
    }

    fn style_value(&self) -> String {
        self.value.style_value_helper()
    }
}
