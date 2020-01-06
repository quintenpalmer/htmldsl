use super::traits::style_traits;

pub struct StyleAssignment<'a> {
    pub name: String,
    pub styles: Vec<&'a dyn style_traits::Style>,
}

pub fn style_sheet_string(style_sheet: &Vec<StyleAssignment>) -> String {
    style_sheet.iter().fold("".into(), |compiled, curr| {
        format!("{} {}", compiled, single_style_string(curr))
    })
}

fn single_style_string(style_assignment: &StyleAssignment) -> String {
    format!(
        "{} {{ {} }}",
        style_assignment.name,
        style_traits::render_styles(&style_assignment.styles)
    )
}
