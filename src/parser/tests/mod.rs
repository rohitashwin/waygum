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