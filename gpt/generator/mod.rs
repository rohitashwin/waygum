use super::parser::Element;

pub fn to_html(element: &Element) -> String {
    match element {
        Element::Heading(text) => format!("<h1>{}</h1>", text),
        Element::List(items) => {
            let mut html = String::new();
            html.push_str("<ul>");
            for item in items {
                html.push_str(&format!("<li>{}</li>", to_html(item)));
            }
            html.push_str("</ul>");
            html
        }
        Element::ListItem(text) => text.clone(),
        Element::Quote(text) => format!("<blockquote>{}</blockquote>", text),
        Element::Bold(text) => format!("<strong>{}</strong>", text),
        Element::Italic(text) => format!("<em>{}</em>", text),
        Element::Code(text) => format!("<code>{}</code>", text),
        Element::Strikethrough(text) => format!("<del>{}</del>", text),
        Element::Text(text) => text.clone(),
    }
}
