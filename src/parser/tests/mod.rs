#[cfg(test)]
use super::*;
use super::lexer::{ self, * };

#[test]
fn section() -> Result<(), ParseError> {
    let mut lexer = lexer::Lexer::new("# Hello, World!");
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse()?;
    println!("{:?}", parsed);
    assert!(parsed == vec![Element::Section("Hello, World!".to_string())]);
    Ok(())
}

#[test]
fn subsection() -> Result<(), ParseError> {
    let mut lexer = lexer::Lexer::new("## Hello, World!");
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse()?;
    println!("{:?}", parsed);
    assert!(parsed == vec![Element::Subsection("Hello, World!".to_string())]);
    Ok(())
}

#[test]
fn subsubsection() -> Result<(), ParseError> {
    let mut lexer = lexer::Lexer::new("### Hello, World!");
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse()?;
    println!("{:?}", parsed);
    assert!(parsed == vec![Element::Subsubsection("Hello, World!".to_string())]);
    Ok(())
}

#[test]
fn section_subsection() -> Result<(), ParseError> {
    let mut lexer = lexer::Lexer::new(
        "# Hello, World! ## Hello, World!\n## Hello, World! ### Hello, World!"
    );
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse()?;
    println!("{:?}", parsed);
    assert!(
        parsed ==
            vec![
                Element::Section("Hello, World! ## Hello, World!".to_string()),
                Element::Subsection("Hello, World! ### Hello, World!".to_string())
            ]
    );
    Ok(())
}

#[test]
fn basic_list() -> Result<(), ParseError> {
    let mut lexer = lexer::Lexer::new("- Hello, World!");
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse()?;
    println!("{:?}", parsed);
    assert!(parsed == vec![Element::List(vec![Element::ListItem("Hello, World!".to_string())])]);
    Ok(())
}

#[test]
fn two_level_list() -> Result<(), ParseError> {
    let mut lexer = lexer::Lexer::new("- Line 1\n- Line 2");
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse()?;
    println!("{:?}", parsed);
    assert!(
        parsed ==
            vec![
                Element::List(
                    vec![
                        Element::ListItem("Line 1".to_string()),
                        Element::ListItem("Line 2".to_string())
                    ]
                )
            ]
    );
    Ok(())
}

#[test]
fn different_level_list() -> Result<(), ParseError> {
    let mut lexer = lexer::Lexer::new("- Line 1\n-- Line 2");
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse()?;
    println!("{:?}", parsed);
    assert!(
        parsed ==
            vec![
                Element::List(
                    vec![
                        Element::ListItem("Line 1".to_string()),
                        Element::List(vec![Element::ListItem("Line 2".to_string())])
                    ]
                )
            ]
    );
    Ok(())
}

#[test]
fn list_with_text() -> Result<(), ParseError> {
    let mut lexer = lexer::Lexer::new("- Line 1\n-- Line 2\n# Heading");
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse()?;
    println!("{:?}", parsed);
    assert!(
        parsed ==
            vec![
                Element::List(
                    vec![
                        Element::ListItem("Line 1".to_string()),
                        Element::List(vec![Element::ListItem("Line 2".to_string())])
                    ]
                ),
                Element::Section("Heading".to_string())
            ]
    );
    Ok(())
}

#[test]
fn list_text_list() -> Result<(), ParseError> {
    let mut lexer = lexer::Lexer::new("- Line 1\n-- Line 2\n# Heading\n- Line 3");
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse()?;
    println!("{:?}", parsed);
    assert!(
        parsed ==
            vec![
                Element::List(
                    vec![
                        Element::ListItem("Line 1".to_string()),
                        Element::List(vec![Element::ListItem("Line 2".to_string())])
                    ]
                ),
                Element::Section("Heading".to_string()),
                Element::List(vec![Element::ListItem("Line 3".to_string())])
            ]
    );
    Ok(())
}

#[test]
fn list_text_list_text() -> Result<(), ParseError> {
	let mut lexer = lexer::Lexer::new("- Line 1\n-- Line 2\n# Heading\n- Line 3\n# Heading 2");
	let tokens = lexer.tokenize();
	let mut parser = Parser::new(tokens);
	let parsed = parser.parse()?;
	println!("{:?}", parsed);
	assert!(
		parsed ==
			vec![
				Element::List(
					vec![
						Element::ListItem("Line 1".to_string()),
						Element::List(vec![Element::ListItem("Line 2".to_string())])
					]
				),
				Element::Section("Heading".to_string()),
				Element::List(vec![Element::ListItem("Line 3".to_string())]),
				Element::Section("Heading 2".to_string())
			]
	);
	Ok(())
}

#[test]
fn quote() -> Result<(), ParseError> {
	let mut lexer = lexer::Lexer::new("> Hello, World!");
	let tokens = lexer.tokenize();
	let mut parser = Parser::new(tokens);
	let parsed = parser.parse()?;
	println!("{:?}", parsed);
	assert!(parsed == vec![Element::Quote("Hello, World!".to_string())]);
	Ok(())
}

#[test]
fn quote_with_text() -> Result<(), ParseError> {
	let mut lexer = lexer::Lexer::new("> Hello, World!\n# Heading");
	let tokens = lexer.tokenize();
	let mut parser = Parser::new(tokens);
	let parsed = parser.parse()?;
	println!("{:?}", parsed);
	assert!(
		parsed ==
			vec![
				Element::Quote("Hello, World!".to_string()),
				Element::Section("Heading".to_string())
			]
	);
	Ok(())
}

#[test]
fn quote_with_text_quote() -> Result<(), ParseError> {
	let mut lexer = lexer::Lexer::new("> Hello, World!\n# Heading\n> Hello, World!");
	let tokens = lexer.tokenize();
	let mut parser = Parser::new(tokens);
	let parsed = parser.parse()?;
	println!("{:?}", parsed);
	assert!(
		parsed ==
			vec![
				Element::Quote("Hello, World!".to_string()),
				Element::Section("Heading".to_string()),
				Element::Quote("Hello, World!".to_string())
			]
	);
	Ok(())
}

#[test]
fn styles() -> Result<(), ParseError> {
	let mut lexer = lexer::Lexer::new("*Hello, World!*");
	let tokens = lexer.tokenize();
	let mut parser = Parser::new(tokens);
	let parsed = parser.parse()?;
	println!("{:?}", parsed);
	assert!(parsed == vec![Element::Bold("Hello, World!".to_string())]);
	Ok(())
}

#[test]
fn sample_document() -> Result<(), ParseError> {
	let mut lexer = lexer::Lexer::new(
		r#"
# Hello, World!
This is a simple document written in the custom markdown language.
## Tests
This is a test line that should be parsed as a paragraph.
- This is a list item
-- This is a sub-list item
- This is another list item
> This is a quote
*This is bold text*
/This is italic text/
~This is strikethrough text~
`This is code text`
"#,
	);
	let tokens = lexer.tokenize();
	let mut parser = Parser::new(tokens);
	let parsed = parser.parse()?;
	println!("{:?}", parsed);
	assert!(
		parsed ==
			vec![
				Element::Section("Hello, World!".to_string()),
				Element::Text("This is a simple document written in the custom markdown language.".to_string()),
				Element::Section("Tests".to_string()),
				Element::Text("This is a test line that should be parsed as a paragraph.".to_string()),
				Element::List(
					vec![
						Element::ListItem("This is a list item".to_string()),
						Element::List(vec![Element::ListItem("This is a sub-list item".to_string())])
					]
				),
				Element::List(vec![Element::ListItem("This is another list item".to_string())]),
				Element::Quote("This is a quote".to_string()),
				Element::Bold("This is bold text".to_string()),
				Element::Italics("This is italic text".to_string()),
				Element::Strikethrough("This is strikethrough text".to_string()),
				Element::Code("This is code text".to_string())
			]
	);
	Ok(())
}