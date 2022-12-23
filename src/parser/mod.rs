use super::lexer::{ self, * };
use std::{ error::Error };
mod tests;

pub struct Parser {
    tokens: Vec<lexer::Token>,
    current_list_depth: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().rev().collect(),
            current_list_depth: 0,
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
            Some(Token::List(list_depth)) => self.parse_list(),
            // Some(Token::Quote) => self.parse_quote(),
            // Some(Token::Bold) => self.parse_bold(),
            // Some(Token::Italics) => self.parse_italics(),
            // Some(Token::Code) => self.parse_code(),
            // Some(Token::Strikethrough) => self.parse_strikethrough(),
            // Some(Token::Text(str_contents)) => self.parse_text(),
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
        let section_contents = match self.next() {
            Some(Token::Text(str_contents)) => str_contents,
            Some(token) => {
                return Err(
                    ParseError::UnexpectedToken(
                        format!("'Text' was expected but encountered {:?}", token)
                    )
                );
            }
            None => String::new(),
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
        // let Token::List(ListDepth(depth)) = self.peek().unwrap();
        todo!()
    }

    pub fn parse_list_item(&mut self) -> Result<Vec<Element>, ParseError> {
        let mut contents: Vec<String> = vec![];
        todo!()
    }
}

#[derive(Debug, PartialEq)]
enum Element {
    Section(String),
    Subsection(String),
    Subsubsection(String),
    ListItem(String),
    List(Vec<Element>),
    Quote(String),
    Bold(String),
    Italics(String),
    Code(String),
    Strikethrough(String),
    Text(String),
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