use super::attributes;

pub fn render_simple_html_page(html: Html) -> String {
    format!("<!DOCTYPE html>{}", render_tag_element(&html))
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
    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute>;
    fn get_children(&self) -> Vec<Renderable>;
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
        attributes::render_attributes(attrs),
        rendered_children,
        name
    )
}

pub struct Html<'a> {
    pub head: Option<Head>,
    pub body: Option<Body>,
    pub lang: attributes::Lang,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Html<'a> {
    fn get_name(&self) -> String {
        "html".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        let mut ret: Vec<&dyn attributes::Attribute> = vec![&self.lang];
        if self.styles.values.len() > 0 {
            ret.push(&self.styles);
        }
        ret
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

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
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
    pub charset: Option<attributes::Charset>,
}

impl TagRenderable for Meta {
    fn get_name(&self) -> String {
        "meta".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
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

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
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

pub struct H1 {
    pub children: Vec<Element>,
}

impl TagRenderable for H1 {
    fn get_name(&self) -> String {
        "h1".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
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

impl GenericElement for H1 {
    fn is_generic_element_marker(&self) {}
}

pub struct P {
    pub children: Vec<Element>,
}

impl TagRenderable for P {
    fn get_name(&self) -> String {
        "p".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
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

impl GenericElement for P {
    fn is_generic_element_marker(&self) {}
}

pub struct Table {
    pub thead: Option<Thead>,
    pub tbody: Tbody,
}

impl TagRenderable for Table {
    fn get_name(&self) -> String {
        "table".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        vec![]
    }

    fn get_children(&self) -> Vec<Renderable> {
        let mut ret: Vec<Renderable> = Vec::new();
        match self.thead {
            Some(ref v) => ret.push(Renderable::Tag(v)),
            None => (),
        }
        ret.push(Renderable::Tag(&self.tbody));
        ret
    }
}

impl GenericElement for Table {
    fn is_generic_element_marker(&self) {}
}

pub struct Thead {
    pub trs: Vec<Thr>,
}

impl TagRenderable for Thead {
    fn get_name(&self) -> String {
        "thead".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        vec![]
    }

    fn get_children(&self) -> Vec<Renderable> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.trs.iter() {
            ret.push(Renderable::Tag(m));
        }
        ret
    }
}

pub struct Thr {
    pub ths: Vec<Th>,
}

impl TagRenderable for Thr {
    fn get_name(&self) -> String {
        "tr".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        vec![]
    }

    fn get_children(&self) -> Vec<Renderable> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.ths.iter() {
            ret.push(Renderable::Tag(m));
        }
        ret
    }
}

pub struct Th {
    pub children: Vec<Element>,
}

impl TagRenderable for Th {
    fn get_name(&self) -> String {
        "th".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
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

pub struct Tbody {
    pub trs: Vec<Tr>,
}

impl TagRenderable for Tbody {
    fn get_name(&self) -> String {
        "tbody".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        vec![]
    }

    fn get_children(&self) -> Vec<Renderable> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.trs.iter() {
            ret.push(Renderable::Tag(m));
        }
        ret
    }
}

pub struct Tr {
    pub tds: Vec<Td>,
}

impl TagRenderable for Tr {
    fn get_name(&self) -> String {
        "tr".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        vec![]
    }

    fn get_children(&self) -> Vec<Renderable> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.tds.iter() {
            ret.push(Renderable::Tag(m));
        }
        ret
    }
}

pub struct Td {
    pub children: Vec<Element>,
}

impl TagRenderable for Td {
    fn get_name(&self) -> String {
        "td".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
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

pub struct Code {
    pub children: Vec<Element>,
}

impl TagRenderable for Code {
    fn get_name(&self) -> String {
        "code".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
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

impl GenericElement for Code {
    fn is_generic_element_marker(&self) {}
}

pub struct Pre {
    pub children: Vec<Element>,
}

impl TagRenderable for Pre {
    fn get_name(&self) -> String {
        "pre".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
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

impl GenericElement for Pre {
    fn is_generic_element_marker(&self) {}
}
