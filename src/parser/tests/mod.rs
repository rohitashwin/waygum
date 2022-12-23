use super::*;
use super::lexer::{ self, * };

#[test]
fn test_section() -> Result<(), ParseError> {
	let mut lexer = lexer::Lexer::new("# Hello, World!");
	let tokens = lexer.tokenize();
	let mut parser = Parser::new(tokens);
	let parsed = parser.parse()?;
	println!("{:?}", parsed);
	assert!(parsed == vec![Element::Section("Hello, World!".to_string())]);
	Ok(())
}

#[test]
fn test_subsection() -> Result<(), ParseError> {
	let mut lexer = lexer::Lexer::new("## Hello, World!");
	let tokens = lexer.tokenize();
	let mut parser = Parser::new(tokens);
	let parsed = parser.parse()?;
	println!("{:?}", parsed);
	assert!(parsed == vec![Element::Subsection("Hello, World!".to_string())]);
	Ok(())
}

#[test]
fn test_subsubsection() -> Result<(), ParseError> {
	let mut lexer = lexer::Lexer::new("### Hello, World!");
	let tokens = lexer.tokenize();
	let mut parser = Parser::new(tokens);
	let parsed = parser.parse()?;
	println!("{:?}", parsed);
	assert!(parsed == vec![Element::Subsubsection("Hello, World!".to_string())]);
	Ok(())
}

#[test]
fn test_section_subsection() -> Result<(), ParseError> {
	let mut lexer = lexer::Lexer::new("# Hello, World! ## Hello, World!\n## Hello, World! ### Hello, World!");
	let tokens = lexer.tokenize();
	let mut parser = Parser::new(tokens);
	let parsed = parser.parse()?;
	println!("{:?}", parsed);
	assert!(parsed == vec![
		Element::Section("Hello, World! ## Hello, World!".to_string()),
		Element::Subsection("Hello, World! ### Hello, World!".to_string())
	]);
	Ok(())
}
