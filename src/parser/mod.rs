use super::lexer::{ self, * };
use std::{ error::Error, vec, rc::Rc };
mod tests;

pub struct Parser {
    tokens: Vec<lexer::Token>,
    current_list_depth: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().rev().collect(),
            current_list_depth: 1,
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.last()
    }

    pub fn next(&mut self) -> Option<Token> {
        self.tokens.pop()
    }

    pub fn parse_next(&mut self) -> Result<Option<Element>, ParseError> {
        match self.peek() {
            Some(Token::Section) => self.parse_section(),
            Some(Token::Subsection) => self.parse_subsection(),
            Some(Token::Subsubsection) => self.parse_subsubsection(),
            Some(Token::List(list_depth)) => {
                self.current_list_depth = 1;
                self.parse_list()
            },
			Some(Token::Quote) => self.parse_quote(),
			Some(Token::Bold) => self.parse_bold(),
			Some(Token::Italics) => self.parse_italics(),
			Some(Token::Strikethrough) => self.parse_strikethrough(),
			Some(Token::Code) => self.parse_code(),
			Some(Token::Text(_)) => self.parse_text(),
            Some(Token::EOF) => Ok(None),
            _ => Ok(None),
            None => Err(ParseError::UnexpectedToken("Unkown token encountered".to_string())),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Element>, ParseError> {
        let mut elements: Vec<Element> = vec![];
        while let Some(element) = self.parse_next()? {
            elements.push(element);
        }
        Ok(elements)
    }

    pub fn parse_section(&mut self) -> Result<Option<Element>, ParseError> {
        let section_token = self.next().unwrap();
        let section_contents = match self.peek() {
			Some(Token::Text(_) | Token::Bold | Token::Italics | Token::Strikethrough | Token::Code) => {
				self.parse_text()?
			}
		};
        Ok(Some(Element::Section(section_contents)))
    }

    pub fn parse_subsection(&mut self) -> Result<Option<Element>, ParseError> {
        let subsection_token = self.next().unwrap();
        let subsection_contents = match self.next() {
            Some(Token::Text(str_contents)) => str_contents,
            Some(token) => {
                return Err(
                    ParseError::UnexpectedToken(
                        format!("'Text' token expected, but found {:?}", token)
                    )
                );
            }
            None => String::new(),
        };
        Ok(Some(Element::Subsection(subsection_contents)))
    }

    pub fn parse_subsubsection(&mut self) -> Result<Option<Element>, ParseError> {
        let subsubsection_token = self.next().unwrap();
        let subsubsection_contents = match self.next() {
            Some(Token::Text(str_contents)) => str_contents,
            Some(token) => {
                return Err(
                    ParseError::UnexpectedToken(
                        format!("'Text' token expected, but found {:?}", token)
                    )
                );
            }
            None => String::new(),
        };
        Ok(Some(Element::Subsubsection(subsubsection_contents)))
    }

    pub fn parse_list(&mut self) -> Result<Option<Element>, ParseError> {
        dbg!("Parse List Called");
        let mut list_items = vec![];
        loop {
            let next_item_ref = match self.peek() {
                Some(token) => token,
                None => {
                    return Ok(None);
                }
            };
            match next_item_ref {
                Token::List(ListDepth(depth)) => {
                    dbg!("Parsing List Depth: {:?}", depth);
                    if *depth < self.current_list_depth {
                        self.current_list_depth -= 1;
                        return Ok(Some(Element::List(list_items)));
                    } else if *depth > self.current_list_depth {
                        self.current_list_depth += 1;
                        list_items.push(
                            self
                                .parse_list()?
                                .ok_or(
                                    ParseError::UnexpectedToken(
                                        format!("Unexpected List Parse Error")
                                    )
                                )?
                        );
                    } else {
                        list_items.extend(self.parse_list_items()?);
                    }
                }
                _ => {
                    return Ok(Some(Element::List(list_items)));
                }
            }
        }
    }

    pub fn parse_list_items(&mut self) -> Result<Vec<Element>, ParseError> {
        dbg!("Parse List Items Called");
        let mut contents = vec![];
        loop {
            match self.peek() {
                Some(Token::List(ListDepth(depth))) => {
                    if *depth == self.current_list_depth {
                        let list_marker = self.next();
                        let list_content = self.parse_text()?.ok_or(
							ParseError::UnexpectedToken(
								format!("Unexpected List Parse Error")
							)
						)?;
                        contents.push(Element::ListItem(vec![list_content]));
                    } else {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
        }
        dbg!(&contents);
        Ok(contents)
    }

    pub fn parse_quote(&mut self) -> Result<Option<Element>, ParseError> {
        let quote_token = self.next().unwrap();
        let quote_contents = match self.next() {
            Some(Token::Text(str_contents)) => str_contents,
            Some(token) => {
                return Err(
                    ParseError::UnexpectedToken(
                        format!("'Text' token expected, but found {:?}", token)
                    )
                );
            }
            None => String::new(),
        };
        Ok(Some(Element::Quote(quote_contents)))
    }

    pub fn parse_bold(&mut self) -> Result<Option<Element>, ParseError> {
        let bold_token = self.next().unwrap();
        let bold_contents = match self.next() {
            Some(Token::Text(str_contents)) => str_contents,
            Some(token) => {
                return Err(
                    ParseError::UnexpectedToken(
                        format!("'Text' token expected, but found {:?}", token)
                    )
                );
            }
            None => String::new(),
        };
        Ok(Some(Element::Bold(bold_contents)))
    }

    pub fn parse_italics(&mut self) -> Result<Option<Element>, ParseError> {
        let italics_token = self.next().unwrap();
        let italics_contents = match self.next() {
            Some(Token::Text(str_contents)) => str_contents,
            Some(token) => {
                return Err(
                    ParseError::UnexpectedToken(
                        format!("'Text' token expected, but found {:?}", token)
                    )
                );
            }
            None => String::new(),
        };
        Ok(Some(Element::Italics(italics_contents)))
    }

    pub fn parse_code(&mut self) -> Result<Option<Element>, ParseError> {
        let code_token = self.next().unwrap();
        let code_contents = match self.next() {
            Some(Token::Text(str_contents)) => str_contents,
            Some(token) => {
                return Err(
                    ParseError::UnexpectedToken(
                        format!("'Text' token expected, but found {:?}", token)
                    )
                );
            }
            None => String::new(),
        };
        Ok(Some(Element::Code(code_contents)))
    }

    pub fn parse_strikethrough(&mut self) -> Result<Option<Element>, ParseError> {
        let strikethrough_token = self.next().unwrap();
        let strikethrough_contents = match self.next() {
            Some(Token::Text(str_contents)) => str_contents,
            Some(token) => {
                return Err(
                    ParseError::UnexpectedToken(
                        format!("'Text' token expected, but found {:?}", token)
                    )
                );
            }
            None => String::new(),
        };
        Ok(Some(Element::Strikethrough(strikethrough_contents)))
    }

    pub fn parse_text(&mut self) -> Result<Option<TextElement>, ParseError> {
		let mut text_contents = vec![];
		loop {
			match self.peek() {
				Some(Token::Text(_)) => {
					let text_token = self.next().unwrap();
					if let Token::Text(str_contents) = text_token {
						text_contents.push(Element::PureText(str_contents));
					}
				},
				Some(Token::Bold) => {
					if let Some(bold_element) = self.parse_bold()? {
						text_contents.push(bold_element);
					}
				},
				Some(Token::Italics) => {
					if let Some(italics_element) = self.parse_italics()? {
						text_contents.push(italics_element);
					}
				},
				Some(Token::Code) => {
					if let Some(code_element) = self.parse_code()? {
						text_contents.push(code_element);
					}
				},
				Some(Token::Strikethrough) => {
					if let Some(strikethrough_element) = self.parse_strikethrough()? {
						text_contents.push(strikethrough_element);
					}
				},
				_ => {
					return Ok(Some(Element::Text(text_contents)));
				}
			}
		}
    }
}

#[derive(Debug, PartialEq)]
pub enum TextElement {
	Raw(String),
	Bold(String),
	Italics(String),
	Code(String),
	Strikethrough(String),
}

#[derive(Debug, PartialEq)]
enum Element {
    Section(TextElement),
    Subsection(TextElement),
    Subsubsection(TextElement),
    ListItem(TextElement),
    List(Vec<Element>),
    Quote(TextElement),
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken(message) =>
                write!(f, "Unexpected Token encountered, {message}"),
        }
    }
}

impl Error for ParseError {}