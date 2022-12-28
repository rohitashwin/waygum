use super::html::ToHtml;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

pub struct Document<'a> {
	input: String,
	converted: String,
	title: &'a str,
	style: Option<String>,
}

impl<'a> Document<'a> {
	pub fn load(path: &'a str, style: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {
		let mut inpfile = File::open(path)?;
		let mut file_contents = String::new();
		inpfile.read_to_string(&mut file_contents)?;
		let file_re = Regex::new(r"([^/]*)(?:\.)[^.]*?$").unwrap();
		let file_name = file_re.captures(path).unwrap().get(1).unwrap().as_str();
		let style_contents = match style {
			Some(style) => {
				let mut stylefile = File::open(style)?;
				let mut style_contents = String::new();
				stylefile.read_to_string(&mut style_contents)?;
				Some(style_contents)
			}
			None => None,
		};
		Ok(Self {
			input: file_contents,
			converted: String::new(),
			title: file_name,
			style: style_contents,
		})
	}

	pub fn convert_to_html(&mut self) -> Result<String, Box<dyn std::error::Error>> {
		let html_contents = self.to_html();
		self.converted = html_contents;
		Ok(self.converted.clone())
	}
}

impl<'a> ToHtml for Document<'a> {
	fn to_html(&self) -> String {
		let mut html = String::new();
		html.push_str(
			r#"<!DOCTYPE html>
<html>
	<head>
		<title>"#,
		);
		html.push_str(self.title);
		html.push_str(r#"</title>
	<style>"#);
		if let Some(style) = &self.style {
			html.push_str(style);
		}
		html.push_str(r#"</style>
	</head>
	<body>
		<div id="content">"#);
		html.push_str(&self.input);
		html.push_str(r#"</div>
	</body>
</html>"#);
		html
	}
}