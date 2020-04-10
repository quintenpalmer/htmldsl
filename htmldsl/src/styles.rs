use super::units;
use htmldsl_internal::style_traits::Style;

pub enum Display {
    Inline,
    Grid,
    None,
}

impl Style for Display {
    fn style_key(&self) -> String {
        "display".into()
    }

    fn style_value(&self) -> String {
        match self {
            Display::Inline => "inline".into(),
            Display::Grid => "grid".into(),
            Display::None => "none".into(),
        }
    }
}

pub enum Margin {
    AllFour(units::NumberOrAuto),
    VerticalHorizontal(units::NumberOrAuto, units::NumberOrAuto),
    TopHorizontalBotton(
        units::NumberOrAuto,
        units::NumberOrAuto,
        units::NumberOrAuto,
    ),
    TopRightBottonLeft(
        units::NumberOrAuto,
        units::NumberOrAuto,
        units::NumberOrAuto,
        units::NumberOrAuto,
    ),
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
    AllFour(units::Number),
    VerticalHorizontal(units::Number, units::Number),
    TopHorizontalBotton(units::Number, units::Number, units::Number),
    TopRightBottonLeft(units::Number, units::Number, units::Number, units::Number),
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

pub struct Border {
    pub style: units::BorderStyle,
}

impl Style for Border {
    fn style_key(&self) -> String {
        "border".into()
    }

    fn style_value(&self) -> String {
        self.style.unit_str()
    }
}

pub struct BorderCollapse {
    pub value: units::BorderCollapseStyle,
}

impl Style for BorderCollapse {
    fn style_key(&self) -> String {
        "border-collapse".into()
    }

    fn style_value(&self) -> String {
        self.value.unit_str()
    }
}

pub struct BorderSpacing {
    pub value: units::Number,
}

impl Style for BorderSpacing {
    fn style_key(&self) -> String {
        "border-spacing".into()
    }

    fn style_value(&self) -> String {
        self.value.style_value_helper()
    }
}

pub struct VerticalAlign {
    pub value: units::VerticalAlignValue,
}

impl Style for VerticalAlign {
    fn style_key(&self) -> String {
        "vertical-align".into()
    }

    fn style_value(&self) -> String {
        self.value.value_string()
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
        format!("{}px", self.value)
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

pub struct Color {
    pub color_value: String,
}

impl Style for Color {
    fn style_key(&self) -> String {
        "color".into()
    }

    fn style_value(&self) -> String {
        self.color_value.clone()
    }
}

pub struct FontFamily {
    pub name: String,
}

impl Style for FontFamily {
    fn style_key(&self) -> String {
        "font-family".into()
    }

    fn style_value(&self) -> String {
        self.name.clone()
    }
}

pub struct Height {
    pub value: units::NumberOrAuto,
}

impl Style for Height {
    fn style_key(&self) -> String {
        "height".into()
    }

    fn style_value(&self) -> String {
        self.value.style_value_helper()
    }
}

pub struct Width {
    pub value: units::NumberOrAuto,
}

impl Style for Width {
    fn style_key(&self) -> String {
        "width".into()
    }

    fn style_value(&self) -> String {
        self.value.style_value_helper()
    }
}

pub struct Visibility {
    pub value: units::VisibilityValue,
}

impl Style for Visibility {
    fn style_key(&self) -> String {
        "visibility".into()
    }

    fn style_value(&self) -> String {
        self.value.style_value_helper()
    }
}
