# waygum

A simple markup language.

I wrote this language to use day to day instead of markdown. I wrote the lexer and the parser in Rust which helped me with learning the language. It was an interesting learning experience.

The language is written in `.wg` files. The parser/codegen itself doesn't care about the file format so it's just an indication. The codegen converts the `.wg` files into html, the stylesheet can me provided.

The supported elements can be found in `grammar.bnf`, but here's the gist:

### Features

- Three levels of headings
- Infinitely nested lists
- Styles
- Quotes

### Maybe?

- Links
- Images

(These should be very simple to add.)
