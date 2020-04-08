use super::attributes;
use super::style_sheet;
use htmldsl_internal::attr_traits::Attribute;
use htmldsl_internal::element_traits::{
    Element, Renderable, TagRenderableAttrs, TagRenderableChildren,
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

#[derive(TagRenderable, TagRenderableName)]
#[tag_renderable_name(name = "html")]
pub struct Html<'a> {
    pub head: Option<Head<'a>>,
    pub body: Option<Body<'a>>,
    pub lang: attributes::Lang,
    pub styles: attributes::StyleAttr<'a>,
}

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

#[derive(TagRenderable, TagRenderableName)]
#[tag_renderable_name(name = "head")]
pub struct Head<'a> {
    pub metas: Vec<Meta<'a>>,
    pub styles: Vec<Style<'a>>,
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

#[derive(TagRenderable, TagRenderableName)]
#[tag_renderable_name(name = "meta")]
pub struct Meta<'a> {
    pub charset: Option<attributes::Charset>,
    pub styles: attributes::StyleAttr<'a>,
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

#[derive(TagRenderable, TagRenderableName)]
#[tag_renderable_name(name = "style")]
pub struct Style<'a> {
    pub style_sheet: style_sheet::StyleSheet<'a>,
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

#[derive(TagRenderableName, TagRenderable, TagRenderableChildren)]
#[tag_renderable_name(name = "body")]
pub struct Body<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderableAttrs for Body<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(TagRenderableName, GenericRenderable, TagRenderable, TagRenderableChildren)]
#[tag_renderable_name(name = "div")]
pub struct Div<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderableAttrs for Div<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(TagRenderableName, GenericRenderable, TagRenderable, TagRenderableChildren)]
#[tag_renderable_name(name = "span")]
pub struct Span<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderableAttrs for Span<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(TagRenderableName, GenericRenderable, TagRenderable, TagRenderableChildren)]
#[tag_renderable_name(name = "h1")]
pub struct H1<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderableAttrs for H1<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(TagRenderableName, GenericRenderable, TagRenderable, TagRenderableChildren)]
#[tag_renderable_name(name = "h2")]
pub struct H2<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderableAttrs for H2<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(TagRenderableName, GenericRenderable, TagRenderable, TagRenderableChildren)]
#[tag_renderable_name(name = "h3")]
pub struct H3<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderableAttrs for H3<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(TagRenderableName, GenericRenderable, TagRenderable, TagRenderableChildren)]
#[tag_renderable_name(name = "p")]
pub struct P<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderableAttrs for P<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(TagRenderableName, GenericRenderable, TagRenderable)]
#[tag_renderable_name(name = "table")]
pub struct Table<'a> {
    pub thead: Option<Thead<'a>>,
    pub tbody: Tbody<'a>,
    pub styles: attributes::StyleAttr<'a>,
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

#[derive(TagRenderableName, TagRenderable)]
#[tag_renderable_name(name = "thead")]
pub struct Thead<'a> {
    pub trs: Vec<Thr<'a>>,
    pub styles: attributes::StyleAttr<'a>,
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

#[derive(TagRenderableName, TagRenderable)]
#[tag_renderable_name(name = "tr")]
pub struct Thr<'a> {
    pub ths: Vec<Th<'a>>,
    pub styles: attributes::StyleAttr<'a>,
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

#[derive(TagRenderableName, TagRenderable, TagRenderableChildren)]
#[tag_renderable_name(name = "th")]
pub struct Th<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderableAttrs for Th<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(TagRenderableName, TagRenderable)]
#[tag_renderable_name(name = "tbody")]
pub struct Tbody<'a> {
    pub trs: Vec<Tr<'a>>,
    pub styles: attributes::StyleAttr<'a>,
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

#[derive(TagRenderableName, TagRenderable)]
#[tag_renderable_name(name = "tr")]
pub struct Tr<'a> {
    pub tds: Vec<Td<'a>>,
    pub styles: attributes::StyleAttr<'a>,
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

#[derive(TagRenderableName, TagRenderable, TagRenderableChildren)]
#[tag_renderable_name(name = "td")]
pub struct Td<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderableAttrs for Td<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(TagRenderableName, GenericRenderable, TagRenderable, TagRenderableChildren)]
#[tag_renderable_name(name = "code")]
pub struct Code<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderableAttrs for Code<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(TagRenderableName, GenericRenderable, TagRenderable, TagRenderableChildren)]
#[tag_renderable_name(name = "pre")]
pub struct Pre<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderableAttrs for Pre<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(TagRenderableName, GenericRenderable, TagRenderable)]
#[tag_renderable_name(name = "img")]
pub struct Img<'a> {
    pub src: attributes::Src,
    pub styles: attributes::StyleAttr<'a>,
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

#[derive(TagRenderableName, GenericRenderable, TagRenderable, TagRenderableChildren)]
#[tag_renderable_name(name = "a")]
pub struct A<'a> {
    pub href: attributes::Href,
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> TagRenderableAttrs for A<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![&self.href], &self.styles)
    }
}

#[derive(TagRenderableName, GenericRenderable, TagRenderable)]
#[tag_renderable_name(name = "form")]
pub struct Form<'a> {
    pub formmethod: attributes::Formmethod,
    pub action: Option<attributes::Action>,
    pub inputs: Vec<Input<'a>>,
    pub button: Button<'a>,
    pub styles: attributes::StyleAttr<'a>,
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

#[derive(TagRenderableName, GenericRenderable, TagRenderable)]
#[tag_renderable_name(name = "button")]
pub struct Button<'a> {
    pub child: Element,
    pub styles: attributes::StyleAttr<'a>,
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

#[derive(TagRenderableName, GenericRenderable, TagRenderable)]
#[tag_renderable_name(name = "input")]
pub struct Input<'a> {
    pub type_: Option<attributes::InputType>,
    pub name: attributes::Name,
    pub value: attributes::Value,
    pub styles: attributes::StyleAttr<'a>,
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
