use super::attributes;
use super::style_sheet;
use htmldsl_internal::attr_traits::Attribute;
use htmldsl_internal::element_traits::{
    Element, GenericRenderable, Renderable, TagRenderable, TagRenderableAttrs,
    TagRenderableChildren, TagRenderableName,
};

mod util {
    use crate::attributes;
    use htmldsl_internal::attr_traits::Attribute;

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

#[derive(TagRenderableName)]
#[tag_renderable_name(name = "html")]
pub struct Html<'a> {
    pub head: Option<Head<'a>>,
    pub body: Option<Body<'a>>,
    pub lang: attributes::Lang,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Html<'a> {}

impl<'a> TagRenderableAttrs for Html<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![&self.lang], &self.styles)
    }
}

impl<'a> TagRenderableChildren for Html<'a> {
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

impl<'a> TagRenderable for Head<'a> {}

impl<'a> TagRenderableName for Head<'a> {
    fn get_name(&self) -> String {
        "head".into()
    }
}

impl<'a> TagRenderableAttrs for Head<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        Vec::new()
    }
}

impl<'a> TagRenderableChildren for Head<'a> {
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

impl<'a> TagRenderable for Meta<'a> {}
impl<'a> TagRenderableName for Meta<'a> {
    fn get_name(&self) -> String {
        "meta".into()
    }
}

impl<'a> TagRenderableAttrs for Meta<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(
            self.charset.as_ref().map_or(Vec::new(), |v| vec![v]),
            &self.styles,
        )
    }
}

impl<'a> TagRenderableChildren for Meta<'a> {
    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        Ok(vec![])
    }
}

pub struct Style<'a> {
    pub style_sheet: style_sheet::StyleSheet<'a>,
}

impl<'a> TagRenderable for Style<'a> {}

impl<'a> TagRenderableName for Style<'a> {
    fn get_name(&self) -> String {
        "style".into()
    }
}

impl<'a> TagRenderableAttrs for Style<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        Vec::new()
    }
}

impl<'a> TagRenderableChildren for Style<'a> {
    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        Err(style_sheet::style_sheet_string(&self.style_sheet))
    }
}

pub struct Body<'a> {
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Body<'a> {}
impl<'a> TagRenderableName for Body<'a> {
    fn get_name(&self) -> String {
        "body".into()
    }
}

impl<'a> TagRenderableAttrs for Body<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

impl<'a> TagRenderableChildren for Body<'a> {
    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        Ok(ret)
    }
}

pub struct Div<'a> {
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Div<'a> {}
impl<'a> TagRenderableName for Div<'a> {
    fn get_name(&self) -> String {
        "div".into()
    }
}

impl<'a> TagRenderableAttrs for Div<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

impl<'a> TagRenderableChildren for Div<'a> {
    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        Ok(ret)
    }
}

impl<'a> GenericRenderable for Div<'a> {}

pub struct Span<'a> {
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Span<'a> {}
impl<'a> TagRenderableName for Span<'a> {
    fn get_name(&self) -> String {
        "span".into()
    }
}

impl<'a> TagRenderableAttrs for Span<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

impl<'a> TagRenderableChildren for Span<'a> {
    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        Ok(ret)
    }
}

impl<'a> GenericRenderable for Span<'a> {}

pub struct H1<'a> {
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for H1<'a> {}
impl<'a> TagRenderableName for H1<'a> {
    fn get_name(&self) -> String {
        "h1".into()
    }
}

impl<'a> TagRenderableAttrs for H1<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

impl<'a> TagRenderableChildren for H1<'a> {
    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        Ok(ret)
    }
}

impl<'a> GenericRenderable for H1<'a> {}

pub struct H2<'a> {
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for H2<'a> {}
impl<'a> TagRenderableName for H2<'a> {
    fn get_name(&self) -> String {
        "h2".into()
    }
}
impl<'a> TagRenderableAttrs for H2<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}
impl<'a> TagRenderableChildren for H2<'a> {
    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        Ok(ret)
    }
}

impl<'a> GenericRenderable for H2<'a> {}

pub struct H3<'a> {
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for H3<'a> {}
impl<'a> TagRenderableName for H3<'a> {
    fn get_name(&self) -> String {
        "h3".into()
    }
}
impl<'a> TagRenderableAttrs for H3<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}
impl<'a> TagRenderableChildren for H3<'a> {
    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        Ok(ret)
    }
}

impl<'a> GenericRenderable for H3<'a> {}

pub struct P<'a> {
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for P<'a> {}
impl<'a> TagRenderableName for P<'a> {
    fn get_name(&self) -> String {
        "p".into()
    }
}
impl<'a> TagRenderableAttrs for P<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}
impl<'a> TagRenderableChildren for P<'a> {
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

impl<'a> TagRenderable for Table<'a> {}
impl<'a> TagRenderableName for Table<'a> {
    fn get_name(&self) -> String {
        "table".into()
    }
}
impl<'a> TagRenderableAttrs for Table<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}
impl<'a> TagRenderableChildren for Table<'a> {
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

impl<'a> TagRenderable for Thead<'a> {}
impl<'a> TagRenderableName for Thead<'a> {
    fn get_name(&self) -> String {
        "thead".into()
    }
}
impl<'a> TagRenderableAttrs for Thead<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}
impl<'a> TagRenderableChildren for Thead<'a> {
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

impl<'a> TagRenderable for Thr<'a> {}
impl<'a> TagRenderableName for Thr<'a> {
    fn get_name(&self) -> String {
        "tr".into()
    }
}
impl<'a> TagRenderableAttrs for Thr<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}
impl<'a> TagRenderableChildren for Thr<'a> {
    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.ths.iter() {
            ret.push(Renderable::Tag(m));
        }
        Ok(ret)
    }
}

pub struct Th<'a> {
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Th<'a> {}
impl<'a> TagRenderableName for Th<'a> {
    fn get_name(&self) -> String {
        "th".into()
    }
}
impl<'a> TagRenderableAttrs for Th<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}
impl<'a> TagRenderableChildren for Th<'a> {
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

impl<'a> TagRenderable for Tbody<'a> {}
impl<'a> TagRenderableName for Tbody<'a> {
    fn get_name(&self) -> String {
        "tbody".into()
    }
}
impl<'a> TagRenderableAttrs for Tbody<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}
impl<'a> TagRenderableChildren for Tbody<'a> {
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

impl<'a> TagRenderable for Tr<'a> {}
impl<'a> TagRenderableName for Tr<'a> {
    fn get_name(&self) -> String {
        "tr".into()
    }
}
impl<'a> TagRenderableAttrs for Tr<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}
impl<'a> TagRenderableChildren for Tr<'a> {
    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.tds.iter() {
            ret.push(Renderable::Tag(m));
        }
        Ok(ret)
    }
}

pub struct Td<'a> {
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Td<'a> {}
impl<'a> TagRenderableName for Td<'a> {
    fn get_name(&self) -> String {
        "td".into()
    }
}
impl<'a> TagRenderableAttrs for Td<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}
impl<'a> TagRenderableChildren for Td<'a> {
    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        Ok(ret)
    }
}

pub struct Code<'a> {
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Code<'a> {}
impl<'a> TagRenderableName for Code<'a> {
    fn get_name(&self) -> String {
        "code".into()
    }
}
impl<'a> TagRenderableAttrs for Code<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}
impl<'a> TagRenderableChildren for Code<'a> {
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
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Pre<'a> {}
impl<'a> TagRenderableName for Pre<'a> {
    fn get_name(&self) -> String {
        "pre".into()
    }
}
impl<'a> TagRenderableAttrs for Pre<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}
impl<'a> TagRenderableChildren for Pre<'a> {
    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        Ok(ret)
    }
}

impl<'a> GenericRenderable for Pre<'a> {}

pub struct Img<'a> {
    pub src: attributes::Src,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Img<'a> {}
impl<'a> TagRenderableName for Img<'a> {
    fn get_name(&self) -> String {
        "img".into()
    }
}
impl<'a> TagRenderableAttrs for Img<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![&self.src], &self.styles)
    }
}
impl<'a> TagRenderableChildren for Img<'a> {
    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        Ok(Vec::new())
    }
}

impl<'a> GenericRenderable for Img<'a> {}

pub struct A<'a> {
    pub href: attributes::Href,
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for A<'a> {}
impl<'a> TagRenderableName for A<'a> {
    fn get_name(&self) -> String {
        "a".into()
    }
}
impl<'a> TagRenderableAttrs for A<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![&self.href], &self.styles)
    }
}
impl<'a> TagRenderableChildren for A<'a> {
    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut ret: Vec<Renderable> = Vec::new();
        for m in self.children.iter() {
            ret.push(m.into_renderable());
        }
        Ok(ret)
    }
}

impl<'a> GenericRenderable for A<'a> {}

pub struct Form<'a> {
    pub formmethod: attributes::Formmethod,
    pub action: Option<attributes::Action>,
    pub inputs: Vec<Input<'a>>,
    pub button: Button<'a>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Form<'a> {}
impl<'a> TagRenderableName for Form<'a> {
    fn get_name(&self) -> String {
        "form".into()
    }
}
impl<'a> TagRenderableAttrs for Form<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        let mut attrs: Vec<&dyn Attribute> = vec![&self.formmethod];
        match self.action {
            Some(ref a) => attrs.push(a),
            None => {}
        };
        util::full_attrs(attrs, &self.styles)
    }
}
impl<'a> TagRenderableChildren for Form<'a> {
    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        let mut children: Vec<Renderable> =
            self.inputs.iter().map(|x| Renderable::Tag(x)).collect();
        children.push(Renderable::Tag(&self.button));
        Ok(children)
    }
}

impl<'a> GenericRenderable for Form<'a> {}

pub struct Button<'a> {
    pub child: Element,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Button<'a> {}
impl<'a> TagRenderableName for Button<'a> {
    fn get_name(&self) -> String {
        "button".into()
    }
}
impl<'a> TagRenderableAttrs for Button<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}
impl<'a> TagRenderableChildren for Button<'a> {
    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        Ok(vec![self.child.into_renderable()])
    }
}

impl<'a> GenericRenderable for Button<'a> {}

pub struct Input<'a> {
    pub type_: Option<attributes::InputType>,
    pub name: attributes::Name,
    pub value: attributes::Value,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderable for Input<'a> {}
impl<'a> TagRenderableName for Input<'a> {
    fn get_name(&self) -> String {
        "input".into()
    }
}
impl<'a> TagRenderableAttrs for Input<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        let mut attrs: Vec<&dyn Attribute> = vec![&self.name, &self.value];
        match self.type_ {
            Some(ref a) => attrs.push(a),
            None => {}
        };
        util::full_attrs(attrs, &self.styles)
    }
}
impl<'a> TagRenderableChildren for Input<'a> {
    fn get_children(&self) -> Result<Vec<Renderable>, String> {
        Ok(Vec::new())
    }
}

impl<'a> GenericRenderable for Input<'a> {}
