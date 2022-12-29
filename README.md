# waygum

Custom Markup Language written in Rust.

### Why?

I wanted to make a blogging website, and I thought building a custom language that I can write the blogs with would be an interesting experience. I come from C++, and I know the frustrations involved in developing C++ applications. Unlike what many touted as Rust's defining features such as type safety or the borrow checker, I found `cargo`, rust's package manager, to be the most useful one. The former were nice to haves but they can be easily achieved in C++ with good practices, while external libraries is a huge pain in C++, especially with the necessity of build tools such as `make` or `cmake`. In the end, I found writing code in rust to be a very rewarding experience.

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
| Styles                | \*bold\* \/Italics\/ \$ code\$ \_underline\_ \~strikethrough\~ !\[link-name\](link) |                                                  |

### Screenshots

The screenshots show how the code looks and how the generated html looks. The default html codegen doesn't have any styles, this is just one example styling. The styles are extremely and you need to modify very few selectors. 

> #### Section Heading
<img width="441" alt="Screenshot 2022-12-29 at 12 07 31 PM" src="https://user-images.githubusercontent.com/119449399/209929668-2b548e1d-f654-4cbb-8201-08973a54f4c9.png">

<img width="575" alt="Screenshot 2022-12-29 at 12 07 38 PM" src="https://user-images.githubusercontent.com/119449399/209929739-9c3fdf64-8ab2-40af-b6c9-701898369216.png">

---

> #### Subsection with Text Snippet (they're auto numbered)
<img width="494" alt="Screenshot 2022-12-29 at 12 07 48 PM" src="https://user-images.githubusercontent.com/119449399/209929841-695f32d0-d0b0-4cbd-a2c0-10689da3efc4.png">

<img width="481" alt="Screenshot 2022-12-29 at 12 07 55 PM" src="https://user-images.githubusercontent.com/119449399/209929858-3a674176-f29e-42f0-b11a-f5985995da2f.png">

---

> #### Quote with Inline Styling

<img width="395" alt="Screenshot 2022-12-29 at 12 08 03 PM" src="https://user-images.githubusercontent.com/119449399/209929903-f01cb5eb-c725-4392-b815-4f1db4fb758f.png">

<img width="521" alt="Screenshot 2022-12-29 at 12 08 07 PM" src="https://user-images.githubusercontent.com/119449399/209929922-fde2aa15-5c3f-4b0c-bbd3-e3a8f2976048.png">

---

> #### Image with caption
<img width="643" alt="Screenshot 2022-12-29 at 12 08 22 PM" src="https://user-images.githubusercontent.com/119449399/209930168-bb53f3db-e125-4d50-9ff2-ea2072abc8ef.png">

<img width="273" alt="Screenshot 2022-12-29 at 12 08 28 PM" src="https://user-images.githubusercontent.com/119449399/209930184-b4df5ee4-8eb7-4df8-90dc-83baf8fd70fc.png">

---

> #### Multilevel List

<img width="303" alt="Screenshot 2022-12-29 at 12 09 00 PM" src="https://user-images.githubusercontent.com/119449399/209930410-97bd24dc-4d13-4c22-8cba-b0a73170894d.png">

<img width="490" alt="Screenshot 2022-12-29 at 12 09 07 PM" src="https://user-images.githubusercontent.com/119449399/209930421-eb445a9f-3f03-436e-930d-a06d6b5022af.png">

---

> #### Codeblocks

<img width="636" alt="Screenshot 2022-12-29 at 12 09 33 PM" src="https://user-images.githubusercontent.com/119449399/209930458-ed6dbda5-6af6-4c07-8236-63cce52f5794.png">

<img width="770" alt="Screenshot 2022-12-29 at 12 09 56 PM" src="https://user-images.githubusercontent.com/119449399/209930477-205efa3d-5edf-48c9-8faf-2486e4d270f3.png">

---

> #### Small Snippet with Codeblock and styles

<img width="647" alt="Screenshot 2022-12-29 at 12 10 24 PM" src="https://user-images.githubusercontent.com/119449399/209930543-2d82bf70-bd09-4e27-a30a-c5eaba131a83.png">

<img width="776" alt="Screenshot 2022-12-29 at 12 10 31 PM" src="https://user-images.githubusercontent.com/119449399/209930561-16a8c436-aa31-4e22-b9da-b084e378493f.png">

---

> #### Table (supports styling)

<img width="632" alt="Screenshot 2022-12-29 at 12 10 42 PM" src="https://user-images.githubusercontent.com/119449399/209930649-6b400365-ac4d-44df-a45b-3f31d1b849d2.png">

<img width="768" alt="Screenshot 2022-12-29 at 12 10 54 PM" src="https://user-images.githubusercontent.com/119449399/209930670-a2fc2713-db86-4f16-8a32-ae9e74921e11.png">

---

### About

I've written and rewritten this parser multiple times now. After multiple attempts at manually writing a lexer, I've come to realize that doing all that with regex would reduce my headache by about 10 times.

The parser itself doesn't use regex. 

The whole thing is pretty small (~2000 sloc) including the tests, which occupy the most sloc anyway. It's extremely fast being written in rust and all, and this has been a good learning experience for me, learning both about simple languages and rust.

### How to run?

I'll be uploading this as a crate soon, so look out for that. In the mean time, you can just clone this repo.

To convert a document manually when you clone this repo, you'll need to follow these steps:
Use the `Document` struct to construct a document object with the `from` function. This takes in the path of the `.wg` file and the path to an optional `css` file. Use the `convert_to_html` method - this should return a string containing the html contents.
