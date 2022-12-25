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
    assert!(
        parsed == vec![Element::Section(Text(vec![TextElement::Raw("Hello, World!".to_string())]))]
    );
    Ok(())
}

#[test]
fn subsection() -> Result<(), ParseError> {
    let mut lexer = lexer::Lexer::new("## Hello, World!");
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse()?;
    println!("{:?}", parsed);
    assert!(
        parsed ==
            vec![Element::Subsection(Text(vec![TextElement::Raw("Hello, World!".to_string())]))]
    );
    Ok(())
}

#[test]
fn section_subsection() -> Result<(), ParseError> {
    let mut lexer = lexer::Lexer::new("# Hello, World!\n## Hello, World!");
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse()?;

    println!("{:?}", parsed);
    assert!(
        parsed ==
            vec![
                Element::Section(Text(vec![TextElement::Raw("Hello, World!".to_string())])),
                Element::Subsection(Text(vec![TextElement::Raw("Hello, World!".to_string())]))
            ]
    );
    Ok(())
}

#[test]
fn section_subsection_text() -> Result<(), ParseError> {
    let mut lexer = lexer::Lexer::new("# Hello, World!\n## Hello, World!\nHello, World!");
    let tokens = lexer.tokenize();
    println!("{:?}", tokens);
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse()?;
    println!("{:?}", parsed);
    assert!(
        parsed ==
            vec![
                Element::Section(Text(vec![TextElement::Raw("Hello, World!".to_string())])),
                Element::Subsection(Text(vec![TextElement::Raw("Hello, World!".to_string())])),
                Element::Paragraph(Text(vec![TextElement::Raw("Hello, World!".to_string())]))
            ]
    );
    Ok(())
}

#[test]
fn list() -> Result<(), ParseError> {
    let mut lexer = lexer::Lexer::new("- Line 1");
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse()?;
    println!("{:?}", parsed);
    assert!(
        parsed ==
            vec![
                Element::List(
                    vec![Element::ListItem(Text(vec![TextElement::Raw("Line 1".to_string())]))]
                )
            ]
    );
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
                        Element::ListItem(Text(vec![TextElement::Raw("Line 1".to_string())])),
                        Element::ListItem(Text(vec![TextElement::Raw("Line 2".to_string())]))
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
                        Element::ListItem(Text(vec![TextElement::Raw("Line 1".to_string())])),
                        Element::List(
                            vec![
                                Element::ListItem(
                                    Text(vec![TextElement::Raw("Line 2".to_string())])
                                )
                            ]
                        )
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
                        Element::ListItem(Text(vec![TextElement::Raw("Line 1".to_string())])),
                        Element::List(
                            vec![
                                Element::ListItem(
                                    Text(vec![TextElement::Raw("Line 2".to_string())])
                                )
                            ]
                        )
                    ]
                ),
                Element::Section(Text(vec![TextElement::Raw("Heading".to_string())]))
            ]
    );
    Ok(())
}

#[test]
fn list_text_list() -> Result<(), ParseError> {
    let mut lexer = lexer::Lexer::new("- Line 1\n# Heading\n-- Line 2");
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse()?;
    println!("{:?}", parsed);

    assert!(
        parsed ==
            vec![
                Element::List(
                    vec![Element::ListItem(Text(vec![TextElement::Raw("Line 1".to_string())]))]
                ),
                Element::Section(Text(vec![TextElement::Raw("Heading".to_string())])),
                Element::List(
                    vec![
                        Element::List(
                            vec![
                                Element::ListItem(
                                    Text(vec![TextElement::Raw("Line 2".to_string())])
                                )
                            ]
                        )
                    ]
                )
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
                        Element::ListItem(Text(vec![TextElement::Raw("Line 1".to_string())])),
                        Element::List(
                            vec![
                                Element::ListItem(
                                    Text(vec![TextElement::Raw("Line 2".to_string())])
                                )
                            ]
                        )
                    ]
                ),
                Element::Section(Text(vec![TextElement::Raw("Heading".to_string())])),
                Element::List(
                    vec![Element::ListItem(Text(vec![TextElement::Raw("Line 3".to_string())]))]
                ),
                Element::Section(Text(vec![TextElement::Raw("Heading 2".to_string())]))
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
    assert!(
        parsed == vec![Element::Quote(Text(vec![TextElement::Raw("Hello, World!".to_string())]))]
    );
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
                Element::Quote(Text(vec![TextElement::Raw("Hello, World!".to_string())])),
                Element::Section(Text(vec![TextElement::Raw("Heading".to_string())]))
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
                Element::Quote(Text(vec![TextElement::Raw("Hello, World!".to_string())])),
                Element::Section(Text(vec![TextElement::Raw("Heading".to_string())])),
                Element::Quote(Text(vec![TextElement::Raw("Hello, World!".to_string())]))
            ]
    );
    Ok(())
}

#[test]
fn bold() -> Result<(), ParseError> {
    let mut lexer = lexer::Lexer::new("*Hello, World!*");
    let tokens = lexer.tokenize();
    dbg!(&tokens);
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse()?;
    println!("{:?}", parsed);
    assert!(
        parsed ==
            vec![Element::Paragraph(Text(vec![TextElement::Bold("Hello, World!".to_string())]))]
    );
    Ok(())
}

#[test]
fn italics() -> Result<(), ParseError> {
    let mut lexer = lexer::Lexer::new("/Hello, World!/\n# Heading");
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse()?;
    println!("{:?}", parsed);
    assert!(
        parsed ==
            vec![
                Element::Paragraph(Text(vec![TextElement::Italics("Hello, World!".to_string())])),
                Element::Section(Text(vec![TextElement::Raw("Heading".to_string())]))
            ]
    );
    Ok(())
}

#[test]
fn code() -> Result<(), ParseError> {
    let mut lexer = lexer::Lexer::new("`Hello, World!`");
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse()?;
    println!("{:?}", parsed);
    assert!(
        parsed ==
            vec![Element::Paragraph(Text(vec![TextElement::Code("Hello, World!".to_string())]))]
    );
    Ok(())
}

#[test]
fn strikethrough() -> Result<(), ParseError> {
    let mut lexer = lexer::Lexer::new("~Hello, World!~");
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse()?;
    println!("{:?}", parsed);
    assert!(
        parsed ==
            vec![
                Element::Paragraph(
                    Text(vec![TextElement::Strikethrough("Hello, World!".to_string())])
                )
            ]
    );
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
"#
    );
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let parsed = parser.parse()?;
    println!("{:?}", parsed);
    assert!(
        parsed ==
            vec![
                Element::Section(Text(vec![TextElement::Raw("Hello, World!".to_string())])),
                Element::Paragraph(
                    Text(
                        vec![
                            TextElement::Raw(
                                "This is a simple document written in the custom markdown language.".to_string()
                            )
                        ]
                    )
                ),
                Element::Subsection(Text(vec![TextElement::Raw("Tests".to_string())])),
                Element::Paragraph(
                    Text(
                        vec![
                            TextElement::Raw(
                                "This is a test line that should be parsed as a paragraph.".to_string()
                            )
                        ]
                    )
                ),
                Element::List(
                    vec![
                        Element::ListItem(
                            Text(vec![TextElement::Raw("This is a list item".to_string())])
                        ),
                        Element::List(
                            vec![
                                Element::ListItem(
                                    Text(
                                        vec![
                                            TextElement::Raw("This is a sub-list item".to_string())
                                        ]
                                    )
                                )
                            ]
                        ),
                        Element::ListItem(
                            Text(vec![TextElement::Raw("This is another list item".to_string())])
                        )
                    ]
                ),
                Element::Quote(Text(vec![TextElement::Raw("This is a quote".to_string())])),
                Element::Paragraph(Text(vec![TextElement::Bold("This is bold text".to_string())])),
                Element::Paragraph(
                    Text(vec![TextElement::Italics("This is italic text".to_string())])
                ),
                Element::Paragraph(
                    Text(vec![TextElement::Strikethrough("This is strikethrough text".to_string())])
                ),
                Element::Paragraph(Text(vec![TextElement::Code("This is code text".to_string())]))
            ]
    );
    Ok(())
}