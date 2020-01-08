use super::attributes;
use super::style_sheet;
use super::traits::attr_traits::Attribute;
use super::traits::element_traits::{Element, GenericRenderable, Renderable, TagRenderable};

mod util {
    use crate::attributes;
    use crate::traits::attr_traits::Attribute;

    pub fn full_attrs<'a>(
        mut attrs: Vec<&'a dyn Attribute>,
        styles: &'a attributes::StyleAttr<'a>,
    ) -> Vec<&'a dyn Attribute> {
        if styles.values.len() > 0 {
            attrs.push(styles);
        }
        attrs
    }
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

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![&self.lang], &self.styles)
    }

    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        match self.head {
            Some(ref v) => ret.push(Renderable::Tag(v)),
            None => (),
        }
        match self.body {
            Some(ref v) => ret.push(Renderable::Tag(v)),
            None => (),
        }
        Ok(ret)
    }
}

pub struct Head<'a> {
    pub metas: Vec<Meta<'a>>,
    pub styles: Vec<Style<'a>>,
}

impl<'a> TagRenderable for Head<'a> {
    fn get_name(&self) -> String {
        "head".into()
    }

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        Vec::new()
    }

    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.metas.iter() {
            ret.push(Renderable::Tag(m));
        }
        for s in self.styles.iter() {
            ret.push(Renderable::Tag(s));
        }
        Ok(ret)
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

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(
            self.charset.as_ref().map_or(Vec::new(), |v| vec![v]),
            &self.styles,
        )
    }

    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        Ok(vec![])
    }
}

pub struct Style<'a> {
    pub style_sheet: style_sheet::StyleSheet<'a>,
}

impl<'a> TagRenderable for Style<'a> {
    fn get_name(&self) -> String {
        "style".into()
    }

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        Vec::new()
    }

    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        Err(style_sheet::style_sheet_string(&self.style_sheet))
    }
}

pub struct Body<'a> {
    pub children: Vec<Element<'a>>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Body<'a> {
    fn get_name(&self) -> String {
        "body".into()
    }

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        Ok(ret)
    }
}

pub struct Div<'a> {
    pub children: Vec<Element<'a>>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Div<'a> {
    fn get_name(&self) -> String {
        "div".into()
    }

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        Ok(ret)
    }
}

impl<'a> GenericRenderable for Div<'a> {}

pub struct H1<'a> {
    pub children: Vec<Element<'a>>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for H1<'a> {
    fn get_name(&self) -> String {
        "h1".into()
    }

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        Ok(ret)
    }
}

impl<'a> GenericRenderable for H1<'a> {}

pub struct P<'a> {
    pub children: Vec<Element<'a>>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for P<'a> {
    fn get_name(&self) -> String {
        "p".into()
    }

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        Ok(ret)
    }
}

impl<'a> GenericRenderable for P<'a> {}

pub struct Table<'a> {
    pub thead: Option<Thead<'a>>,
    pub tbody: Tbody<'a>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Table<'a> {
    fn get_name(&self) -> String {
        "table".into()
    }

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        match self.thead {
            Some(ref v) => ret.push(Renderable::Tag(v)),
            None => (),
        }
        ret.push(Renderable::Tag(&self.tbody));
        Ok(ret)
    }
}

impl<'a> GenericRenderable for Table<'a> {}

pub struct Thead<'a> {
    pub trs: Vec<Thr<'a>>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Thead<'a> {
    fn get_name(&self) -> String {
        "thead".into()
    }

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.trs.iter() {
            ret.push(Renderable::Tag(m));
        }
        Ok(ret)
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

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.ths.iter() {
            ret.push(Renderable::Tag(m));
        }
        Ok(ret)
    }
}

pub struct Th<'a> {
    pub children: Vec<Element<'a>>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Th<'a> {
    fn get_name(&self) -> String {
        "th".into()
    }

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        Ok(ret)
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

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.trs.iter() {
            ret.push(Renderable::Tag(m));
        }
        Ok(ret)
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

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.tds.iter() {
            ret.push(Renderable::Tag(m));
        }
        Ok(ret)
    }
}

pub struct Td<'a> {
    pub children: Vec<Element<'a>>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Td<'a> {
    fn get_name(&self) -> String {
        "td".into()
    }

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        Ok(ret)
    }
}

pub struct Code<'a> {
    pub children: Vec<Element<'a>>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Code<'a> {
    fn get_name(&self) -> String {
        "code".into()
    }

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        Ok(ret)
    }
}

impl<'a> GenericRenderable for Code<'a> {}

pub struct Pre<'a> {
    pub children: Vec<Element<'a>>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Pre<'a> {
    fn get_name(&self) -> String {
        "pre".into()
    }

    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }

    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        Ok(ret)
    }
}

impl<'a> GenericRenderable for Pre<'a> {}