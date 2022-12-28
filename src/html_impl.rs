use super::html::ToHtml;
use super::parser::*;

impl ToHtml for ParseArtefact {
    fn to_html(&self) -> String {
        match self {
            ParseArtefact::Section(depth, title) => {
                return format!("<h1>{depth} {title}</h1>");
            }
            ParseArtefact::Subsection(section_depth, depth, title) => {
                return format!("<h2>{section_depth}.{depth} {title}</h2>");
            }
            ParseArtefact::Subsubsection(section_depth, subsection_depth, depth, title) => {
                return format!("<h3>{section_depth}.{subsection_depth}.{depth} {title}</h3>");
            }
            ParseArtefact::Paragraph(text) => {
                let mut paragraph_contents = String::new();
                for artefact in text {
                    paragraph_contents.push_str(&artefact.to_html());
                }
                return format!("<p>{}</p>", paragraph_contents);
            }
            ParseArtefact::List(list) => {
                let mut list_contents = String::new();
                for artefact in list {
                    list_contents.push_str(&artefact.to_html());
                }
                return format!("<ol>{}</ol>", list_contents);
            }
            ParseArtefact::ListItem(text) => {
                let list_item_contents = text.to_html();
                return format!("<li>{}</li>", list_item_contents);
            }
            ParseArtefact::Quote(text) => {
                let quote_contents = text.to_html();
                return format!("<blockquote>{}</blockquote>", quote_contents);
            }
            ParseArtefact::Table(table_rows) => {
                if let Some(ParseArtefact::TableRow(first_row_contents)) = table_rows.get(0) {
                    let mut header_data = vec![];
                    for artefact in first_row_contents {
                        header_data.push(artefact.to_html());
                    }
                    let table_header = format!(
                        "<thead><tr>{}</tr><thead>",
                        header_data
                            .iter()
                            .map(|header| format!("<th>{}</th>", header))
                            .collect::<Vec<String>>()
                            .join("")
                    );
                    let mut table_body = String::new();
                    for row in table_rows[1..].iter() {
                        table_body.push_str(&row.to_html());
                    }
                    return format!("<table>{table_header}<tbody>{table_body}</tbody></table>");
                } else {
                    return String::new();
                }
            }
            ParseArtefact::TableRow(row_contents) => {
                let mut row_data = vec![];
                for artefact in row_contents {
                    row_data.push(artefact.to_html());
                }
                return format!(
                    "<tr>{}</tr>",
                    row_data
                        .iter()
                        .map(|header| format!("<td>{}</td>", header))
                        .collect::<Vec<String>>()
                        .join("")
                );
            }
            ParseArtefact::Button(text, link) => {
                return format!("<a href=\"{link}\" class=\"md-button\">{text}</a>");
            }
            ParseArtefact::Image(caption, link) => {
                return format!(
                    "<img src=\"{link}\" alt=\"{caption}\" /><p class=\"img-caption\">{caption}</p>"
                );
            }
            ParseArtefact::Codeblock(code) => {
                return format!("<pre><code>{code}</code></pre>");
            }
            _ => {
                panic!("Unkown artefact type");
            }
        }
    }
}

impl<'a> ToHtml for &'a ParseArtefact {
    fn to_html(&self) -> String {
        match self {
            ParseArtefact::Section(depth, title) => {
                return format!("<h1>{depth} {title}</h1>");
            }
            ParseArtefact::Subsection(section_depth, depth, title) => {
                return format!("<h2>{section_depth}.{depth} {title}</h2>");
            }
            ParseArtefact::Subsubsection(section_depth, subsection_depth, depth, title) => {
                return format!("<h3>{section_depth}.{subsection_depth}.{depth} {title}</h3>");
            }
            ParseArtefact::Paragraph(text) => {
                let mut paragraph_contents = String::new();
                for artefact in text {
                    paragraph_contents.push_str(&artefact.to_html());
                }
                return format!("<p>{}</p>", paragraph_contents);
            }
            ParseArtefact::List(list) => {
                let mut list_contents = String::new();
                for artefact in list {
                    list_contents.push_str(&artefact.to_html());
                }
                return format!("<ol>{}</ol>", list_contents);
            }
            ParseArtefact::ListItem(text) => {
                let list_item_contents = text.to_html();
                return format!("<li>{}</li>", list_item_contents);
            }
            ParseArtefact::Quote(text) => {
                let quote_contents = text.to_html();
                return format!("<blockquote>{}</blockquote>", quote_contents);
            }
            ParseArtefact::Table(table_rows) => {
                if let Some(ParseArtefact::TableRow(first_row_contents)) = table_rows.get(0) {
                    let mut header_data = vec![];
                    for artefact in first_row_contents {
                        header_data.push(artefact.to_html());
                    }
                    let table_header = format!(
                        "<thead><tr>{}</tr><thead>",
                        header_data
                            .iter()
                            .map(|header| format!("<th>{}</th>", header))
                            .collect::<Vec<String>>()
                            .join("")
                    );
                    let mut table_body = String::new();
                    for row in table_rows[1..].iter() {
                        table_body.push_str(&row.to_html());
                    }
                    return format!(
                        "<table>{table_header}<tbody>{table_body}</tbody></table>",
                        table_header = table_header,
                        table_body = table_body
                    );
                } else {
                    return String::new();
                }
            }
            ParseArtefact::TableRow(row_contents) => {
                let mut row_data = vec![];
                for artefact in row_contents {
                    row_data.push(artefact.to_html());
                }
                return format!(
                    "<tr>{}</tr>",
                    row_data
                        .iter()
                        .map(|header| format!("<td>{}</td>", header))
                        .collect::<Vec<String>>()
                        .join("")
                );
            }
            ParseArtefact::Button(text, link) => {
                return format!("<a href=\"{link}\" class=\"md-button\">{text}</a>");
            }
            ParseArtefact::Image(caption, link) => {
                return format!(
                    "<img src=\"{link}\" alt=\"{caption}\" /><p class=\"img-caption\">{caption}</p>"
                );
            }
            ParseArtefact::Codeblock(code) => {
                return format!("<pre><code>{code}</code></pre>");
            }
            _ => {
                panic!("Unkown artefact type");
            }
        }
    }
}

impl ToHtml for Text {
    fn to_html(&self) -> String {
        let Text(contents) = self;
        contents
            .iter()
            .map(|text| text.to_html())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl<'a> ToHtml for &'a Text {
    fn to_html(&self) -> String {
        let Text(contents) = self;
        return contents
            .iter()
            .map(|text| text.to_html())
            .collect::<Vec<String>>()
            .join(" ");
    }
}

impl ToHtml for TextArtefact {
    fn to_html(&self) -> String {
        match self {
            TextArtefact::Bold(text) => {
                return format!("<strong>{}</strong>", text);
            }
            TextArtefact::Italics(text) => {
                return format!("<em>{}</em>", text);
            }
            TextArtefact::Strikethrough(text) => {
                return format!("<del>{}</del>", text);
            }
            TextArtefact::Underline(text) => {
                return format!("<u>{}</u>", text);
            }
            TextArtefact::Code(text) => {
                return format!("<code>{}</code>", text);
            }
            TextArtefact::Link(text, link) => {
                return format!("<a href=\"{link}\">{text}</a>");
            }
            TextArtefact::Raw(text) => {
                return text.to_string();
            }
            _ => {
				panic!("Unkown artefact type");
			}
		}
    }
}

impl<'a> ToHtml for &'a TextArtefact {
    fn to_html(&self) -> String {
        match self {
            TextArtefact::Bold(text) => {
                return format!("<strong>{}</strong>", text);
            }
            TextArtefact::Italics(text) => {
                return format!("<em>{}</em>", text);
            }
            TextArtefact::Strikethrough(text) => {
                return format!("<del>{}</del>", text);
            }
            TextArtefact::Underline(text) => {
                return format!("<u>{}</u>", text);
            }
            TextArtefact::Code(text) => {
                return format!("<code>{}</code>", text);
            }
            TextArtefact::Link(text, link) => {
                return format!("<a href=\"{link}\">{text}</a>");
            }
            TextArtefact::Raw(text) => {
                return text.to_string();
            }
            _ => { panic!("Unkown artefact type"); }
        }
    }
}