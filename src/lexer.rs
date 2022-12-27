use lazy_static::lazy_static;
use fancy_regex::Regex as FancyRegex;

lazy_static! {
    static ref SECTION: FancyRegex = FancyRegex::new(r#"^@(?!@)\s*(\S.*)$"#).unwrap();
    static ref SUBSECTION: FancyRegex = FancyRegex::new(r#"^@@(?!@)\s*(\S.*)$"#).unwrap();
    static ref SUBSUBSECTION: FancyRegex = FancyRegex::new(r#"^@@@(?!@)\s*(\S.*)$"#).unwrap();
    static ref LIST: FancyRegex = FancyRegex::new(r#"(^-+)\s*(\S.*)$"#).unwrap();
    static ref QUOTE: FancyRegex = FancyRegex::new(r#"^>(?!>)\s*(\S.*)$"#).unwrap();
    static ref TABLE: FancyRegex = FancyRegex::new(r#"^\|(.*)\|$"#).unwrap();
    static ref IMAGE: FancyRegex = FancyRegex::new(r#"^!\[(.*)\]\((.*)\)$"#).unwrap();
    static ref CODEBLOCK: FancyRegex = FancyRegex::new(r"^\$\$\$").unwrap();
    static ref TEXT: FancyRegex = FancyRegex::new(r#"^(.*)$"#).unwrap();
    static ref BUTTON: FancyRegex = FancyRegex::new(r#"^#!\[([^\]]*)\]\{([^\}]*)\}$"#).unwrap();
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Section(String),
    Subsection(String),
    Subsubsection(String),
    List {
        depth: usize,
        text: String,
    },
    Quote(String),
    Table(Vec<String>),
    Image {
        caption: String,
        path: String,
    },
    Codeblock(String),
    Button {
        text: String,
        link: String,
    },
    Text(String),
    Newline,
    EOF,
}

pub struct Lexer {
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    pub fn tokenize(&self) -> Result<Vec<Token>, fancy_regex::Error> {
        let mut tokens = Vec::new();
        let mut lines = self.input.lines();
        while let Some(line) = lines.next() {
            if let Some(captures) = SECTION.captures(line)? {
                tokens.push(Token::Section(captures.get(1).unwrap().as_str().to_string()));
            } else if let Some(captures) = SUBSECTION.captures(line)? {
                tokens.push(Token::Subsection(captures.get(1).unwrap().as_str().to_string()));
            } else if let Some(captures) = SUBSUBSECTION.captures(line)? {
                tokens.push(Token::Subsubsection(captures.get(1).unwrap().as_str().to_string()));
            } else if let Some(captures) = LIST.captures(line)? {
                let depth = captures.get(1).unwrap().as_str().len();
                tokens.push(Token::List {
                    depth,
                    text: captures.get(2).unwrap().as_str().to_string(),
                });
            } else if let Some(captures) = QUOTE.captures(line)? {
                tokens.push(Token::Quote(captures.get(1).unwrap().as_str().to_string()));
            } else if let Some(captures) = TABLE.captures(line)? {
                let mut line = captures.get(1).unwrap().as_str();
                let column_items = line
                    .split('|')
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<String>>();
                tokens.push(Token::Table(column_items));
            } else if let Some(captures) = IMAGE.captures(line)? {
                tokens.push(Token::Image {
                    caption: captures.get(1).unwrap().as_str().to_string(),
                    path: captures.get(2).unwrap().as_str().to_string(),
                });
            } else if let Some(_) = CODEBLOCK.captures(line)? {
                let mut codeblock = String::new();
                while let Some(line) = lines.next() {
                    if line == "$$$" {
                        break;
                    }
                    codeblock.push_str(line);
                    codeblock.push_str("\n");
                }
                tokens.push(Token::Codeblock(codeblock));
            } else if let Some(captures) = BUTTON.captures(line)? {
                tokens.push(Token::Button {
                    text: captures.get(1).unwrap().as_str().to_string(),
                    link: captures.get(2).unwrap().as_str().to_string(),
                });
            } else if let Some(captures) = TEXT.captures(line)? {
                let text = captures.get(1).unwrap().as_str().trim().to_string();
                if text.is_empty() {
                    continue;
                }
                tokens.push(Token::Text(text));
            } else {
                tokens.push(Token::Newline);
            }
        }
        tokens.push(Token::EOF);
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section() -> Result<(), fancy_regex::Error> {
        let input = String::from("@ Section");
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        assert_eq!(tokens, vec![Token::Section(String::from("Section")), Token::EOF]);
        Ok(())
    }

    #[test]
    fn subsection() -> Result<(), fancy_regex::Error> {
        let input = String::from("@@ Subsection");
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        assert_eq!(tokens, vec![Token::Subsection(String::from("Subsection")), Token::EOF]);
        Ok(())
    }

    #[test]
    fn subsubsection() -> Result<(), fancy_regex::Error> {
        let input = String::from("@@@ Subsubsection");
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        assert_eq!(tokens, vec![Token::Subsubsection(String::from("Subsubsection")), Token::EOF]);
        Ok(())
    }

    #[test]
    fn list() -> Result<(), fancy_regex::Error> {
        let input = String::from("- List");
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        assert_eq!(tokens, vec![Token::List { depth: 1, text: String::from("List") }, Token::EOF]);
        Ok(())
    }

    #[test]
    fn quote() -> Result<(), fancy_regex::Error> {
        let input = String::from("> Quote");
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        assert_eq!(tokens, vec![Token::Quote(String::from("Quote")), Token::EOF]);
        Ok(())
    }

    #[test]
    fn table() -> Result<(), fancy_regex::Error> {
        let input = String::from("| Col1 | Col2 | Col3 |");
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        assert_eq!(
            tokens,
            vec![
                Token::Table(
                    vec![String::from("Col1"), String::from("Col2"), String::from("Col3")]
                ),
                Token::EOF
            ]
        );
        Ok(())
    }

    #[test]
    fn image() -> Result<(), fancy_regex::Error> {
        let input = String::from("![Caption](path/to/image)");
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        assert_eq!(
            tokens,
            vec![
                Token::Image {
                    caption: String::from("Caption"),
                    path: String::from("path/to/image"),
                },
                Token::EOF
            ]
        );
        Ok(())
    }

    #[test]
    fn codeblock() -> Result<(), fancy_regex::Error> {
        let input = String::from(
            "$$$\n#include<iostream>\nint main() {\n\tstd::cout << \"Hello World!\" << std::endl;\n}\n$$$"
        );
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        assert_eq!(
            tokens,
            vec![
                Token::Codeblock(
                    String::from(
                        "#include<iostream>\nint main() {\n\tstd::cout << \"Hello World!\" << std::endl;\n}\n"
                    )
                ),
                Token::EOF
            ]
        );
        Ok(())
    }

    #[test]
    fn text() -> Result<(), fancy_regex::Error> {
        let input = String::from("Text");
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        assert_eq!(tokens, vec![Token::Text(String::from("Text")), Token::EOF]);
        Ok(())
    }

    #[test]
    fn text_with_newline() -> Result<(), fancy_regex::Error> {
        let input = String::from("Text\n");
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        assert_eq!(tokens, vec![Token::Text(String::from("Text")), Token::EOF]);
        Ok(())
    }

    #[test]
    fn text_with_newline_and_space() -> Result<(), fancy_regex::Error> {
        let input = String::from("Text\n ");
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        assert_eq!(tokens, vec![Token::Text(String::from("Text")), Token::EOF]);
        Ok(())
    }

    #[test]
    fn text_with_newline_and_tab() -> Result<(), fancy_regex::Error> {
        let input = String::from("Text\n\t");
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        assert_eq!(tokens, vec![Token::Text(String::from("Text")), Token::EOF]);
        Ok(())
    }

    #[test]
    fn text_with_newline_and_space_and_tab() -> Result<(), fancy_regex::Error> {
        let input = String::from("Text\n \t");
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        assert_eq!(tokens, vec![Token::Text(String::from("Text")), Token::EOF]);
        Ok(())
    }

    #[test]
    fn text_with_newline_and_tab_and_space() -> Result<(), fancy_regex::Error> {
        let input = String::from("Text\n\t ");
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        assert_eq!(tokens, vec![Token::Text(String::from("Text")), Token::EOF]);
        Ok(())
    }

    #[test]
    fn text_with_newline_and_tab_and_space_and_text() -> Result<(), fancy_regex::Error> {
        let input = String::from("Text\n\t Text");
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        assert_eq!(
            tokens,
            vec![Token::Text(String::from("Text")), Token::Text(String::from("Text")), Token::EOF]
        );
        Ok(())
    }

    #[test]
    fn text_with_newline_and_tab_and_space_and_text_and_newline() -> Result<
        (),
        fancy_regex::Error
    > {
        let input = String::from("Text\n\t Text\n");
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        assert_eq!(
            tokens,
            vec![Token::Text(String::from("Text")), Token::Text(String::from("Text")), Token::EOF]
        );
        Ok(())
    }

    #[test]
    fn basic_all() -> Result<(), fancy_regex::Error> {
        let input = String::from(
            "@ Heading\n\nText\n\n> Quote\n\n| Col1 | Col2 | Col3 |\n\n![Caption](path/to/image)\n\n$$$\n#include<iostream>\nint main() {\n\tstd::cout << \"Hello World!\" << std::endl;\n}\n$$$"
        );
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        assert_eq!(
            tokens,
            vec![
                Token::Section(String::from("Heading")),
                Token::Text(String::from("Text")),
                Token::Quote(String::from("Quote")),
                Token::Table(
                    vec![String::from("Col1"), String::from("Col2"), String::from("Col3")]
                ),
                Token::Image {
                    caption: String::from("Caption"),
                    path: String::from("path/to/image"),
                },
                Token::Codeblock(
                    String::from(
                        "#include<iostream>\nint main() {\n\tstd::cout << \"Hello World!\" << std::endl;\n}\n"
                    )
                ),
                Token::EOF
            ]
        );
        Ok(())
    }

    #[test]
    fn basic_all_with_newline() -> Result<(), fancy_regex::Error> {
        let input = String::from(
            "@ Heading\n\nText\n\n> Quote\n\n| Col1 | Col2 | Col3 |\n\n![Caption](path/to/image)\n\n$$$\n#include<iostream>\nint main() {\n\tstd::cout << \"Hello World!\" << std::endl;\n}\n$$$\n"
        );
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        assert_eq!(
            tokens,
            vec![
                Token::Section(String::from("Heading")),
                Token::Text(String::from("Text")),
                Token::Quote(String::from("Quote")),
                Token::Table(
                    vec![String::from("Col1"), String::from("Col2"), String::from("Col3")]
                ),
                Token::Image {
                    caption: String::from("Caption"),
                    path: String::from("path/to/image"),
                },
                Token::Codeblock(
                    String::from(
                        "#include<iostream>\nint main() {\n\tstd::cout << \"Hello World!\" << std::endl;\n}\n"
                    )
                ),
                Token::EOF
            ]
        );
        Ok(())
    }

    #[test]
    fn complex_all() -> Result<(), fancy_regex::Error> {
        let input = String::from(
            "@ Heading\n\nText\n\n> Quote\n\n| Col1 | Col2 | Col3 |\n\n![Caption](path/to/image)\n\n$$$\n#include<iostream>\nint main() {\n\tstd::cout << \"Hello World!\" << std::endl;\n}\n$$$\n\n@@ Heading 2\n\nText 2\n\n> Quote 2\n\n| Col1 | Col2 | Col3 |\n\n![Caption 2](path/to/image/2)\n\n$$$\n#include<iostream>\nint main() {\n\tstd::cout << \"Hello World!\" << std::endl;\n}\n$$$"
        );
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        assert_eq!(
            tokens,
            vec![
                Token::Section(String::from("Heading")),
                Token::Text(String::from("Text")),
                Token::Quote(String::from("Quote")),
                Token::Table(
                    vec![String::from("Col1"), String::from("Col2"), String::from("Col3")]
                ),
                Token::Image {
                    caption: String::from("Caption"),
                    path: String::from("path/to/image"),
                },
                Token::Codeblock(
                    String::from(
                        "#include<iostream>\nint main() {\n\tstd::cout << \"Hello World!\" << std::endl;\n}\n"
                    )
                ),
                Token::Subsection(String::from("Heading 2")),
                Token::Text(String::from("Text 2")),
                Token::Quote(String::from("Quote 2")),
                Token::Table(
                    vec![String::from("Col1"), String::from("Col2"), String::from("Col3")]
                ),
                Token::Image {
                    caption: String::from("Caption 2"),
                    path: String::from("path/to/image/2"),
                },
                Token::Codeblock(
                    String::from(
                        "#include<iostream>\nint main() {\n\tstd::cout << \"Hello World!\" << std::endl;\n}\n"
                    )
                ),
                Token::EOF
            ]
        );
        Ok(())
    }

    #[test]
    fn complex_all_with_newline() -> Result<(), fancy_regex::Error> {
        let input = String::from(
            "@ Heading\n\nText\n\n> Quote\n\n| Col1 | Col2 | Col3 |\n\n![Caption](path/to/image)\n\n$$$\n#include<iostream>\nint main() {\n\tstd::cout << \"Hello World!\" << std::endl;\n}\n$$$\n\n@@ Heading 2\n\nText 2\n\n> Quote 2\n\n| Col1 | Col2 | Col3 |\n\n![Caption 2](path/to/image/2)\n\n$$$\n#include<iostream>\nint main() {\n\tstd::cout << \"Hello World!\" << std::endl;\n}\n$$$\n"
        );
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize()?;
        assert_eq!(
            tokens,
            vec![
                Token::Section(String::from("Heading")),
                Token::Text(String::from("Text")),
                Token::Quote(String::from("Quote")),
                Token::Table(
                    vec![String::from("Col1"), String::from("Col2"), String::from("Col3")]
                ),
                Token::Image {
                    caption: String::from("Caption"),
                    path: String::from("path/to/image"),
                },
                Token::Codeblock(
                    String::from(
                        "#include<iostream>\nint main() {\n\tstd::cout << \"Hello World!\" << std::endl;\n}\n"
                    )
                ),
                Token::Subsection(String::from("Heading 2")),
                Token::Text(String::from("Text 2")),
                Token::Quote(String::from("Quote 2")),
                Token::Table(
                    vec![String::from("Col1"), String::from("Col2"), String::from("Col3")]
                ),
                Token::Image {
                    caption: String::from("Caption 2"),
                    path: String::from("path/to/image/2"),
                },
                Token::Codeblock(
                    String::from(
                        "#include<iostream>\nint main() {\n\tstd::cout << \"Hello World!\" << std::endl;\n}\n"
                    )
                ),
                Token::EOF
            ]
        );
        Ok(())
    }

    #[test]
    fn non_assert_sample_doc() {
        let sample = String::from(
            r#"@ Learning C++
/ Learning Objectives: Learn the basics of C++ /

@@ History of C++
C++ was developed by Bjarne Stroustrup at Bell Labs in the 1980s. It was originally called C with Classes, but was renamed C++ in 1983. C++ is a superset of C, which means that all valid C programs are also valid C++ programs. C++ is an object-oriented language, which means that it is designed to allow programmers to create their own data types. C++ is a compiled language, which means that it must be translated into machine code before it can be run. C++ is a statically typed language, which means that the type of a variable must be known at compile time. C++ is a strongly typed language, which means that the type of a variable cannot be changed. C++ is a case-sensitive language, which means that the names of variables, functions, and classes are case-sensitive. C++ is a free-form language, which means that the programmer can choose how to format the code. C++ is a multi-paradigm language, which means that it supports multiple programming styles. C++ is a general-purpose language, which means that it can be used to write any type of program.

> C++ was developed by Bjarne Stroustrup

!bjarne-stroustrup.jpg!Bjarne Stroustrup at the 2012 C++Now conference.

@@ Some Characterisitcs of C++
- Object oriented
- Compiled
-- Translated into machine code
- Statically typed
- Strongly typed
- Case-sensitive
- Free-form
- Multi-paradigm
- General-purpose

@@ Sample C++ Program
$$$
#include <iostream>

int main() {
	std::cout << "Hello, World!" << std::endl;
	return 0;
}
$$$

/ This program prints "Hello, World!" to the screen. /

@@ Compiling C++ Programs
C++ programs must be compiled before they can be run. The compiler translates the C++ code into machine code. The compiler is usually called g++ on Linux and Mac OS X, and cl on Windows. The compiler is usually invoked with the following command:
$$$
g++ -o program program.cpp
$$$
/ This command compiles the C++ program program.cpp and creates an executable program. /

@@ Running C++ Programs
C++ programs are run by executing the executable file that was created by the compiler. The executable file is usually called a.out on Linux and Mac OS X, and program.exe on Windows. The executable file is usually invoked with the following command:
$$$
./a.out
$$$
/ This command runs the C++ program a.out. /

@@ Keywords
C++ has a number of keywords that are reserved and cannot be used as variable names. The following is a list of the most commonly used keywords:
| name | description |
| int | integer |
| float | floating-point number |
| double | double-precision floating-point number |
| char | character |
| bool | boolean |
| void | no return value |
| true | boolean true |
| false | boolean false |
| if | if statement |
| else | else statement |
| while | while loop |
| for | for loop |
| break | break statement |
| continue | continue statement |
| return | return statement |"#
        );
        let lexer = Lexer::new(sample);
        let tokens = lexer.tokenize().unwrap();
        println!("{:?}", tokens);
    }

	#[test]
	fn button() -> Result<(), fancy_regex::Error> {
		let input = String::from("#![Button]{https://example.com}");
		let lexer = Lexer::new(input);
		let tokens = lexer.tokenize()?;
		assert_eq!(
			tokens,
			vec![
				Token::Button {
					text: String::from("Button"),
					link: String::from("https://example.com"),
				},
				Token::EOF
			]
		);
		Ok(())
	}

}