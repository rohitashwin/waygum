use std::env;
use std::fs::File;
use std::io::{Read, Write};
mod lexer;
mod parser;
mod generator;
use lexer::Lexer;
use parser::Parser;
use generator::to_html;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = env::args().nth(1).ok_or("missing file path argument")?;
    let output_path = env::args().nth(2).ok_or("missing output file path argument")?;
    let mut file = File::open(file_path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);
    let document = parser.parse()?;

    let html = document
        .elements
        .iter()
        .map(to_html)
        .collect::<String>();

    let mut output_file = File::create(output_path)?;
    output_file.write_all(html.as_bytes())?;
    Ok(())
}