use super::attributes;
use super::style_sheet;
use super::units;
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

#[derive(TagRenderable, TagRenderableName, TagRenderableStyleSetter)]
#[tag_renderable_name(name = "html")]
pub struct Html<'a> {
    pub head: Option<Head<'a>>,
    pub body: Option<Body<'a>>,
    pub lang: attributes::Lang,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> Html<'a> {
    pub fn style_less(
        head: Option<Head<'a>>,
        body: Option<Body<'a>>,
        lang: attributes::Lang,
    ) -> Self {
        Html {
            head: head,
            body: body,
            lang: lang,
            styles: attributes::StyleAttr::empty(),
        }
    }
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

impl<'a> Head<'a> {
    pub fn new(metas: Vec<Meta<'a>>, styles: Vec<Style<'a>>) -> Self {
        Head {
            metas: metas,
            styles: styles,
        }
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

#[derive(TagRenderable, TagRenderableName, TagRenderableStyleSetter)]
#[tag_renderable_name(name = "meta")]
pub struct Meta<'a> {
    pub charset: Option<attributes::Charset>,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> Meta<'a> {
    pub fn style_less(charset: Option<attributes::Charset>) -> Self {
        Meta {
            charset: charset,
            styles: attributes::StyleAttr::empty(),
        }
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

#[derive(TagRenderableName, TagRenderable, TagRenderableChildren, TagRenderableStyleSetter)]
#[tag_renderable_name(name = "body")]
pub struct Body<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> Body<'a> {
    pub fn style_less(children: Vec<Element>) -> Self {
        Body {
            children: children,
            styles: attributes::StyleAttr::empty(),
        }
    }
}

impl<'a> TagRenderableAttrs for Body<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(
    TagRenderableName,
    GenericRenderable,
    TagRenderable,
    TagRenderableChildren,
    TagRenderableStyleSetter,
    TagRenderableIntoElement,
)]
#[tag_renderable_name(name = "div")]
pub struct Div<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> Div<'a> {
    pub fn style_less(children: Vec<Element>) -> Self {
        Div {
            children: children,
            styles: attributes::StyleAttr::empty(),
        }
    }
}

impl<'a> TagRenderableAttrs for Div<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(
    TagRenderableName,
    GenericRenderable,
    TagRenderable,
    TagRenderableChildren,
    TagRenderableStyleSetter,
    TagRenderableIntoElement,
)]
#[tag_renderable_name(name = "span")]
pub struct Span<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> Span<'a> {
    pub fn style_less(children: Vec<Element>) -> Self {
        Span {
            children: children,
            styles: attributes::StyleAttr::empty(),
        }
    }
}

impl<'a> TagRenderableAttrs for Span<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(
    TagRenderableName,
    GenericRenderable,
    TagRenderable,
    TagRenderableChildren,
    TagRenderableStyleSetter,
    TagRenderableIntoElement,
)]
#[tag_renderable_name(name = "h1")]
pub struct H1<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> H1<'a> {
    pub fn style_less(children: Vec<Element>) -> Self {
        H1 {
            children: children,
            styles: attributes::StyleAttr::empty(),
        }
    }
}

impl<'a> TagRenderableAttrs for H1<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(
    TagRenderableName,
    GenericRenderable,
    TagRenderable,
    TagRenderableChildren,
    TagRenderableStyleSetter,
    TagRenderableIntoElement,
)]
#[tag_renderable_name(name = "h2")]
pub struct H2<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> H2<'a> {
    pub fn style_less(children: Vec<Element>) -> Self {
        H2 {
            children: children,
            styles: attributes::StyleAttr::empty(),
        }
    }
}

impl<'a> TagRenderableAttrs for H2<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(
    TagRenderableName,
    GenericRenderable,
    TagRenderable,
    TagRenderableChildren,
    TagRenderableStyleSetter,
    TagRenderableIntoElement,
)]
#[tag_renderable_name(name = "h3")]
pub struct H3<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> H3<'a> {
    pub fn style_less(children: Vec<Element>) -> Self {
        H3 {
            children: children,
            styles: attributes::StyleAttr::empty(),
        }
    }
}

impl<'a> TagRenderableAttrs for H3<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(
    TagRenderableName,
    GenericRenderable,
    TagRenderable,
    TagRenderableChildren,
    TagRenderableStyleSetter,
    TagRenderableIntoElement,
)]
#[tag_renderable_name(name = "h4")]
pub struct H4<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> H4<'a> {
    pub fn style_less(children: Vec<Element>) -> Self {
        H4 {
            children: children,
            styles: attributes::StyleAttr::empty(),
        }
    }
}

impl<'a> TagRenderableAttrs for H4<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(
    TagRenderableName,
    GenericRenderable,
    TagRenderable,
    TagRenderableChildren,
    TagRenderableStyleSetter,
    TagRenderableIntoElement,
)]
#[tag_renderable_name(name = "p")]
pub struct P<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> P<'a> {
    pub fn style_less(children: Vec<Element>) -> Self {
        P {
            children: children,
            styles: attributes::StyleAttr::empty(),
        }
    }
}

impl<'a> TagRenderableAttrs for P<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(
    TagRenderableName,
    GenericRenderable,
    TagRenderable,
    TagRenderableStyleSetter,
    TagRenderableIntoElement,
)]
#[tag_renderable_name(name = "table")]
pub struct Table<'a> {
    pub thead: Option<Thead<'a>>,
    pub tbody: Tbody<'a>,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> Table<'a> {
    pub fn style_less(thead: Option<Thead<'a>>, tbody: Tbody<'a>) -> Self {
        Table {
            thead: thead,
            tbody: tbody,
            styles: attributes::StyleAttr::empty(),
        }
    }

    pub fn style_less_from_vecs(
        thead: Option<Vec<Vec<Element>>>,
        tbody: Vec<Vec<Element>>,
    ) -> Self {
        Table {
            thead: match thead {
                Some(h) => Some(Thead::style_less(
                    h.into_iter()
                        .map(|row| {
                            Thr::style_less(
                                row.into_iter()
                                    .map(|data| Th::style_less(vec![data]))
                                    .collect(),
                            )
                        })
                        .collect(),
                )),
                None => None,
            },
            tbody: Tbody::style_less(
                tbody
                    .into_iter()
                    .map(|row| {
                        Tr::style_less(
                            row.into_iter()
                                .map(|data| Td::style_less(vec![data]))
                                .collect(),
                        )
                    })
                    .collect(),
            ),
            styles: attributes::StyleAttr::empty(),
        }
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

#[derive(TagRenderableName, TagRenderable, TagRenderableChildren, TagRenderableStyleSetter)]
#[tag_renderable_name(name = "thead")]
pub struct Thead<'a> {
    #[tag_renderable_children(type = "renderable")]
    pub trs: Vec<Thr<'a>>,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> Thead<'a> {
    pub fn style_less(trs: Vec<Thr<'a>>) -> Self {
        Thead {
            trs: trs,
            styles: attributes::StyleAttr::empty(),
        }
    }
}

impl<'a> TagRenderableAttrs for Thead<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(TagRenderableName, TagRenderable, TagRenderableChildren, TagRenderableStyleSetter)]
#[tag_renderable_name(name = "tr")]
pub struct Thr<'a> {
    #[tag_renderable_children(type = "renderable")]
    pub ths: Vec<Th<'a>>,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> Thr<'a> {
    pub fn style_less(ths: Vec<Th<'a>>) -> Self {
        Thr {
            ths: ths,
            styles: attributes::StyleAttr::empty(),
        }
    }
}

impl<'a> TagRenderableAttrs for Thr<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(TagRenderableName, TagRenderable, TagRenderableChildren, TagRenderableStyleSetter)]
#[tag_renderable_name(name = "th")]
pub struct Th<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> Th<'a> {
    pub fn style_less(children: Vec<Element>) -> Self {
        Th {
            children: children,
            styles: attributes::StyleAttr::empty(),
        }
    }
}

impl<'a> TagRenderableAttrs for Th<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(TagRenderableName, TagRenderable, TagRenderableChildren, TagRenderableStyleSetter)]
#[tag_renderable_name(name = "tbody")]
pub struct Tbody<'a> {
    #[tag_renderable_children(type = "renderable")]
    pub trs: Vec<Tr<'a>>,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> Tbody<'a> {
    pub fn style_less(trs: Vec<Tr<'a>>) -> Self {
        Tbody {
            trs: trs,
            styles: attributes::StyleAttr::empty(),
        }
    }
}

impl<'a> TagRenderableAttrs for Tbody<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(TagRenderableName, TagRenderable, TagRenderableChildren, TagRenderableStyleSetter)]
#[tag_renderable_name(name = "tr")]
pub struct Tr<'a> {
    #[tag_renderable_children(type = "renderable")]
    pub tds: Vec<Td<'a>>,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> Tr<'a> {
    pub fn style_less(tds: Vec<Td<'a>>) -> Self {
        Tr {
            tds: tds,
            styles: attributes::StyleAttr::empty(),
        }
    }
}

impl<'a> TagRenderableAttrs for Tr<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(TagRenderableName, TagRenderable, TagRenderableChildren, TagRenderableStyleSetter)]
#[tag_renderable_name(name = "td")]
pub struct Td<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> Td<'a> {
    pub fn style_less(children: Vec<Element>) -> Self {
        Td {
            children: children,
            styles: attributes::StyleAttr::empty(),
        }
    }
}

impl<'a> TagRenderableAttrs for Td<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(
    TagRenderableName,
    GenericRenderable,
    TagRenderable,
    TagRenderableChildren,
    TagRenderableStyleSetter,
    TagRenderableIntoElement,
)]
#[tag_renderable_name(name = "code")]
pub struct Code<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> Code<'a> {
    pub fn style_less(children: Vec<Element>) -> Self {
        Code {
            children: children,
            styles: attributes::StyleAttr::empty(),
        }
    }
}

impl<'a> TagRenderableAttrs for Code<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(
    TagRenderableName,
    GenericRenderable,
    TagRenderable,
    TagRenderableChildren,
    TagRenderableStyleSetter,
    TagRenderableIntoElement,
)]
#[tag_renderable_name(name = "pre")]
pub struct Pre<'a> {
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> Pre<'a> {
    pub fn style_less(children: Vec<Element>) -> Self {
        Pre {
            children: children,
            styles: attributes::StyleAttr::empty(),
        }
    }
}

impl<'a> TagRenderableAttrs for Pre<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![], &self.styles)
    }
}

#[derive(
    TagRenderableName,
    GenericRenderable,
    TagRenderable,
    TagRenderableStyleSetter,
    TagRenderableIntoElement,
)]
#[tag_renderable_name(name = "img")]
pub struct Img<'a> {
    pub src: attributes::Src,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> Img<'a> {
    pub fn style_less(src: attributes::Src) -> Self {
        Img {
            src: src,
            styles: attributes::StyleAttr::empty(),
        }
    }

    pub fn style_less_with_src(src: String) -> Self {
        Img {
            src: attributes::Src {
                value: units::SourceValue::new(src),
            },
            styles: attributes::StyleAttr::empty(),
        }
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

#[derive(
    TagRenderableName,
    GenericRenderable,
    TagRenderable,
    TagRenderableChildren,
    TagRenderableStyleSetter,
    TagRenderableIntoElement,
)]
#[tag_renderable_name(name = "a")]
pub struct A<'a> {
    pub href: attributes::Href,
    #[tag_renderable_children(type = "element")]
    pub children: Vec<Element>,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> A<'a> {
    pub fn style_less(href: attributes::Href, children: Vec<Element>) -> Self {
        A {
            href: href,
            children: children,
            styles: attributes::StyleAttr::empty(),
        }
    }
}

impl<'a> TagRenderableAttrs for A<'a> {
    fn get_attributes(&self) -> Vec<&dyn Attribute> {
        util::full_attrs(vec![&self.href], &self.styles)
    }
}

#[derive(
    TagRenderableName,
    GenericRenderable,
    TagRenderable,
    TagRenderableStyleSetter,
    TagRenderableIntoElement,
)]
#[tag_renderable_name(name = "form")]
pub struct Form<'a> {
    pub formmethod: attributes::Formmethod,
    pub action: Option<attributes::Action>,
    pub inputs: Vec<Input<'a>>,
    pub button: Button<'a>,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> Form<'a> {
    pub fn style_less(
        formmethod: attributes::Formmethod,
        action: Option<attributes::Action>,
        inputs: Vec<Input<'a>>,
        button: Button<'a>,
    ) -> Self {
        Form {
            formmethod: formmethod,
            action: action,
            inputs: inputs,
            button: button,
            styles: attributes::StyleAttr::empty(),
        }
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

#[derive(
    TagRenderableName,
    GenericRenderable,
    TagRenderable,
    TagRenderableStyleSetter,
    TagRenderableIntoElement,
)]
#[tag_renderable_name(name = "button")]
pub struct Button<'a> {
    pub child: Element,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> Button<'a> {
    pub fn style_less(child: Element) -> Self {
        Button {
            child: child,
            styles: attributes::StyleAttr::empty(),
        }
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

#[derive(
    TagRenderableName,
    GenericRenderable,
    TagRenderable,
    TagRenderableStyleSetter,
    TagRenderableIntoElement,
)]
#[tag_renderable_name(name = "input")]
pub struct Input<'a> {
    pub type_: Option<attributes::InputType>,
    pub name: attributes::Name,
    pub value: attributes::Value,
    #[tag_renderable_style]
    pub styles: attributes::StyleAttr<'a>,
}

impl<'a> Input<'a> {
    pub fn style_less(
        type_: Option<attributes::InputType>,
        name: attributes::Name,
        value: attributes::Value,
    ) -> Self {
        Input {
            type_: type_,
            name: name,
            value: value,
            styles: attributes::StyleAttr::empty(),
        }
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
