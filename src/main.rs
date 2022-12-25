extern crate waygum;

use waygum::convert_to_html;
use clap::Parser;

#[derive(Parser)]
struct Opts {
	#[clap(short, long, help = "Path to input file")]
	input: String,
	#[clap(short, long, help = "Path to stylesheet")]
	stylesheet: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let opts: Opts = Opts::parse();
	let input = std::fs::read_to_string(opts.input)?;
	let stylesheet = std::fs::read_to_string(opts.stylesheet)?;
	let html = convert_to_html(&input, &stylesheet)?;
	Ok(())
}