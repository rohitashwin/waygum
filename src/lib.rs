mod lexer;
mod parser;
mod html_impl;
mod document;
mod tests;
mod html;

use std::fs::File;
use std::io::prelude::*;

pub fn convert_to_html(input_file: &str, style_file: Option<&str>, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
	let mut doc = document::Document::load(input_file, style_file)?;
	let html = doc.convert_to_html()?;
	let mut outfile = File::create(output_file)?;
	outfile.write_all(html.as_bytes())?;
	Ok(())
}