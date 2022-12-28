#[cfg(test)]
use super::*;
use super::lexer;

#[test]
fn test_load() -> Result<(), Box<dyn std::error::Error>> {
	let mut doc = document::Document::load("./src/tests/test.wg", None).unwrap();
	println!("{}", doc.convert_to_html()?);
	Ok(())
}

#[test]
fn test_lexer() {
	let mut lexer = lexer::Lexer::new(String::from(r#"@ Learning C++

This is a sample learning C++ document. 

/ This is an italics comment. /"#));
	let tokens = lexer.tokenize().unwrap();
	println!("{:?}", tokens);
	println!();
	let mut parser = parser::Parser::new(tokens);
	let parsed = parser.parse().unwrap();
	println!("{:?}", parsed);
}