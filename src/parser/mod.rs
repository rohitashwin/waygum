use super::lexer::{ self, * };
use std::{ error::Error, vec };
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

    fn peek(&self) -> Option<&Token> {
        self.tokens.last()
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.pop()
    }

    fn parse_next(&mut self) -> Result<Option<Element>, ParseError> {
        match self.peek() {
            Some(Token::Section) => self.parse_section(),
            Some(Token::Subsection) => self.parse_subsection(),
            Some(Token::Subsubsection) => self.parse_subsubsection(),
            Some(Token::List(_)) => {
                self.current_list_depth = 1;
                self.parse_list()
            }
            Some(Token::Quote) => self.parse_quote(),
            Some(
                Token::Bold | Token::Italics | Token::Strikethrough | Token::Code | Token::Text(_),
            ) => self.parse_paragraph(),
            Some(Token::EOF) => Ok(None),
            Some(Token::Newline) => {
                self.next();
                self.parse_next()
            }
            _ => Err(ParseError::UnexpectedToken("Unknown token encountered".to_string())),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Element>, ParseError> {
        let mut elements: Vec<Element> = vec![];
        while let Some(element) = self.parse_next()? {
            elements.push(element);
        }
        Ok(elements)
    }

    fn parse_section(&mut self) -> Result<Option<Element>, ParseError> {
        let section_token = self.next();
        let section_contents = match self.peek() {
            Some(
                Token::Text(_) | Token::Bold | Token::Italics | Token::Strikethrough | Token::Code,
            ) => {
                self.parse_text()?
            }
            _ => Some(Text(vec![])),
        };
        Ok(Some(Element::Section(section_contents.unwrap())))
    }

    fn parse_subsection(&mut self) -> Result<Option<Element>, ParseError> {
        let subsection_token = self.next().unwrap();
        let subsection_contents = match self.peek() {
            Some(
                Token::Text(_) | Token::Bold | Token::Italics | Token::Strikethrough | Token::Code,
            ) => {
                self.parse_text()?
            }
            _ => Some(Text(vec![])),
        };
        Ok(Some(Element::Subsection(subsection_contents.unwrap())))
    }

    fn parse_subsubsection(&mut self) -> Result<Option<Element>, ParseError> {
        let subsubsection_token = self.next().unwrap();
        let subsubsection_contents = match self.peek() {
            Some(
                Token::Text(_) | Token::Bold | Token::Italics | Token::Strikethrough | Token::Code,
            ) => {
                self.parse_text()?
            }
            _ => Some(Text(vec![])),
        };
        Ok(Some(Element::Subsubsection(subsubsection_contents.unwrap())))
    }
    fn parse_list(&mut self) -> Result<Option<Element>, ParseError> {
        let mut list_items = vec![];
        loop {
            match self.peek() {
                Some(Token::List(ListDepth(depth))) => {
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

    fn parse_list_items(&mut self) -> Result<Vec<Element>, ParseError> {
        let mut contents = vec![];
        loop {
            match self.peek() {
                Some(Token::List(ListDepth(depth))) => {
                    if *depth == self.current_list_depth {
                        let _list_marker = self.next();
                        let list_content = self
                            .parse_text()?
                            .ok_or(
                                ParseError::UnexpectedToken(format!("Unexpected List Parse Error"))
                            )?;
                        contents.push(Element::ListItem(list_content));
                    } else {
                        break;
                    }
                }
				Some(Token::Newline) => {
					self.next();
				}
                _ => {
                    break;
                }
            }
        }
        Ok(contents)
    }

    fn parse_quote(&mut self) -> Result<Option<Element>, ParseError> {
        let _quote_token = self.next().unwrap();
        let quote_contents = match self.peek() {
            Some(
                Token::Text(_) | Token::Bold | Token::Italics | Token::Strikethrough | Token::Code,
            ) => {
                self.parse_text()?
            }
            _ => Some(Text(vec![])),
        };
        Ok(Some(Element::Quote(quote_contents.unwrap())))
    }

    fn parse_bold(&mut self) -> Result<Option<TextElement>, ParseError> {
        let bold_token = self.next().unwrap();
        // consume text until next bold token
        let mut bold_contents = String::new();
        while let Some(token) = self.peek() {
            match token {
                Token::Bold => {
                    self.next();
                    break;
                }
                Token::Text(str_contents) => {
                    bold_contents.push_str(str_contents);
                    self.next();
                }
                Token::EOF => {
                    break;
                }
                _ => {
                    return Err(
                        ParseError::UnexpectedToken(format!("Unexpected token in bold text"))
                    );
                }
            }
        }
        Ok(Some(TextElement::Bold(bold_contents)))
    }

    fn parse_italics(&mut self) -> Result<Option<TextElement>, ParseError> {
        let italics_token = self.next().unwrap();
        // consume text until next italics token
        let mut italics_contents = String::new();
        while let Some(token) = self.peek() {
            match token {
                Token::Italics => {
                    self.next();
                    break;
                }
                Token::Text(str_contents) => {
                    italics_contents.push_str(str_contents);
                    self.next();
                }
                Token::EOF => {
                    break;
                }
                _ => {
                    return Err(
                        ParseError::UnexpectedToken(format!("Unexpected token in italics text"))
                    );
                }
            }
        }
        Ok(Some(TextElement::Italics(italics_contents)))
    }

    fn parse_code(&mut self) -> Result<Option<TextElement>, ParseError> {
        let code_token = self.next().unwrap();
        // consume text until next code token
        let mut code_contents = String::new();
        while let Some(token) = self.peek() {
            match token {
                Token::Code => {
                    self.next();
                    break;
                }
                Token::Text(str_contents) => {
                    code_contents.push_str(str_contents);
                    self.next();
                }
                Token::EOF => {
                    break;
                }
                _ => {
                    return Err(
                        ParseError::UnexpectedToken(format!("Unexpected token in code text"))
                    );
                }
            }
        }
        Ok(Some(TextElement::Code(code_contents)))
    }

    fn parse_strikethrough(&mut self) -> Result<Option<TextElement>, ParseError> {
        let strikethrough_token = self.next().unwrap();
        // consume text until next strikethrough token
        let mut strikethrough_contents = String::new();
        while let Some(token) = self.peek() {
            match token {
                Token::Strikethrough => {
                    self.next();
                    break;
                }
                Token::Text(str_contents) => {
                    strikethrough_contents.push_str(str_contents);
                    self.next();
                }
                Token::EOF => {
                    break;
                }
                _ => {
                    return Err(
                        ParseError::UnexpectedToken(
                            format!("Unexpected token in strikethrough text")
                        )
                    );
                }
            }
        }
        Ok(Some(TextElement::Strikethrough(strikethrough_contents)))
    }

    fn parse_text(&mut self) -> Result<Option<Text>, ParseError> {
        let mut text_contents = vec![];
        loop {
            match self.peek() {
                Some(Token::Text(_)) => {
                    let text_token = self.next().unwrap();
                    if let Token::Text(str_contents) = text_token {
                        text_contents.push(TextElement::Raw(str_contents));
                    }
                }
                Some(Token::Bold) => {
                    if let Some(bold_element) = self.parse_bold()? {
                        text_contents.push(bold_element);
                    }
                }
                Some(Token::Italics) => {
                    if let Some(italics_element) = self.parse_italics()? {
                        text_contents.push(italics_element);
                    }
                }
                Some(Token::Strikethrough) => {
                    if let Some(strikethrough_element) = self.parse_strikethrough()? {
                        text_contents.push(strikethrough_element);
                    }
                }
                Some(Token::Code) => {
                    if let Some(code_element) = self.parse_code()? {
                        text_contents.push(code_element);
                    }
                }
                _ => {
                    break;
                }
            }
        }
        Ok(Some(Text(text_contents)))
    }

    fn parse_paragraph(&mut self) -> Result<Option<Element>, ParseError> {
        let paragraph_contents = self.parse_text()?;
        Ok(Some(Element::Paragraph(paragraph_contents.unwrap())))
    }
}

#[derive(Debug, PartialEq)]
enum TextElement {
    Raw(String),
    Bold(String),
    Italics(String),
    Code(String),
    Strikethrough(String),
}

#[derive(Debug, PartialEq)]
struct Text(Vec<TextElement>);

#[derive(Debug, PartialEq)]
enum Element {
    Section(Text),
    Subsection(Text),
    Subsubsection(Text),
    ListItem(Text),
    List(Vec<Element>),
    Quote(Text),
    Paragraph(Text),
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