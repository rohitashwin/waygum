use super::lexer::*;
use std::error::Error as StdError; 
pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser { lexer }
    }

    pub fn parse(&mut self) -> Result<Document, Error> {
        let mut document = Document::new();
        while let Some(element) = self.next()? {
            document.elements.push(element);
        }
        Ok(document)
    }

    fn next(&mut self) -> Result<Option<Element>, Error> {
        match self.lexer.token() {
            Token::EOF => Ok(None),
            Token::Heading => self.parse_heading(),
            Token::List => self.parse_list(),
            Token::Quote => self.parse_quote(),
            Token::Bold => self.parse_style(Token::Bold, Element::Bold),
            Token::Italic => self.parse_style(Token::Italic, Element::Italic),
            Token::Code => self.parse_style(Token::Code, Element::Code),
            Token::Strikethrough => self.parse_style(Token::Strikethrough, Element::Strikethrough),
            Token::Text(c) => Ok(Some(Element::Text(c.to_string()))),
        }
    }

    fn parse_heading(&mut self) -> Result<Option<Element>, Error> {
        let mut content = String::new();
        loop {
            match self.lexer.token() {
                Token::Text(c) => content.push(c),
                Token::Heading => break,
                Token::EOF => return Ok(Some(Element::Heading(content))),
                _ => return Err(Error::UnexpectedToken),
            }
        }
        Ok(Some(Element::Heading(content)))
    }

    fn parse_list(&mut self) -> Result<Option<Element>, Error> {
        let mut items = Vec::new();
        loop {
            match self.lexer.token() {
                Token::List => {
                    let item = self.parse_list_item()?.unwrap();
                    items.push(item);
                }
                Token::EOF => return Ok(Some(Element::List(items))),
                _ => return Err(Error::UnexpectedToken),
            }
        }
    }

    fn parse_list_item(&mut self) -> Result<Option<Element>, Error> {
        let mut content = String::new();
        loop {
            match self.lexer.token() {
                Token::Text(c) => content.push(c),
                Token::List => break,
                Token::EOF => return Ok(Some(Element::ListItem(content))),
                _ => return Err(Error::UnexpectedToken),
            }
        }
        Ok(Some(Element::ListItem(content)))
    }

    fn parse_quote(&mut self) -> Result<Option<Element>, Error> {
        let mut content = String::new();
        loop {
            match self.lexer.token() {
                Token::Text(c) => content.push(c),
                Token::Quote => break,
                Token::EOF => return Ok(Some(Element::Quote(content))),
                _ => return Err(Error::UnexpectedToken),
            }
        }
        Ok(Some(Element::Quote(content)))
    }


    fn parse_style<F>(&mut self, end_token: Token, constructor: F) -> Result<Option<Element>, Error>
    where
        F: Fn(String) -> Element,
    {
        let mut content = String::new();
        loop {
            match self.lexer.token() {
                Token::Text(c) => content.push(c),
                t if t == end_token => break,
                Token::EOF => return Ok(Some(constructor(content))),
                _ => return Err(Error::UnexpectedToken),
            }
        }
        Ok(Some(constructor(content)))
    }
}

#[derive(Debug, PartialEq)]
pub struct Document {
    pub elements: Vec<Element>,
}

impl Document {
    fn new() -> Self {
        Document { elements: Vec::new() }
    }
}

#[derive(Debug, PartialEq)]
pub enum Element {
    Heading(String),
    List(Vec<Element>),
    ListItem(String),
    Quote(String),
    Bold(String),
    Italic(String),
    Code(String),
    Strikethrough(String),
    Text(String),
}

#[derive(Debug)]
pub enum Error {
    UnexpectedToken,
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::UnexpectedToken => write!(f, "unexpected token"),
        }
    }
}

impl StdError for Error {}