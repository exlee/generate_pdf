# generate_pdf

## What is it?

Simple "development" PDF generator.

Generates visually distinct PDFs with set characteristic such as file size or number of pages.
Document is packed with data using invisible `_` character.

I'd recommend inspecting source code (of both generator and output) for safety.

Known issues:
- file sizes aren't 100% accurate due to how PDFs are rendered
- rendered text isn't typesetted carefully so it could be broken

## Installation

- Clone repository
- Build `cargo build -r` or..
- `cargo install --path .` to install in path

## Usage

```
Usage: generate_pdf [OPTIONS] <color> [text]...

Arguments:
  <color>    Text color, web color names and hex codes (without #) are supported e.g. red, blue, hotpink, 00ff00, rgb(100,100,100)
  [text]...  Text to print

Options:
      --pages <PAGES>     Number of pages to generate [default: 1]
  -o, --output <OUTPUT>
  -s, --size <SIZE>       Generated file size ByteSize strings accepted, eg: 10Mb, 1024Kb etc. Data is distributed across all pages. (might not be accurate)
      --no-random-string  Don't print random number inside
      --no-pagenum        Don't print page numbers
      --no-sizeinfo       Don't print PDF size information
      --no-stats          Print stats (file size, generation time)
      --silent            Print only resulting file name on success
      --debug             Show debug messages
  -h, --help              Print help
  -V, --version           Print version
```

- Colors are parsed by [csscolorparser](https://crates.io/crates/csscolorparser) crate
- File sizes are parsed by [bytesize](https://crates.io/crates/bytesize) crate
- Default file name infers from named color. If color can't be inferred `--output` option has to be provided
- Using named color (instead of hex provided one) guarantees name generation
- By default PDF content is rather verbose

## Example PDF content

```
> generate_pdf --output example.pdf --pages 10 --size 1MiB slateblue This is example file!
Finished in 21.494ms. Final file size: 1.1 MB.
File name: example.pdf
```

[Output](./example.pdf)
