# htmldsl

## Description 

This is a library to help with programatically generating HTML.

## Anatomy of the project:

htmldsl/
    lib.rs - library top level file, exports all modules in crate and all contents of the internal crate
    elements.rs - Contains common HTML elements (like `<h1>` and `<p>`)
    attributes.rs - Contains attributes to assign in elements (like `src="..."` and `href="..."`)
    styles.rs - Contains styles to be built into the style attribute (like `margin: "..."`)
    units.rs - Contains units to be specified by attributes or styles (like `0px` or `utf-8`)
    style_sheet.rs - Contains the structure for a style sheet, which can be set on the `<head>`
htmldsl_internal/
    lib.rs - Contains the traits that the previous crates builds on
htmlds_internal_derive/
    *.rs - Derive macros for various traits defined in htmldsl_internal

## Example usage

Example main leveraging this library:

```
use htmldsl::attributes;
use htmldsl::elements;
use htmldsl::style_sheet;
use htmldsl::styles;
use htmldsl::units;
use htmldsl::TagRenderableIntoElement;

fn main() {
    let html = elements::Html::style_less(
        Some(elements::Head::new(
            vec![elements::Meta::style_less(Some(attributes::Charset {
                value: units::CharsetValue::Utf8,
            }))],
            vec![elements::Style {
                style_sheet: style_sheet::StyleSheet {
                    assignments: vec![style_sheet::StyleAssignment {
                        names: vec!["img".into()],
                        styles: vec![&styles::Border {
                            style: units::BorderStyle::None,
                        }],
                    }],
                },
            }],
        )),
        Some(elements::Body::style_less(vec![
            elements::H1::style_less(vec![htmldsl::text("Welcome!".into())]).into_element(),
            elements::P::style_less(vec![htmldsl::text("Glad you could join us.".into())])
                .into_element(),
        ])),
        attributes::Lang {
            tag: units::LanguageTag::En,
            sub_tag: units::LanguageSubTag::Us,
        },
    );

    println!("{}", htmldsl::render_simple_html_page(true, html));
}
```

Which will output:

```
<!DOCTYPE html>
<html lanq="en-US">
	<head>
		<meta charset="utf-8"></meta>
		<style> img { border: none;  }</style>
	</head>
	<body>
		<h1>Welcome!</h1>
		<p>Glad you could join us.</p>
	</body>
</html>
```
