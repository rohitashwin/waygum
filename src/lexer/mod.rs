use std::char;

mod tests;

pub struct Lexer<'lt> {
    input: &'lt str,
    pos: usize,
}

#[derive(Debug, PartialEq)]
pub struct ListDepth(pub usize);

#[derive(Debug, PartialEq)]
pub enum Token {
    Section,
    Subsection,
    Subsubsection,
    List(ListDepth),
    Quote,
    Bold,
    Italics,
    Code,
    Strikethrough,
    Text(String),
    EOF,
    // TODO: Implement a Escape sequence token
}

impl<'lt> Lexer<'lt> {
    pub fn new(input: &'lt str) -> Self {
        Self {
            input,
            pos: 0,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    pub fn advance(&mut self) -> Option<char> {
        match self.peek() {
            Some(ch) => {
                self.pos += 1;
                Some(ch)
            }
            None => None,
        }
    }

    pub fn advance_while<F: Fn(char) -> bool>(&mut self, lambda: F) -> String {
        let mut output = String::new();
        while let Some(ch) = self.peek() {
            if lambda(ch) {
                output.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        output
    }

    pub fn advance_whitespace(&mut self) {
        self.advance_while(|c| c.is_whitespace());
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.peek() {
            match ch {
                '#' => {
                    let hashes = self.advance_while(|c| c == '#');
                    match hashes.len() {
                        1 => {
                            tokens.push(Token::Section);
                        }
                        2 => {
                            tokens.push(Token::Subsection);
                        }
                        _ => {
                            tokens.push(Token::Subsubsection);
                        }
                    }
                    self.advance_whitespace();
                }
                '-' => {
                    let dashes = self.advance_while(|c| c == '-');
                    self.advance_whitespace();
                    tokens.push(Token::List(ListDepth(dashes.len())));
                }
                '>' => {
                    self.advance();
                    self.advance_whitespace();
                    tokens.push(Token::Quote);
                }
                '*' => {
                    self.advance();
                    self.advance_whitespace();
                    tokens.push(Token::Bold);
                }
                '/' => {
                    self.advance();
                    self.advance_whitespace();
                    tokens.push(Token::Italics);
                }
                '`' => {
                    self.advance();
                    self.advance_whitespace();
                    tokens.push(Token::Code);
                }
                '~' => {
                    self.advance();
                    self.advance_whitespace();
                    tokens.push(Token::Strikethrough);
                }
                _ => {
                    let inline_characters = ['*', '/', '`', '~'];
                    let mut text = String::new();
                    while let Some(ch) = self.peek() {
                        if ch == '\n' || inline_characters.contains(&ch) {
                            break;
                        }
                        text.push(ch);
                        self.advance();
                    }
                    tokens.push(Token::Text(text.trim_end().to_string()));
                    self.advance_whitespace();
                }
            }
        }
        self.advance_whitespace();
        tokens.push(Token::EOF);
        tokens
    }
}