mod lexer;
mod parser;
mod codegen;

use lexer::Lexer;
use parser::Parser;
use codegen::Codegen;

use std::error::Error;

pub fn convert_to_html(filename: &str, stylsheet_path: &str) -> Result<String, Box<dyn Error>> {
	let input = std::fs::read_to_string(filename)?;
	let stylesheet = std::fs::read_to_string(stylsheet_path)?;
	let mut lexer = Lexer::new(&input);
	let tokens = lexer.tokenize();
	let mut parser = Parser::new(tokens);
	let ast = parser.parse()?;
	let codegen = Codegen::new();
	let output = codegen.generate(ast, stylesheet)?;
	std::fs::write("output.html", output.clone())?;
	Ok(output)
}