# waygum

Custom Markup Language written in Rust.

### Syntax

The syntax is a breed between markdown and latex(more of markdown). Here's a table of syntax:
| Name                  | Syntax                                                                 | Example Output                                   |
|-----------------------|------------------------------------------------------------------------|--------------------------------------------------|
| Section Heading       | @ Hello, World!                                                        | 1 Hello, World                                   |
| Subsection Heading    | @@ About Me                                                            | 2.1 About Me                                     |
| Subsubsection Heading | @@@ Info                                                               | 1.1.3 Info                                       |
| List                  | - Apple                                                                |                                                  |
| Sublist               | -- Second Level Item                                                   |                                                  |
| Blockquote            | > Hello! -Me                                                           |                                                  |
| Code Block            | $$$ _lines_of_code_here_ $$$                                           |                                                  |
| Table                 | Same as markdown                                                       | The first row is automatically treated as header |
| Image                 | #\[caption\](link)                                                     |                                                  |
| Button                | #!\[text\](link)                                                         |                                                  |
| Styles                | \*bold\* \/Italics\/ \$code\$ \_underline\_ \~strikethrough\~ !\[link-name\](link) |                                                  |

### About

I've written and rewritten this parser multiple times now. After multiple attempts at manually writing a lexer, I've come to realize that doing all that with regex would reduce my headache by about 10 times.

The parser itself doesn't use regex. 

The whole thing is pretty small (~2000 sloc) including the tests, which occupy the most sloc anyway. It's extremely fast being written in rust and all, and this has been a good learning experience for me, learning both about simple languages and rust.

### How to run?

I'll be uploading this as a crate soon, so look out for that. In the mean time, you can just clone this repo.

To convert a document manually when you clone this repo, you'll need to follow these steps:
Use the `Document` struct to construct a document object with the `from` function. This takes in the path of the `.wg` file and the path to an optional `css` file. Use the `convert_to_html` method - this should return a string containing the html contents.
