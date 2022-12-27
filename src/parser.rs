use lazy_static::lazy_static;
use std::{ sync::Mutex };
use super::lexer::*;

#[derive(Debug)]
struct ListState {
    current_depth: usize,
}

#[derive(Debug)]
struct SectionState {
    section_number: usize,
    subsection_number: usize,
    subsubsection_number: usize,
}

lazy_static! {
    static ref LIST_STATE: Mutex<ListState> = Mutex::new(ListState { current_depth: 1 });
    static ref PARSE_LINE: Mutex<Vec<char>> = Mutex::new(vec![]);
    static ref SECTION_STATE: Mutex<SectionState> = Mutex::new(SectionState {
        section_number: 1,
        subsection_number: 1,
        subsubsection_number: 1,
    });
}

pub struct Parser {
    pub tokens: Vec<Token>,
}

#[derive(Debug, PartialEq)]
pub enum TextArtefact {
    Raw(String),
    Bold(String),
    Italics(String),
    Link(String, String),
    Strikethrough(String),
    Underline(String),
    Code(String),
}

#[derive(Debug, PartialEq)]
pub struct Text(pub Vec<TextArtefact>);

#[derive(Debug, PartialEq)]
pub enum ParseArtefact {
    Section(usize, String),
    Subsection(usize, String),
    Subsubsection(usize, String),
    Paragraph(Vec<Text>),
    List(Vec<ParseArtefact>),
    ListItem(Text),
    Quote(Text),
    Table(Vec<ParseArtefact>),
    TableRow(Vec<Text>),
    Button(String, String),
    Image(String, String),
    Codeblock(String),
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.into_iter().rev().collect(),
        }
    }

    fn next(&self) -> Option<&Token> {
        self.tokens.last()
    }

    fn consume(&mut self) -> Option<Token> {
        self.tokens.pop()
    }

    fn consume_while(&mut self, f: impl Fn(&Token) -> bool) -> Vec<Token> {
        let mut result = Vec::new();
        while let Some(token) = self.next() {
            if f(token) {
                result.push(self.consume().unwrap());
            } else {
                break;
            }
        }
        result
    }

    fn parse(&mut self) -> Result<Vec<ParseArtefact>, Box<dyn std::error::Error>> {
        let mut result = vec![];
        while self.next().is_some() {
            result.extend(self.parse_token()?);
        }
        Ok(result)
    }

    fn parse_token(&mut self) -> Result<Vec<ParseArtefact>, Box<dyn std::error::Error>> {
        match self.next() {
            Some(Token::Section(_)) => self.parse_section(),
            Some(Token::Subsection(_)) => self.parse_subsection(),
            Some(Token::Subsubsection(_)) => self.parse_subsubsection(),
            Some(Token::List { .. }) => {
                LIST_STATE.lock()?.current_depth = 1;
                self.parse_list()
            }
            Some(Token::Quote(_)) => self.parse_quote(),
            Some(Token::Table(_)) => Ok(vec![self.parse_table()?]),
            Some(Token::Codeblock(_)) => Ok(vec![self.parse_codeblock()?]),
            Some(Token::Image { .. }) => Ok(vec![self.parse_image()?]),
            Some(Token::Button { .. }) => Ok(vec![self.parse_button()?]),
            Some(Token::Text(_)) => Ok(vec![self.parse_paragraph()?]),
            Some(Token::Newline) => {
                self.consume();
                Ok(vec![])
            }
            _ => {
                self.consume();
                Ok(vec![])
            }
        }
    }

    fn parse_text_artefacts(
        &mut self,
        text: String
    ) -> Result<Vec<TextArtefact>, Box<dyn std::error::Error>> {
        let mut artefacts = vec![];
        *PARSE_LINE.lock()? = text.chars().rev().collect();
        let next_char = || -> Result<Option<char>, Box<dyn std::error::Error>> {
            Ok(PARSE_LINE.lock()?.last().cloned())
        };
        let consume_char = || -> Result<Option<char>, Box<dyn std::error::Error>> {
            Ok(PARSE_LINE.lock()?.pop())
        };
        let consume_chars_while = |
            f: fn(char) -> bool
        | -> Result<String, Box<dyn std::error::Error>> {
            let mut result = String::new();
            while let Some(ch) = next_char()? {
                if f(ch) {
                    result.push(consume_char()?.unwrap());
                } else {
                    break;
                }
            }
            Ok(result)
        };
        let mut current_string = String::new();
        while let Some(ch) = next_char()? {
            match ch {
                '*' => {
                    if current_string.len() > 0 {
                        artefacts.push(TextArtefact::Raw(current_string));
                        current_string = String::new();
                    }
                    consume_char()?;
                    let bold_contents = consume_chars_while(|ch| ch != '*')?;
                    consume_char()?;
                    artefacts.push(TextArtefact::Bold(bold_contents));
                }
                '/' => {
                    if current_string.len() > 0 {
                        artefacts.push(TextArtefact::Raw(current_string));
                        current_string = String::new();
                    }
                    consume_char()?;
                    let italics_contents = consume_chars_while(|ch| ch != '/')?;
                    consume_char()?;
                    artefacts.push(TextArtefact::Italics(italics_contents));
                }
                '$' => {
                    if current_string.len() > 0 {
                        artefacts.push(TextArtefact::Raw(current_string));
                        current_string = String::new();
                    }
                    consume_char()?;
                    let code_contents = consume_chars_while(|ch| ch != '$')?;
                    consume_char()?;
                    artefacts.push(TextArtefact::Code(code_contents));
                }
                '!' => {
                    consume_char()?;
                    if let Some(ch) = next_char()? {
                        if ch == '[' {
                            if current_string.len() > 0 {
                                artefacts.push(TextArtefact::Raw(current_string));
                                current_string = String::new();
                            }
                            consume_char()?;
                            let link_contents = consume_chars_while(|ch| ch != ']')?;
                            consume_char()?;
                            match consume_char()? {
                                Some('(') => (),
                                Some(ch) => {
                                    return Err(
                                        Box::new(
                                            ParseError::UnexpectedChar(
                                                String::from("INLINE_LINK"),
                                                '(',
                                                ch
                                            )
                                        )
                                    );
                                }
                                None => {
                                    return Err(Box::new(ParseError::UnexpectedEOL));
                                }
                            }
                            let link_url = consume_chars_while(|ch| ch != ')')?;
                            consume_char()?;
                            artefacts.push(TextArtefact::Link(link_contents, link_url));
                        } else {
                            current_string.push('!');
                        }
                    } else {
                        current_string.push('!');
                    }
                }
                '~' => {
                    if current_string.len() > 0 {
                        artefacts.push(TextArtefact::Raw(current_string));
                        current_string = String::new();
                    }
                    consume_char()?;
                    let strikethrough_contents = consume_chars_while(|ch| ch != '~')?;
                    consume_char()?;
                    artefacts.push(TextArtefact::Strikethrough(strikethrough_contents));
                }
                '_' => {
                    if current_string.len() > 0 {
                        artefacts.push(TextArtefact::Raw(current_string));
                        current_string = String::new();
                    }
                    consume_char()?;
                    let underline_contents = consume_chars_while(|ch| ch != '_')?;
                    consume_char()?;
                    artefacts.push(TextArtefact::Underline(underline_contents));
                }
                _ => {
                    current_string.push(consume_char()?.unwrap());
                }
            }
        }
        if current_string.len() > 0 {
            artefacts.push(TextArtefact::Raw(current_string));
        }
        Ok(artefacts)
    }

    fn parse_text(&mut self, text: String) -> Result<Text, Box<dyn std::error::Error>> {
        Ok(Text(self.parse_text_artefacts(text)?))
    }

    fn parse_section(&mut self) -> Result<Vec<ParseArtefact>, Box<dyn std::error::Error>> {
        let section_name = match self.consume() {
            Some(Token::Section(name)) => name,
            Some(token) => {
                return Err(
                    Box::new(
                        ParseError::UnexpectedToken(
                            "SECTION".to_string(),
                            Token::Section("_section_header_".to_string()),
                            token
                        )
                    )
                );
            }
            None => {
                return Err(Box::new(ParseError::UnexpectedEOF));
            }
        };
        let section_number = SECTION_STATE.lock()?.section_number;
        SECTION_STATE.lock()?.section_number += 1;
        SECTION_STATE.lock()?.subsection_number = 1;
        SECTION_STATE.lock()?.subsubsection_number = 1;
        Ok(vec![ParseArtefact::Section(section_number, section_name)])
    }

    fn parse_subsection(&mut self) -> Result<Vec<ParseArtefact>, Box<dyn std::error::Error>> {
        let subsection_name = match self.consume() {
            Some(Token::Subsection(name)) => name,
            Some(token) => {
                return Err(
                    Box::new(
                        ParseError::UnexpectedToken(
                            "SUBSECTION".to_string(),
                            Token::Subsection("_subsection_header_".to_string()),
                            token
                        )
                    )
                );
            }
            None => {
                return Err(Box::new(ParseError::UnexpectedEOF));
            }
        };
        let subsection_number = SECTION_STATE.lock()?.subsection_number;
        SECTION_STATE.lock()?.subsection_number += 1;
        SECTION_STATE.lock()?.subsubsection_number = 1;
        Ok(vec![ParseArtefact::Subsection(subsection_number, subsection_name)])
    }

    fn parse_subsubsection(&mut self) -> Result<Vec<ParseArtefact>, Box<dyn std::error::Error>> {
        let subsubsection_name = match self.consume() {
            Some(Token::Subsubsection(name)) => name,
            Some(token) => {
                return Err(
                    Box::new(
                        ParseError::UnexpectedToken(
                            "SUBSUBSECTION".to_string(),
                            Token::Subsubsection("_subsubsection_header_".to_string()),
                            token
                        )
                    )
                );
            }
            None => {
                return Err(Box::new(ParseError::UnexpectedEOF));
            }
        };
        let subsubsection_number = SECTION_STATE.lock()?.subsubsection_number;
        SECTION_STATE.lock()?.subsubsection_number += 1;
        Ok(vec![ParseArtefact::Subsubsection(subsubsection_number, subsubsection_name)])
    }

    fn parse_list(&mut self) -> Result<Vec<ParseArtefact>, Box<dyn std::error::Error>> {
        let list_items = self.parse_list_items()?;
        Ok(vec![ParseArtefact::List(list_items)])
    }

    fn parse_list_items(&mut self) -> Result<Vec<ParseArtefact>, Box<dyn std::error::Error>> {
        let mut list_items = vec![];
        while let Some(Token::List { depth, text }) = self.next() {
            let current_list_depth = LIST_STATE.lock()?.current_depth;
            if &current_list_depth == depth {
                if let Some(Token::List { depth, text }) = self.consume() {
                    list_items.push(ParseArtefact::ListItem(self.parse_text(text)?));
                } else {
                    return Err(Box::new(ParseError::UnexpectedEOF));
                }
            } else if &current_list_depth < depth {
                LIST_STATE.lock()?.current_depth += 1;
                list_items.extend(self.parse_list()?);
            } else {
                LIST_STATE.lock()?.current_depth -= 1;
                break;
            }
        }
        Ok(list_items)
    }

    fn parse_quote(&mut self) -> Result<Vec<ParseArtefact>, Box<dyn std::error::Error>> {
        let quote = match self.consume() {
            Some(Token::Quote(quote)) => quote,
            Some(token) => {
                return Err(
                    Box::new(
                        ParseError::UnexpectedToken(
                            "QUOTE".to_string(),
                            Token::Quote("_quote_".to_string()),
                            token
                        )
                    )
                );
            }
            None => {
                return Err(Box::new(ParseError::UnexpectedEOF));
            }
        };
        let text_contents = self.parse_text(quote)?;
        Ok(vec![ParseArtefact::Quote(text_contents)])
    }

    fn parse_table(&mut self) -> Result<ParseArtefact, Box<dyn std::error::Error>> {
        let mut table_rows = vec![];
        while let Some(Token::Table(_)) = self.next() {
            if let Token::Table(columns) = self.consume().unwrap() {
                let mut table_row = vec![];
                for column in columns {
                    table_row.push(self.parse_text(column)?);
                }
                table_rows.push(ParseArtefact::TableRow(table_row));
                self.consume();
            } else {
                return Err(Box::new(ParseError::UnexpectedEOF));
            }
        }
        Ok(ParseArtefact::Table(table_rows))
    }

    fn parse_paragraph(&mut self) -> Result<ParseArtefact, Box<dyn std::error::Error>> {
        let mut text_contents = vec![];
        while let Some(Token::Text(_)) = self.next() {
            if let Token::Text(text) = self.consume().unwrap() {
                text_contents.push(self.parse_text(text)?);
                self.consume();
            } else {
                return Err(Box::new(ParseError::UnexpectedEOF));
            }
        }
        Ok(ParseArtefact::Paragraph(text_contents))
    }

    fn parse_codeblock(&mut self) -> Result<ParseArtefact, Box<dyn std::error::Error>> {
        match self.consume() {
            Some(Token::Codeblock(codeblock_contents)) =>
                Ok(ParseArtefact::Codeblock(codeblock_contents)),
            Some(token) =>
                Err(
                    Box::new(
                        ParseError::UnexpectedToken(
                            "CODEBLOCK".to_string(),
                            Token::Codeblock("_codeblock_".to_string()),
                            token
                        )
                    )
                ),
            None => Err(Box::new(ParseError::UnexpectedEOF)),
        }
    }

    fn parse_image(&mut self) -> Result<ParseArtefact, Box<dyn std::error::Error>> {
        match self.consume() {
            Some(Token::Image { caption, path }) => Ok(ParseArtefact::Image(caption, path)),
            Some(token) =>
                Err(
                    Box::new(
                        ParseError::UnexpectedToken(
                            "IMAGE".to_string(),
                            Token::Image {
                                caption: String::from("_caption_"),
                                path: String::from("_path_"),
                            },
                            token
                        )
                    )
                ),
            None => Err(Box::new(ParseError::UnexpectedEOF)),
        }
    }

    fn parse_button(&mut self) -> Result<ParseArtefact, Box<dyn std::error::Error>> {
        match self.consume() {
            Some(Token::Button { text, link }) => Ok(ParseArtefact::Button(text, link)),
            Some(token) =>
                Err(
                    Box::new(
                        ParseError::UnexpectedToken(
                            "BUTTON".to_string(),
                            Token::Button {
                                text: String::from("_caption_"),
                                link: String::from("_path_"),
                            },
                            token
                        )
                    )
                ),
            None => Err(Box::new(ParseError::UnexpectedEOF)),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String, Token, Token),
    UnexpectedChar(String, char, char),
    UnexpectedEOL,
    UnexpectedEOF,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedChar(location, expected, found) =>
                write!(
                    f,
                    "Unexpected char while parsing {}: expected {:?}, found {:?}",
                    location,
                    expected,
                    found
                ),
            ParseError::UnexpectedToken(location, expected, found) =>
                write!(
                    f,
                    "Unexpected token while parsing {}: expected {:?}, found {:?}",
                    location,
                    expected,
                    found
                ),
            ParseError::UnexpectedEOL => write!(f, "Unexpected end of line"),
            ParseError::UnexpectedEOF => write!(f, "Unexpected EOF while parsing"),
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() -> Result<(), Box<dyn std::error::Error>> {
        let mut lexer = Lexer::new(String::from(""));
        let mut tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let parse_result = parser.parse()?;
        assert_eq!(parse_result, Vec::<ParseArtefact>::new());
        Ok(())
    }

    #[test]
    fn parse_text() -> Result<(), Box<dyn std::error::Error>> {
        let mut lexer = Lexer::new(String::from("Hello world!"));
        let mut tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let parse_result = parser.parse()?;
        println!("{:?}", parse_result);
        assert_eq!(
            parse_result,
            vec![
                ParseArtefact::Paragraph(
                    vec![Text(vec![TextArtefact::Raw(String::from("Hello world!"))])]
                )
            ]
        );
        Ok(())
    }

    // Style tests
    #[test]
    fn parse_bold() -> Result<(), Box<dyn std::error::Error>> {
        let mut lexer = Lexer::new(String::from("*Hello world!*"));
        let mut tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let parse_result = parser.parse()?;
        println!("{:?}", parse_result);
        assert_eq!(
            parse_result,
            vec![
                ParseArtefact::Paragraph(
                    vec![Text(vec![TextArtefact::Bold(String::from("Hello world!"))])]
                )
            ]
        );
        Ok(())
    }

    #[test]
    fn parse_italics() -> Result<(), Box<dyn std::error::Error>> {
        let mut lexer = Lexer::new(String::from("/Hello world!/"));
        let mut tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let parse_result = parser.parse()?;
        println!("{:?}", parse_result);
        assert_eq!(
            parse_result,
            vec![
                ParseArtefact::Paragraph(
                    vec![Text(vec![TextArtefact::Italics(String::from("Hello world!"))])]
                )
            ]
        );
        Ok(())
    }

    #[test]
    fn parse_underline() -> Result<(), Box<dyn std::error::Error>> {
        let mut lexer = Lexer::new(String::from("_Hello world!_"));
        let mut tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let parse_result = parser.parse()?;
        println!("{:?}", parse_result);
        assert_eq!(
            parse_result,
            vec![
                ParseArtefact::Paragraph(
                    vec![Text(vec![TextArtefact::Underline(String::from("Hello world!"))])]
                )
            ]
        );
        Ok(())
    }

    #[test]
    fn parse_strikethrough() -> Result<(), Box<dyn std::error::Error>> {
        let mut lexer = Lexer::new(String::from("~Hello world!~"));
        let mut tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let parse_result = parser.parse()?;
        println!("{:?}", parse_result);
        assert_eq!(
            parse_result,
            vec![
                ParseArtefact::Paragraph(
                    vec![Text(vec![TextArtefact::Strikethrough(String::from("Hello world!"))])]
                )
            ]
        );
        Ok(())
    }

    #[test]
    fn parse_code() -> Result<(), Box<dyn std::error::Error>> {
        let mut lexer = Lexer::new(String::from("$Hello world!$"));
        let mut tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let parse_result = parser.parse()?;
        println!("{:?}", parse_result);
        assert_eq!(
            parse_result,
            vec![
                ParseArtefact::Paragraph(
                    vec![Text(vec![TextArtefact::Code(String::from("Hello world!"))])]
                )
            ]
        );
        Ok(())
    }

    #[test]
    fn parse_link() -> Result<(), Box<dyn std::error::Error>> {
        let mut lexer = Lexer::new(String::from("![Hello world!](https://example.com)"));
        let mut tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let parse_result = parser.parse()?;
        println!("{:?}", parse_result);
        assert_eq!(
            parse_result,
            vec![
                ParseArtefact::Paragraph(
                    vec![
                        Text(
                            vec![
                                TextArtefact::Link(
                                    String::from("Hello world!"),
                                    String::from("https://example.com")
                                )
                            ]
                        )
                    ]
                )
            ]
        );
        Ok(())
    }

    #[test]
    fn parse_image() -> Result<(), Box<dyn std::error::Error>> {
        let mut lexer = Lexer::new(String::from("#[Hello world!](https://example.com)"));
        let mut tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let parse_result = parser.parse()?;
        println!("{:?}", parse_result);
        assert_eq!(
            parse_result,
            vec![
                ParseArtefact::Image(
                    String::from("Hello world!"),
                    String::from("https://example.com")
                )
            ]
        );
        Ok(())
    }

    #[test]
    fn parse_section() -> Result<(), Box<dyn std::error::Error>> {
        let mut lexer = Lexer::new(String::from("@ Hello world!"));
        let mut tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let parse_result = parser.parse()?;
        println!("{:?}", parse_result);
        assert_eq!(parse_result, vec![ParseArtefact::Section(1, String::from("Hello world!"))]);
        Ok(())
    }

    #[test]
    fn parse_subsection() -> Result<(), Box<dyn std::error::Error>> {
        let mut lexer = Lexer::new(String::from("@@ Hello world!"));
        let mut tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let parse_result = parser.parse()?;
        println!("{:?}", parse_result);
        assert_eq!(parse_result, vec![ParseArtefact::Subsection(1, String::from("Hello world!"))]);
        Ok(())
    }

    #[test]
    fn parse_subsubsection() -> Result<(), Box<dyn std::error::Error>> {
        let mut lexer = Lexer::new(String::from("@@@ Hello world!"));
        let mut tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let parse_result = parser.parse()?;
        println!("{:?}", parse_result);
        assert_eq!(
            parse_result,
            vec![ParseArtefact::Subsubsection(1, String::from("Hello world!"))]
        );
        Ok(())
    }

    #[test]
    fn parse_list() -> Result<(), Box<dyn std::error::Error>> {
        let mut lexer = Lexer::new(String::from("- Hello world!"));
        let mut tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let parse_result = parser.parse()?;
        println!("{:?}", parse_result);
        assert_eq!(
            parse_result,
            vec![
                ParseArtefact::List(
                    vec![
                        ParseArtefact::ListItem(
                            Text(vec![TextArtefact::Raw(String::from("Hello world!"))])
                        )
                    ]
                )
            ]
        );
        Ok(())
    }

    #[test]
    fn parse_multilevel_list() -> Result<(), Box<dyn std::error::Error>> {
        let mut lexer = Lexer::new(String::from("- Hello world!\n-- Hello world!\n-Hello world!"));
        let mut tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        let parse_result = parser.parse()?;
        println!("{:?}", parse_result);
        assert_eq!(
            parse_result,
            vec![
                ParseArtefact::List(
                    vec![
                        ParseArtefact::ListItem(
                            Text(vec![TextArtefact::Raw(String::from("Hello world!"))])
                        ),
                        ParseArtefact::List(
							vec![
								ParseArtefact::ListItem(
									Text(vec![TextArtefact::Raw(String::from("Hello world!"))])
								)
							]
						),
                        ParseArtefact::ListItem(
                            Text(vec![TextArtefact::Raw(String::from("Hello world!"))])
                        )
                    ]
                )
            ]
        );
        Ok(())
    }

	#[test]
	fn parse_multilevel_list_with_paragraph() -> Result<(), Box<dyn std::error::Error>> {
		let mut lexer = Lexer::new(String::from("- Hello world!\n-- Hello world!\n-Hello world!\n\nHello world!"));
		let mut tokens = lexer.tokenize()?;
		let mut parser = Parser::new(tokens);
		let parse_result = parser.parse()?;
		println!("{:?}", parse_result);
		assert_eq!(
			parse_result,
			vec![
				ParseArtefact::List(
					vec![
						ParseArtefact::ListItem(
							Text(vec![TextArtefact::Raw(String::from("Hello world!"))])
						),
						ParseArtefact::List(
							vec![
								ParseArtefact::ListItem(
									Text(vec![TextArtefact::Raw(String::from("Hello world!"))])
								)
							]
						),
						ParseArtefact::ListItem(
							Text(vec![TextArtefact::Raw(String::from("Hello world!"))])
						)
					]
				),
				ParseArtefact::Paragraph(
					vec![Text(vec![TextArtefact::Raw(String::from("Hello world!"))])]
				)
			]
		);
		Ok(())
	}

	#[test]
	fn parse_table() -> Result<(), Box<dyn std::error::Error>> {
		let mut lexer = Lexer::new(String::from("| Hello world! | Hello world! |"));
		let mut tokens = lexer.tokenize()?;
		let mut parser = Parser::new(tokens);
		let parse_result = parser.parse()?;
		println!("{:?}", parse_result);
		assert_eq!(
			parse_result,
			vec![
				ParseArtefact::Table(
					vec![ParseArtefact::TableRow(
						vec![
							Text(vec![TextArtefact::Raw(String::from("Hello world!"))]),
							Text(vec![TextArtefact::Raw(String::from("Hello world!"))])
						]
				)]
				)
			]
		);
		Ok(())
	}

	#[test]
	fn parse_multiline_table() -> Result<(), Box<dyn std::error::Error>> {
		let mut lexer = Lexer::new(String::from("| Hello world! | Hello world! |\n| Hello world! | Hello world! |"));
		let mut tokens = lexer.tokenize()?;
		let mut parser = Parser::new(tokens);
		let parse_result = parser.parse()?;
		println!("{:?}", parse_result);
		assert_eq!(
			parse_result,
			vec![
				ParseArtefact::Table(
					vec![
						ParseArtefact::TableRow(
							vec![
								Text(vec![TextArtefact::Raw(String::from("Hello world!"))]),
								Text(vec![TextArtefact::Raw(String::from("Hello world!"))])
							]
						),
						ParseArtefact::TableRow(
							vec![
								Text(vec![TextArtefact::Raw(String::from("Hello world!"))]),
								Text(vec![TextArtefact::Raw(String::from("Hello world!"))])
							]
						)
					]
				)
			]
		);
		Ok(())
	}

	#[test]
	fn parse_multiline_table_with_paragraph() -> Result<(), Box<dyn std::error::Error>> {
		let mut lexer = Lexer::new(String::from("| Hello world! | Hello world! |\n| Hello world! | Hello world! |\n\nHello world!"));
		let mut tokens = lexer.tokenize()?;
		let mut parser = Parser::new(tokens);
		let parse_result = parser.parse()?;
		println!("{:?}", parse_result);
		assert_eq!(
			parse_result,
			vec![
				ParseArtefact::Table(
					vec![
						ParseArtefact::TableRow(
							vec![
								Text(vec![TextArtefact::Raw(String::from("Hello world!"))]),
								Text(vec![TextArtefact::Raw(String::from("Hello world!"))])
							]
						),
						ParseArtefact::TableRow(
							vec![
								Text(vec![TextArtefact::Raw(String::from("Hello world!"))]),
								Text(vec![TextArtefact::Raw(String::from("Hello world!"))])
							]
						)
					]
				),
				ParseArtefact::Paragraph(
					vec![Text(vec![TextArtefact::Raw(String::from("Hello world!"))])]
				)
			]
		);
		Ok(())
	}
}