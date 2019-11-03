use std::fmt;

pub fn render_simple_html_page(html: Html) -> String {
    format!("<!DOCTYPE html>{}", render_tag_element(&html))
}

pub trait TagRenderable {
    fn get_name(&self) -> String;
    fn get_attributes(&self) -> Vec<&dyn Attribute>;
    fn get_children(&self) -> Vec<Renderable>;
}

pub enum Element {
    Tag(Box<dyn GenericElement>),
    Text(String),
}

impl Element {
    fn into_renderable(&self) -> Renderable {
        match self {
            Element::Tag(ref ge) => Renderable::Tag((**ge).as_tag_renderable()),
            Element::Text(ref t) => Renderable::Text(t.clone()),
        }
    }
}

pub enum Renderable<'a> {
    Tag(&'a dyn TagRenderable),
    Text(String),
}

impl<'a> Renderable<'a> {
    fn render(&self) -> String {
        match self {
            Renderable::Tag(ref tagged) => render_tag_element(&**tagged),
            Renderable::Text(t) => t.clone(),
        }
    }
}

pub trait GenericElement: AsTagRenderable + TagRenderable {
    fn is_generic_element_marker(&self);
}

pub trait AsTagRenderable {
    fn as_tag_renderable(&self) -> &dyn TagRenderable;
}

impl<T: GenericElement> AsTagRenderable for T {
    fn as_tag_renderable(&self) -> &dyn TagRenderable {
        self
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
        render_attributes(attrs),
        rendered_children,
        name
    )
}

pub struct Html {
    pub head: Option<Head>,
    pub body: Option<Body>,
    pub lang: Lang,
}

impl TagRenderable for Html {
    fn get_name(&self) -> String {
        "html".into()
    }

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        vec![&self.lang]
    }

    fn get_children(&self) -> Vec<Renderable> {
        let mut ret: Vec<Renderable> = Vec::new();
        match self.head {
            Some(ref v) => ret.push(Renderable::Tag(v)),
            None => (),
        }
        match self.body {
            Some(ref v) => ret.push(Renderable::Tag(v)),
            None => (),
        }
        ret
    }
}

pub struct Head {
    pub metas: Vec<Meta>,
}

impl TagRenderable for Head {
    fn get_name(&self) -> String {
        "head".into()
    }

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        vec![]
    }

    fn get_children(&self) -> Vec<Renderable> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.metas.iter() {
            ret.push(Renderable::Tag(m));
        }
        ret
    }
}

pub struct Meta {
    pub charset: Option<Charset>,
}

impl TagRenderable for Meta {
    fn get_name(&self) -> String {
        "meta".into()
    }

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        self.charset.as_ref().map_or(Vec::new(), |v| vec![v])
    }

    fn get_children(&self) -> Vec<Renderable> {
        vec![]
    }
}

pub struct Body {
    pub children: Vec<Element>,
}

impl TagRenderable for Body {
    fn get_name(&self) -> String {
        "body".into()
    }

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        vec![]
    }

    fn get_children(&self) -> Vec<Renderable> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        ret
    }
}

pub struct H1 {}

impl TagRenderable for H1 {
    fn get_name(&self) -> String {
        "h1".into()
    }

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        vec![]
    }

    fn get_children(&self) -> Vec<Renderable> {
        vec![]
    }
}

impl GenericElement for H1 {
    fn is_generic_element_marker(&self) {}
}

pub trait Attribute {
    fn attr_key(&self) -> String;
    fn attr_value(&self) -> String;
}

fn render_attribute(attribute: &dyn Attribute) -> String {
    format!("{}={}", attribute.attr_key(), attribute.attr_value())
}

fn render_attributes(attributes: Vec<&dyn Attribute>) -> String {
    attributes.into_iter().fold("".into(), |rendered, a| {
        format!("{} {}", rendered, render_attribute(a))
    })
}

pub struct Lang {
    pub tag: LanguageTag,
    pub sub_tag: LanguageSubTag,
}

impl Attribute for Lang {
    fn attr_key(&self) -> String {
        "lanq".into()
    }

    fn attr_value(&self) -> String {
        format!("{}-{}", self.tag, self.sub_tag)
    }
}

pub struct Charset {
    pub value: CharsetValue,
}

impl Attribute for Charset {
    fn attr_key(&self) -> String {
        "charset".into()
    }

    fn attr_value(&self) -> String {
        format!("{}", self.value)
    }
}

pub enum LanguageTag {
    En,
}

impl fmt::Display for LanguageTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_to_formatter(
            f,
            match self {
                LanguageTag::En => "en",
            },
        )
    }
}

pub enum LanguageSubTag {
    Us,
}

impl fmt::Display for LanguageSubTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_to_formatter(
            f,
            match self {
                LanguageSubTag::Us => "US",
            },
        )
    }
}

pub enum CharsetValue {
    Utf8,
}

impl fmt::Display for CharsetValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_to_formatter(
            f,
            match self {
                CharsetValue::Utf8 => "utf-8",
            },
        )
    }
}

fn write_to_formatter(f: &mut fmt::Formatter<'_>, message: &'static str) -> fmt::Result {
    write!(f, "{}", message)
}
