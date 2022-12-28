#[cfg(test)]
use super::*;

#[test]
fn test_load() -> Result<(), Box<dyn std::error::Error>> {
	let mut doc = document::Document::load("./src/tests/test.wg", None).unwrap();
	println!("{}", doc.convert_to_html()?);
	Ok(())
}