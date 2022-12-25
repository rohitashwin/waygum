extern crate waygum;

use waygum::convert_to_html;
use clap::Parser;

#[derive(Parser)]
struct Opts {
	#[clap(short, long)]
	input: String,
	#[clap(short, long)]
	stylesheet: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	convert_to_html("hello.wg", "style.css")?;
	Ok(())
}