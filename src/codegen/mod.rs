mod tests;
use super::parser::{ self, * };
use std::error::Error;

pub struct Codegen {}

impl Codegen {
    pub fn new() -> Self {
        Self {}
    }

    fn text_gen(&self, text_node: Text) -> Result<String, CodegenError> {
        let mut output = String::new();
        if let Text(contents) = text_node {
            for node in contents {
                match node {
                    TextElement::Raw(text) => output.push_str(&text),
                    TextElement::Bold(contents) =>
                        output.push_str(&format!(" <strong>{}</strong> ", contents)),
                    TextElement::Italics(contents) =>
                        output.push_str(&format!(" <em>{}</em> ", contents)),
                    TextElement::Code(contents) =>
                        output.push_str(&format!(" <code>{}</code> ", contents)),
                    TextElement::Strikethrough(contents) =>
                        output.push_str(&format!(" <del>{}</del> ", contents)),
                    _ => panic!("Invalid text node"),
                }
            }
        }
        Ok(output)
    }

    fn section_gen(&self, section_node: Element) -> Result<String, CodegenError> {
        let mut output = String::new();
        if let Element::Section(text_content) = section_node {
            output.push_str(&format!("<h1>{}</h1>", self.text_gen(text_content)?));
        } else {
            return Err(CodegenError::InvalidParseNode("Expected section node".to_string()));
        }
        Ok(output)
    }

    fn subsection_gen(&self, subsection_node: Element) -> Result<String, CodegenError> {
        let mut output = String::new();
        if let Element::Subsection(text_content) = subsection_node {
            output.push_str(&format!("<h2>{}</h2>", self.text_gen(text_content)?));
        } else {
            return Err(CodegenError::InvalidParseNode("Expected subsection node".to_string()));
        }
        Ok(output)
    }

    fn subsubsection_gen(&self, subsubsection_node: Element) -> Result<String, CodegenError> {
        let mut output = String::new();
        if let Element::Subsubsection(text_content) = subsubsection_node {
            output.push_str(&format!("<h3>{}</h3>", self.text_gen(text_content)?));
        } else {
            return Err(CodegenError::InvalidParseNode("Expected subsubsection node".to_string()));
        }
        Ok(output)
    }

    fn list_gen(&self, list_node: Element) -> Result<String, CodegenError> {
        let mut output = String::new();
        if let Element::List(list_contents) = list_node {
            output.push_str("<ul>");
            for item in list_contents {
                match item {
                    Element::ListItem(text) =>
                        output.push_str(&format!("<li>{}</li>", self.text_gen(text)?)),
                    Element::List(list_contents) =>
                        output.push_str(&self.list_gen(Element::List(list_contents))?),
                    _ => {
                        return Err(
                            CodegenError::InvalidParseNode("Expected list item".to_string())
                        );
                    }
                }
            }
            output.push_str("</ul>");
        } else {
            return Err(CodegenError::InvalidParseNode("Expected list node".to_string()));
        }
        Ok(output)
    }

    fn paragraph_gen(&self, paragraph_node: Element) -> Result<String, CodegenError> {
        let mut output = String::new();
        if let Element::Paragraph(text_content) = paragraph_node {
            output.push_str(&format!("<p>{}</p>", self.text_gen(text_content)?));
        } else {
            return Err(CodegenError::InvalidParseNode("Expected paragraph node".to_string()));
        }
        Ok(output)
    }

    fn quote_gen(&self, quote_node: Element) -> Result<String, CodegenError> {
        let mut output = String::new();
        if let Element::Quote(text_content) = quote_node {
            output.push_str(&format!("<blockquote>{}</blockquote>", self.text_gen(text_content)?));
        } else {
            return Err(CodegenError::InvalidParseNode("Expected quote node".to_string()));
        }
        Ok(output)
    }

    fn element_gen(&self, element_node: Element) -> Result<String, CodegenError> {
        let mut output = String::new();
        match element_node {
            Element::Section(text_content) =>
                output.push_str(&self.section_gen(Element::Section(text_content))?),
            Element::Subsection(text_content) =>
                output.push_str(&self.subsection_gen(Element::Subsection(text_content))?),
            Element::Subsubsection(text_content) =>
                output.push_str(&self.subsubsection_gen(Element::Subsubsection(text_content))?),
            Element::List(list_contents) =>
                output.push_str(&self.list_gen(Element::List(list_contents))?),
            Element::Paragraph(text_content) =>
                output.push_str(&self.paragraph_gen(Element::Paragraph(text_content))?),
            Element::Quote(text_content) =>
                output.push_str(&self.quote_gen(Element::Quote(text_content))?),
            _ => {
                return Err(CodegenError::InvalidParseNode("Invalid element node".to_string()));
            }
        }
        Ok(output)
    }

	fn get_title(&self, document: &Vec<Element>) -> String {
		let mut title = String::new();
		for element in document {
			match element {
				Element::Section(text_content) => {
					title = self.text_gen(text_content.clone()).unwrap();
					break;
				},
				_ => { continue; }
			}
		}
		title
	}

    fn document_gen(&self, document: Vec<Element>, stylesheet: String) -> Result<String, CodegenError> {
        let mut output = String::new();
		let title = self.get_title(&document);

        output.push_str(
			&format!(
				"<!DOCTYPE html><html><head><title>{}</title><style>{}</style></head><body>",
				title,
				stylesheet
			)
        );
        for element in document {
            output.push_str(&self.element_gen(element)?);
        }
        output.push_str("</body></html>");
        Ok(output)
    }

    pub fn generate(&self, document: Vec<Element>, stylesheet: String) -> Result<String, CodegenError> {
        self.document_gen(document, stylesheet)
    }
}

#[derive(Debug)]
pub enum CodegenError {
    InvalidParseNode(String),
}

impl std::fmt::Display for CodegenError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			CodegenError::InvalidParseNode(node) =>
				write!(f, "Invalid parse node: {}", node),
		}
	}
}

impl Error for CodegenError {}
