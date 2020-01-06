pub mod element_traits {
    use super::attr_traits;

    pub enum Element<'a> {
        Tag(&'a dyn GenericRenderable),
        Text(String),
    }

    pub fn tag<'a>(r: &'a dyn GenericRenderable) -> Element<'a> {
        Element::Tag(r)
    }

    pub fn text<'a>(s: String) -> Element<'a> {
        Element::Text(s)
    }

    impl<'a> Element<'a> {
        pub fn into_renderable(&self) -> Renderable {
            match self {
                Element::Tag(ref ge) => Renderable::Tag((**ge).as_tag_renderable()),
                Element::Text(ref t) => Renderable::Text(t.clone()),
            }
        }
    }

    pub trait GenericRenderable: AsTagRenderable + TagRenderable {}

    impl<T: GenericRenderable> AsTagRenderable for T {
        fn as_tag_renderable(&self) -> &dyn TagRenderable {
            self
        }
    }

    pub trait AsTagRenderable {
        fn as_tag_renderable(&self) -> &dyn TagRenderable;
    }

    pub trait TagRenderable {
        fn get_name(&self) -> String;
        fn get_attributes(&self) -> Vec<&dyn attr_traits::Attribute>;
        fn get_children(&self) -> Result<Vec<Renderable>, String>;
    }

    pub enum Renderable<'a> {
        Tag(&'a dyn TagRenderable),
        Text(String),
    }

    impl<'a> Renderable<'a> {
        pub fn render(&self) -> String {
            match self {
                Renderable::Tag(ref tag_element) => {
                    let name = tag_element.get_name();
                    let attrs = tag_element.get_attributes();
                    let rendered_children = match tag_element.get_children() {
                        Ok(v) => v
                            .into_iter()
                            .fold("".into(), |prev, curr| format!("{}{}", prev, curr.render())),
                        Err(s) => s,
                    };
                    format!(
                        "<{}{}>{}</{}>",
                        name,
                        attr_traits::render_attributes(attrs),
                        rendered_children,
                        name
                    )
                }
                Renderable::Text(t) => t.clone(),
            }
        }
    }
}

pub mod attr_traits {
    pub trait Attribute {
        fn attr_key(&self) -> String;
        fn attr_value(&self) -> String;
    }

    pub fn render_attributes(attributes: Vec<&dyn Attribute>) -> String {
        attributes.into_iter().fold("".into(), |rendered, a| {
            format!("{} {}", rendered, render_attribute(a))
        })
    }

    fn render_attribute(attribute: &dyn Attribute) -> String {
        format!("{}={}", attribute.attr_key(), attribute.attr_value())
    }
}

pub mod style_traits {
    pub trait Style {
        fn style_key(&self) -> String;
        fn style_value(&self) -> String;
    }

    pub fn render_styles(styles: &Vec<&dyn Style>) -> String {
        format!(
            "\"{}\"",
            styles.into_iter().fold("".into(), |rendered, s| {
                format!("{}; {}", render_style(*s), rendered)
            })
        )
    }

    fn render_style(style: &dyn Style) -> String {
        format!("{}: {}", style.style_key(), style.style_value())
    }
}
