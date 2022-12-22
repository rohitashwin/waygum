use crate::lexer::{Lexer, Token, ListDepth};
#[test]
fn test_advance() {
	let sample = "TTTTTHello, World";
	let mut lexer = Lexer::new(sample);
	let output = lexer.advance_while(|c| return c == 'T');
	assert!(output == "TTTTT");
	assert!(lexer.pos == 5);
}

#[test]
fn test_tokenize() {
	let sample = "Hello, World";
	let mut lexer = Lexer::new(sample);
	let output = lexer.tokenize();
	assert!(output == vec![Token::Text("Hello, World".to_string()), Token::EOF]);
}

#[test]
fn test_heading() {
	let sample = "# Hello, World";
	let mut lexer = Lexer::new(sample);
	let output = lexer.tokenize();
	assert!(output == vec![Token::Section, Token::Text("Hello, World".to_string()), Token::EOF]);
}

#[test]
fn test_subsection() {
	let sample = "## Hello, World";
	let mut lexer = Lexer::new(sample);
	let output = lexer.tokenize();
	assert!(output == vec![Token::Subsection, Token::Text("Hello, World".to_string()), Token::EOF]);
}

#[test]
fn test_subsubsection() {
	let sample = "### Hello, World";
	let mut lexer = Lexer::new(sample);
	let output = lexer.tokenize();
	assert!(output == vec![Token::Subsubsection, Token::Text("Hello, World".to_string()), Token::EOF]);
}

#[test]
fn test_list() {
	let sample = "- Hello, World";
	let mut lexer = Lexer::new(sample);
	let output = lexer.tokenize();
	assert!(output == vec![Token::List(ListDepth(1)), Token::Text("Hello, World".to_string()), Token::EOF]);
}

#[test]
fn test_list2() {
	let sample = "-- Hello, World";
	let mut lexer = Lexer::new(sample);
	let output = lexer.tokenize();
	assert!(output == vec![Token::List(ListDepth(2)), Token::Text("Hello, World".to_string()), Token::EOF]);
}

#[test]
fn test_quote() {
	let sample = "> Hello, World";
	let mut lexer = Lexer::new(sample);
	let output = lexer.tokenize();
	assert!(output == vec![Token::Quote, Token::Text("Hello, World".to_string()), Token::EOF]);
}

#[test]
fn test_bold() {
	let sample = "* Hello, World *";
	let mut lexer = Lexer::new(sample);
	let output = lexer.tokenize();
	println!("{:?}", output);
	assert!(output == vec![Token::Bold, Token::Text("Hello, World".to_string()), Token::Bold, Token::EOF]);
}

#[test]
fn test_italics() {
	let sample = "/ Hello, World /";
	let mut lexer = Lexer::new(sample);
	let output = lexer.tokenize();
	println!("{:?}", output);
	assert!(output == vec![Token::Italics, Token::Text("Hello, World".to_string()), Token::Italics, Token::EOF]);
}

#[test]
fn test_code() {
	let sample = "` Hello, World `";
	let mut lexer = Lexer::new(sample);
	let output = lexer.tokenize();
	println!("{:?}", output);
	assert!(output == vec![Token::Code, Token::Text("Hello, World".to_string()), Token::Code, Token::EOF]);
}

#[test]
fn test_strikethrough() {
	let sample = "~ Hello, World ~";
	let mut lexer = Lexer::new(sample);
	let output = lexer.tokenize();
	println!("{:?}", output);
	assert!(output == vec![Token::Strikethrough, Token::Text("Hello, World".to_string()), Token::Strikethrough, Token::EOF]);
}

#[test]
fn test_sample() {
	let sample = "# This is a section\n## This is a subsection\n- This is a list item\n- This is another list item\n> This is a quote\n*This is bold*\n/This is italics/\n`This is code`\n~This is strikethrough~";
	let mut lexer = Lexer::new(sample);
	let output = lexer.tokenize();
	println!("{:?}", output);
	assert!(output == vec![
		Token::Section,
		Token::Text("This is a section".to_string()),
		Token::Subsection,
		Token::Text("This is a subsection".to_string()),
		Token::List(ListDepth(1)),
		Token::Text("This is a list item".to_string()),
		Token::List(ListDepth(1)),
		Token::Text("This is another list item".to_string()),
		Token::Quote,
		Token::Text("This is a quote".to_string()),
		Token::Bold,
		Token::Text("This is bold".to_string()),
		Token::Bold,
		Token::Italics,
		Token::Text("This is italics".to_string()),
		Token::Italics,
		Token::Code,
		Token::Text("This is code".to_string()),
		Token::Code,
		Token::Strikethrough,
		Token::Text("This is strikethrough".to_string()),
		Token::Strikethrough,
		Token::EOF
	]);
}

#[test]
fn test_sample_2() {
	let sample = "# Hello, World
This is the hello world section of this document.
## Processing
This is the processing section of this document.
Steps to process:
- Step 1
- Step 2 (optional)
- Step 3
-- Step 3.1
-- Step 3.2
--- Step 3.2.1
- Step 4
## Output
This is the output section of this document.
Output:
*Note:* This is a note.
/Warning:/ This is a warning.
~Error:~ This is an error.
`Success!` This is a success message.
";
	let mut lexer = Lexer::new(sample);
	let output = lexer.tokenize();
	println!("{:?}", output);
	assert!(output == vec![
		Token::Section,
		Token::Text("Hello, World".to_string()),
		Token::Text("This is the hello world section of this document.".to_string()),
		Token::Subsection,
		Token::Text("Processing".to_string()),
		Token::Text("This is the processing section of this document.".to_string()),
		Token::Text("Steps to process:".to_string()),
		Token::List(ListDepth(1)),
		Token::Text("Step 1".to_string()),
		Token::List(ListDepth(1)),
		Token::Text("Step 2 (optional)".to_string()),
		Token::List(ListDepth(1)),
		Token::Text("Step 3".to_string()),
		Token::List(ListDepth(2)),
		Token::Text("Step 3.1".to_string()),
		Token::List(ListDepth(2)),
		Token::Text("Step 3.2".to_string()),
		Token::List(ListDepth(3)),
		Token::Text("Step 3.2.1".to_string()),
		Token::List(ListDepth(1)),
		Token::Text("Step 4".to_string()),
		Token::Subsection,
		Token::Text("Output".to_string()),
		Token::Text("This is the output section of this document.".to_string()),
		Token::Text("Output:".to_string()),
		Token::Bold,
		Token::Text("Note:".to_string()),
		Token::Bold,
		Token::Text("This is a note.".to_string()),
		Token::Italics,
		Token::Text("Warning:".to_string()),
		Token::Italics,
		Token::Text("This is a warning.".to_string()),
		Token::Strikethrough,
		Token::Text("Error:".to_string()),
		Token::Strikethrough,
		Token::Text("This is an error.".to_string()),
		Token::Code,
		Token::Text("Success!".to_string()),
		Token::Code,
		Token::Text("This is a success message.".to_string()),
		Token::EOF
	]);
}