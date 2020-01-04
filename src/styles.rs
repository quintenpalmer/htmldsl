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
