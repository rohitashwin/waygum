pub struct Lexer {
    input: Vec<char>,
}

pub enum CodeLang {
	Rust,
	C,
	Cpp,
	Python,
	Java,
	Javascript,
	Html,
	Css,
	Bash,
	Markdown,
	Unknown,
}

pub enum Token {
    SectionMarker,
	SubsectionMarker,
	SubsubsectionMarker,
    Text(String),
    Codeblock(CodeLang),
    Quote,
    List {
        depth: usize,
		content: String,
    },
    Bold,
    Italic,
    Link,
    InlineCode,
    Image,
    Table(usize),
    Newline,
    EOF,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        Lexer {
            input: input.chars().rev().collect(),
        }
    }

    fn next(&mut self) -> Option<&char> {
        self.input.last()
    }

    fn consume(&mut self) -> Option<char> {
        self.input.pop()
    }

    fn consume_while<F>(&mut self, mut f: F) -> String where F: FnMut(&char) -> bool {
        let mut result = String::new();
        while let Some(c) = self.next() {
            if f(c) {
                result.push(self.consume().unwrap());
            } else {
                break;
            }
        }
        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace() && *c != '\n');
    }

	fn consume_whitespace_and_newline(&mut self) {
		self.consume_whitespace();
		if let Some(c) = self.next() {
			if *c == '\n' {
				self.consume();
			}
		}
	}

	fn read_line(&mut self) -> String {
		let mut line = String::new();
		while let Some(c) = self.next() {
			if *c == '\n' {
				self.consume();
				break;
			}
			line.push(self.consume().unwrap());
		}
		line
	}

    fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = vec![];
        while let Some(c) = self.next() {
            match c {
                '@' => tokens.extend(self.tokenize_section()?),
                '>' => tokens.extend(self.tokenize_quote()?),
                '-' => tokens.extend(self.tokenize_list()?),
                '$' => tokens.extend(self.tokenize_codeblock()?),
                '!' => tokens.extend(self.tokenize_image()?),
                '|' => tokens.extend(self.tokenize_table()?),
                '\n' => tokens.extend(Token::Newline),
                _ => tokens.extend(self.tokenize_text()?),
            }
        }
		tokens.push(Token::EOF);
		Ok(tokens)
    }

	fn tokenize_section(&mut self) -> Result<Vec<Token>, LexerError> {
		let mut tokens = vec![];
		let section_markers = self.consume_while(|ch| *ch == '@');
		match section_markers.len() {
			1 => tokens.push(Token::SectionMarker),
			2 => tokens.push(Token::SubsectionMarker),
			3 => tokens.push(Token::SubsubsectionMarker),
			_ => return Err(LexerError::UnexpectedToken("Upto 3 @".to_string(), section_markers)),
		}
		let text = self.tokenize_text()?;
		tokens.extend(text);
		Ok(tokens)
	}

	fn tokenize_quote(&mut self) -> Result<Vec<Token>, LexerError> {
		let mut tokens = vec![];
		let quote_markers = self.consume_while(|ch| *ch == '>');
		if quote_markers.len() > 1 {
			return Err(LexerError::UnexpectedToken("Upto 1 >".to_string(), quote_markers));
		}
		tokens.push(Token::Quote);
		let text = self.tokenize_text()?;
		tokens.extend(text);
		Ok(tokens)
	}

	fn tokenize_list(&mut self) -> Result<Vec<Token>, LexerError> {
		let mut tokens = vec![];
		let list_markers = self.consume_while(|ch| *ch == '-');
		if list_markers.len() > 1 {
			return Err(LexerError::UnexpectedToken("Upto 1 -".to_string(), list_markers));
		}
		tokens.push(Token::List {
			depth: list_markers.len(),
		});
		let text = self.tokenize_text()?;
		tokens.extend(text);
		Ok(tokens)
	}

	fn tokenize_codeblock(&mut self) -> Result<Vec<Token>, LexerError> {
		let mut tokens = vec![];
		let codeblock_markers = self.consume_while(|ch| *ch == '$');
		if codeblock_markers.len() == 1 {
			return self.tokenize_text();
		}
		if codeblock_markers.len() == 3 {
			self.consume_whitespace();
			let lang = self.consume_while(|ch| ch.is_alphabetic());
			let lang = match lang.as_str() {
				"rust" => CodeLang::Rust,
				"c" => CodeLang::C,
				"cpp" | "c++" => CodeLang::Cpp,
				"python" => CodeLang::Python,
				"java" => CodeLang::Java,
				"js" | "javascript" => CodeLang::Javascript,
				"html" => CodeLang::Html,
				"css" => CodeLang::Css,
				"bash" | "sh" => CodeLang::Bash,
				"md" | "markdown" => CodeLang::Markdown,
				_ => CodeLang::Unknown,
			};
			tokens.push(Token::Codeblock(lang));
			self.consume_whitespace_and_newline();
			let mut code = String::new();
			// handle EOF and all the lines to code until we encounter 3 $
			while let Some(c) = self.next() {
				let line = self.read_line();
				if line == "$$$" {
					break;
				}
				code.push_str(&line);
			}
			tokens.push(Token::Text(code));
		}
		Err(LexerError::UnexpectedToken("1 for inline codeblock, 3 for multiline codeblock".to_string(), codeblock_markers))
	}

	fn tokenize_image(&mut self) -> Result<Vec<Token>, LexerError> {
		let mut tokens = vec![];
		let image_markers = self.consume_while(|ch| *ch == '!');
		if image_markers.len() > 1 {
			return Err(LexerError::Unexpected
}

#[derive(Debug)]
enum LexerError {
    UnexpectedToken(String, String),
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LexerError::UnexpectedToken(expected, got) =>
                write!(f, "Unexpected token: expected {}, got {}", expected, got),
        }
    }
}

impl std::error::Error for LexerError {}