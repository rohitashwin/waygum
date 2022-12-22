pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    peek: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            peek: None,
        };
        lexer.advance();
        lexer
    }

    pub fn advance(&mut self) {
        self.peek = self.input[self.position..].chars().next();
        self.position += self.peek.map_or(0, |c| c.len_utf8());
    }

    pub fn token(&mut self) -> Token {
        // skip whitespace
        while let Some(c) = self.peek {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }

        let token = match self.peek {
            Some('#') => Token::Heading,
            Some('-') => Token::List,
            Some('>') => Token::Quote,
            Some('*') => Token::Bold,
            Some('/') => Token::Italic,
            Some('`') => Token::Code,
            Some('~') => Token::Strikethrough,
            Some(c) => Token::Text(c),
            None => Token::EOF,
        };

        self.advance();
        token
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Heading,
    List,
    Quote,
    Bold,
    Italic,
    Code,
    Strikethrough,
    Text(char),
    EOF,
}
