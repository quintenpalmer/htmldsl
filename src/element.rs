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

fn full_attrs<'a>(
    mut attrs: Vec<&'a dyn attributes::Attribute>,
    styles: &'a attributes::StyleAttr<'a>,
) -> Vec<&'a dyn attributes::Attribute> {
    if styles.values.len() > 0 {
        attrs.push(styles);
    }
    attrs
}

pub struct Html<'a> {
    pub head: Option<Head<'a>>,
    pub body: Option<Body<'a>>,
    pub lang: attributes::Lang,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Html<'a> {
    fn get_name(&self) -> String {
        "html".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        full_attrs(vec![&self.lang], &self.styles)
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

pub struct Head<'a> {
    pub metas: Vec<Meta<'a>>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Head<'a> {
    fn get_name(&self) -> String {
        "head".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Vec<Renderable> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.metas.iter() {
            ret.push(Renderable::Tag(m));
        }
        ret
    }
}

pub struct Meta<'a> {
    pub charset: Option<attributes::Charset>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Meta<'a> {
    fn get_name(&self) -> String {
        "meta".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        full_attrs(
            self.charset.as_ref().map_or(Vec::new(), |v| vec![v]),
            &self.styles,
        )
    }

    fn get_children(&self) -> Vec<Renderable> {
        vec![]
    }
}

pub struct Body<'a> {
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Body<'a> {
    fn get_name(&self) -> String {
        "body".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Vec<Renderable> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        ret
    }
}

pub struct Div<'a> {
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Div<'a> {
    fn get_name(&self) -> String {
        "div".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Vec<Renderable> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        ret
    }
}

impl<'a> GenericElement for Div<'a> {
    fn is_generic_element_marker(&self) {}
}

pub struct H1<'a> {
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for H1<'a> {
    fn get_name(&self) -> String {
        "h1".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Vec<Renderable> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        ret
    }
}

impl<'a> GenericElement for H1<'a> {
    fn is_generic_element_marker(&self) {}
}

pub struct P<'a> {
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for P<'a> {
    fn get_name(&self) -> String {
        "p".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Vec<Renderable> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        ret
    }
}

impl<'a> GenericElement for P<'a> {
    fn is_generic_element_marker(&self) {}
}

pub struct Table<'a> {
    pub thead: Option<Thead<'a>>,
    pub tbody: Tbody<'a>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Table<'a> {
    fn get_name(&self) -> String {
        "table".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        full_attrs(vec![], &self.styles)
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

impl<'a> GenericElement for Table<'a> {
    fn is_generic_element_marker(&self) {}
}

pub struct Thead<'a> {
    pub trs: Vec<Thr<'a>>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Thead<'a> {
    fn get_name(&self) -> String {
        "thead".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Vec<Renderable> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.trs.iter() {
            ret.push(Renderable::Tag(m));
        }
        ret
    }
}

pub struct Thr<'a> {
    pub ths: Vec<Th<'a>>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Thr<'a> {
    fn get_name(&self) -> String {
        "tr".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Vec<Renderable> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.ths.iter() {
            ret.push(Renderable::Tag(m));
        }
        ret
    }
}

pub struct Th<'a> {
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Th<'a> {
    fn get_name(&self) -> String {
        "th".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Vec<Renderable> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        ret
    }
}

pub struct Tbody<'a> {
    pub trs: Vec<Tr<'a>>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Tbody<'a> {
    fn get_name(&self) -> String {
        "tbody".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Vec<Renderable> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.trs.iter() {
            ret.push(Renderable::Tag(m));
        }
        ret
    }
}

pub struct Tr<'a> {
    pub tds: Vec<Td<'a>>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Tr<'a> {
    fn get_name(&self) -> String {
        "tr".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Vec<Renderable> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.tds.iter() {
            ret.push(Renderable::Tag(m));
        }
        ret
    }
}

pub struct Td<'a> {
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Td<'a> {
    fn get_name(&self) -> String {
        "td".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Vec<Renderable> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        ret
    }
}

pub struct Code<'a> {
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Code<'a> {
    fn get_name(&self) -> String {
        "code".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Vec<Renderable> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        ret
    }
}

impl<'a> GenericElement for Code<'a> {
    fn is_generic_element_marker(&self) {}
}

pub struct Pre<'a> {
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Pre<'a> {
    fn get_name(&self) -> String {
        "pre".into()
    }

    fn get_attributes(&self) -> Vec<&dyn attributes::Attribute> {
        full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Vec<Renderable> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        ret
    }
}

impl<'a> GenericElement for Pre<'a> {
    fn is_generic_element_marker(&self) {}
}
