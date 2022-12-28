use clap::Parser;
use waygum::convert_to_html;

#[derive(Parser, Debug)]
#[clap(name = "waygum", version = "0.1.0", author = "Ashwin Rohit")]
struct Args {
	#[arg(short, long)]
	input: String,

	#[arg(short, long)]
	output: String,

	#[arg(short, long)]
	style: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Args::parse();
	convert_to_html(&args.input, args.style.as_deref(), &args.output)?;
	Ok(())
}