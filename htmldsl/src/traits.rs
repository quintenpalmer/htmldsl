pub mod element_traits {
    use super::attr_traits;

    pub enum Element {
        Tag(Box<dyn GenericRenderable>),
        Text(String),
    }

    pub fn tag<'a>(r: Box<dyn GenericRenderable>) -> Element {
        Element::Tag(r)
    }

    pub fn text<'a>(s: String) -> Element {
        Element::Text(s)
    }

    impl<'a> Element {
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

    pub trait TagRenderable:
        TagRenderableName + TagRenderableAttrs + TagRenderableChildren
    {
    }

    pub trait TagRenderableName {
        fn get_name(&self) -> String;
    }

    pub trait TagRenderableAttrs {
        fn get_attributes(&self) -> Vec<&dyn attr_traits::Attribute>;
    }

    pub trait TagRenderableChildren {
        fn get_children(&self) -> Result<Vec<Renderable>, String>;
    }

    pub enum Renderable<'a> {
        Tag(&'a dyn TagRenderable),
        Text(String),
    }

    impl<'a> Renderable<'a> {
        pub fn render(&self) -> String {
            self.render_helper(None)
        }

        pub fn render_pretty(&self) -> String {
            self.render_helper(Some(0))
        }

        fn render_helper(&self, indent: Option<usize>) -> String {
            match self {
                Renderable::Tag(ref tag_element) => {
                    let name = tag_element.get_name();
                    let attrs = tag_element.get_attributes();
                    let children = tag_element.get_children();
                    let rendered_children = match children {
                        Ok(ref v) => v.iter().fold("".into(), |prev, curr| {
                            format!("{}{}", prev, curr.render_helper(indent.map(|x| x + 1)))
                        }),
                        Err(ref s) => s.clone(),
                    };

                    let (
                        leading_new_line,
                        leading_indent_string,
                        closing_new_line,
                        closing_indent_string,
                    ) = match indent {
                        Some(v) => {
                            let all_children_text = match children {
                                Ok(v) => v.iter().fold(true, |prev, curr| {
                                    prev && match curr {
                                        Renderable::Tag(_) => false,
                                        Renderable::Text(_) => true,
                                    }
                                }),
                                Err(_) => true,
                            };

                            let (closing_new_line, closing_indent_string): (String, String) =
                                match all_children_text {
                                    true => ("".into(), "".into()),
                                    false => ("\n".into(), "\t".repeat(v)),
                                };
                            (
                                "\n",
                                "\t".repeat(v),
                                closing_new_line,
                                closing_indent_string,
                            )
                        }
                        None => ("".into(), "".into(), "".into(), "".into()),
                    };

                    format!(
                        "{}{}<{}{}>{}{}{}</{}>",
                        leading_new_line,
                        leading_indent_string,
                        name,
                        attr_traits::render_attributes(attrs),
                        rendered_children,
                        closing_new_line,
                        closing_indent_string,
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
        format!("{}=\"{}\"", attribute.attr_key(), attribute.attr_value())
    }
}

pub mod style_traits {
    pub trait Style {
        fn style_key(&self) -> String;
        fn style_value(&self) -> String;
    }

    pub fn render_styles(styles: &Vec<&dyn Style>) -> String {
        styles.into_iter().fold("".into(), |rendered, s| {
            format!("{}; {}", render_style(*s), rendered)
        })
    }

    fn render_style(style: &dyn Style) -> String {
        format!("{}: {}", style.style_key(), style.style_value())
    }
}
