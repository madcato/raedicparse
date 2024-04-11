# raedicparse

This proyect parses the "Diccionario de la RAE" in epub format and generates a text file which ieach line is "<word>=<definition>\n"

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

## Pending TODO

- Remove all the html tag from the text ouput.
- Create one line for each significance of each word. If a word has two meaning, separte it in two diferent lines.