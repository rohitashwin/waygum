# waygum

A simple markup language.

### How to run?

```
cargo run -- -i <path/to/input.wg> -s <path/to/style.css>
```

### Screenshot

![Screenshot 2022-12-25 at 3 07 27 PM](https://user-images.githubusercontent.com/119449399/209467324-5e4b5917-ae55-48f7-971c-522fb9f6bb0b.png)

I wrote this language to use day to day instead of markdown. I wrote the lexer and the parser in Rust which helped me with learning the language. It was an interesting learning experience.

The language is written in `.wg` files. The parser/codegen itself doesn't care about the file format so it's just an indication. The codegen converts the `.wg` files into html, the stylesheet can/should be provided.

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
