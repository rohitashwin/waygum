use std::error::Error;

#[cfg(test)]

use super::super::lexer::*;
use super::{super::parser::*, Codegen};

#[test]
fn sanity() {
	assert_eq!(1, 1);
}

// #[test]
// fn text() -> Result<(), Box<dyn Error>> {
// 	let mut lexer = Lexer::new("Hello, World!");
// 	let tokens = lexer.tokenize();
// 	let mut parser = Parser::new(tokens);
// 	let codegen = Codegen::new();
// 	let ast = parser.parse()?;
// 	println!("{:?}", ast);
// 	let output = codegen.generate(ast)?;
// 	println!("{}", output);
// 	Ok(())
// }