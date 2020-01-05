pub mod element_traits {
    use super::attr_traits;
    use crate::attributes;

    pub enum Element {
        Tag(Box<dyn GenericElement>),
        Text(String),
    }

    impl Element {
        pub fn into_renderable(&self) -> Renderable {
            match self {
                Element::Tag(ref ge) => Renderable::Tag((**ge).as_tag_renderable()),
                Element::Text(ref t) => Renderable::Text(t.clone()),
            }
        }
    }

    pub trait GenericElement: AsTagRenderable + TagRenderable {
        fn is_generic_element_marker(&self);
    }

    impl<T: GenericElement> AsTagRenderable for T {
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
        fn get_children(&self) -> Vec<Renderable>;
    }

    pub enum Renderable<'a> {
        Tag(&'a dyn TagRenderable),
        Text(String),
    }

    impl<'a> Renderable<'a> {
        pub fn render(&self) -> String {
            match self {
                Renderable::Tag(ref tagged) => render_tag_element(&**tagged),
                Renderable::Text(t) => t.clone(),
            }
        }
    }

    fn render_tag_element(tag_element: &dyn TagRenderable) -> String {
        let name = tag_element.get_name();
        let attrs = tag_element.get_attributes();
        let rendered_children = tag_element
            .get_children()
            .into_iter()
            .fold("".into(), |prev, curr| format!("{}{}", prev, curr.render()));
        format!(
            "<{}{}>{}</{}>",
            name,
            attr_traits::render_attributes(attrs),
            rendered_children,
            name
        )
    }

    pub fn full_attrs<'a>(
        mut attrs: Vec<&'a dyn attr_traits::Attribute>,
        styles: &'a attributes::StyleAttr<'a>,
    ) -> Vec<&'a dyn attr_traits::Attribute> {
        if styles.values.len() > 0 {
            attrs.push(styles);
        }
        attrs
    }
}

pub mod attr_traits {
    pub trait Attribute {
        fn attr_key(&self) -> String;
        fn attr_value(&self) -> String;
    }

    fn render_attribute(attribute: &dyn Attribute) -> String {
        format!("{}={}", attribute.attr_key(), attribute.attr_value())
    }

    pub fn render_attributes(attributes: Vec<&dyn Attribute>) -> String {
        attributes.into_iter().fold("".into(), |rendered, a| {
            format!("{} {}", rendered, render_attribute(a))
        })
    }
}

pub mod style_traits {
    pub trait Style {
        fn style_key(&self) -> String;
        fn style_value(&self) -> String;
    }
}
