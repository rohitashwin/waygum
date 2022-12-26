use std::{ error::Error, vec, collections::vec_deque };

pub struct Lexer {
	lines: Vec<String>,
	line: usize,
	pos: usize,
}

impl Lexer {
	fn from(lines: Vec<String>) -> Lexer {
		Lexer {
			lines,
			line: 0,
			pos: 0,
		}
	}

	fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
		let mut tokens = vec![];
		for line in self.lines {

		}
	}

	fn tokenize_section_kind(&mut self) -> Result<Vec<Token>>, LexerError> {
		Ok(vec![])
	}

	fn tokenize_list_kind(&mut self) -> Result<Vec<Token>>, LexerError> {
		Ok(vec![])
	}

	fn tokenize_quote(&mut self) -> Result<Vec<Token>>, LexerError> {
		Ok(vec![])
	}

	fn tokenize_codeblock(&mut self) -> Result<Vec<Token>>, LexerError> {
		Ok(vec![])
	}

	fn tokenize_table(&mut self) -> Result<Vec<Token>>, LexerError> {
		Ok(vec![])
	}

	fn tokenize_image(&mut self) -> Result<Vec<Token>>, LexerError> {
		Ok(vec![])
	}

	fn tokenize_bold(&mut self) -> Result<Vec<Token>>, LexerError> {
		Ok(vec![])
	}

	fn tokenize_italic(&mut self) -> Result<Vec<Token>>, LexerError> {
		Ok(vec![])
	}

	fn tokenize_link(&mut self) -> Result<Vec<Token>>, LexerError> {
		Ok(vec![])
	}

	fn tokenize_inline_code(&mut self) -> Result<Vec<Token>>, LexerError> {
		Ok(vec![])
	}

	fn tokenize_strikethrough(&mut self) -> Result<Vec<Token>>, LexerError> {
		Ok(vec![])
	}

	fn tokenize_underline(&mut self) -> Result<Vec<Token>>, LexerError> {
		Ok(vec![])
	}

	fn tokenize_newline(&mut self) -> Result<Vec<Token>>, LexerError> {
		Ok(vec![])
	}

	fn tokenize_eof(&mut self) -> Result<Vec<Token>>, LexerError> {
		Ok(vec![])
	}
}

#[derive(Debug, PartialEq)]
pub enum Token {
	// Line Elements
	Section,
	Subsection,
	Subsubsection,
	List(usize),
	Quote,
	Codeblock,
	Table,
	Image,

	// Inline Elements
	Bold,
	Italic,
	Link,
	InlineCode,
	Strikethrough,
	Underline,

	// Other
	Newline,
	EOF,
}






















#[derive(Debug)]
enum LexerError {
    UnexpectedToken(String, String),
    UnexpectedEOF,
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            LexerError::UnexpectedToken(ref s, ref c) =>
                write!(f, "Unexpected token, expected {} got {}", s, c),
            LexerError::UnexpectedEOF => write!(f, "Unexpected end of file"),
        }
    }
}

impl Error for LexerError {}