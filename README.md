# csl-to-markdown
![GitHub language count](https://img.shields.io/badge/language-rust-blue.svg) ![GitHub](https://img.shields.io/github/license/mashape/apistatus.svg) [![Build Status](https://travis-ci.org/Eonm/csl-to-markdown.svg?branch=master)](https://travis-ci.org/Eonm/csl-to-markdown)

Makes your Zotero's/Mendeley's bibliography export compatible with markdown
## Download
Download the last release [here](https://github.com/Eonm/csl-to-markdown/releases/latest).

## Requirements
You need to install Rust [see](https://www.rust-lang.org/en-US/install.html) to test and build this code.

## Test

```bash
cargo test
```

## Build

```bash
cargo build --release
```

## Usage

1- Make your csl file compatible with markdown. See the full list of styles [here](https://www.zotero.org/styles).

**On unix based system :**
```bash
./csl_to_markdown -i input_file.csl -o output_file.csl
./csl_to_markdown -i input_file.csl > output_file.csl
```

**On windows :**
```dos
./csl_to_markdown.exe -i input_file.csl -o output_file.csl
./csl_to_markdown.exe -i input_file.csl > output_file.csl
```

2- Add this new style to Zotero/Mendeley. 

**Zotero :** [See how to install a citation style](https://www.zotero.org/support/styles#alternative_installation_methods) from a file.

**Mendeley :** View → Citation Style → More styles → Get More Styles → in "Download style field" type : ```file:///path_to_your/csl_file.csl```.

3- Use this new style to create your bibliography.

**Zotero :** See how to [create a bibliography](https://www.zotero.org/support/creating_bibliographies). Dont forget to select the good citation style before exporting your bibliography.

**Mendeley :** See [How to copy & paste formatted citations anywhere](https://blog.mendeley.com/tag/copypaste/)

## Acknowledgement

This sofwtare is inspired by this blog article : ["Markdown et Zotero"](https://zotero.hypotheses.org/2258#autres_usages)

## Licence

This software is distributed under the MIT licence.
