# raedicparse

This proyect parses the "Diccionario de la RAE" in epub format and generates a text file which each line has the format "\<word\>=\<definition\>\n"

## Installation

1. Install Rust and Cargo if you don't have them installed yet. You can follow the instructions on [Rust's official website](https://www.rust-lang.org/tools/install)
2. Clone this repository: `git clone https://github.com/madcato/raedicparse.git`
3. Compile with `cargo build --release`
4. Find and download an epub version of the "Diccionario de la RAE" from the official website.

## Usage

> Usage: raedicparse --epub-path <EPUB_PATH> --output-path <OUTPUT_PATH>
> 
> Options:
>   -e, --epub-path <EPUB_PATH>      EPUB file to parse
>   -o, --output-path <OUTPUT_PATH>  output txt file to generate the definitions of each word
>   -h, --help                       Print help
>   -V, --version                    Print version

Sample usage: 

```bash
$ cargo run -- --epub-path ~/Desktop/Diccionario\ de\ la\ Lengua\ Española.epub --output-path dic.txt
```

**IMPORTANT: epub must be decompressed**

## Pending TODO

- Remove all the html tag from the text ouput.
- Create one line for each significance of each word. If a word has two meaning, separte it in two diferent lines.