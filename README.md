# LolCompiler

LolCompiler is a custom, lightweight compiler written in Rust that translates a LOLCODE-inspired markup language into standard HTML. It parses `.lol` files, analyzes the syntax and semantics (including variables and scoping), generates an equivalent `.html` file, and automatically opens it in your default web browser (on Windows and macOS).

## Features

* **Custom Lexer & Parser:** Reads whitespace-separated tokens and validates your custom `.lol` syntax.
* **Semantic Analysis:** Supports scoping and variables. You can define variables and inject them anywhere in your text.
* **Automatic Browser Launch:** Compiles your code and instantly pops open the generated HTML file in your default browser.
* **HTML Element Support:** Includes syntax for `<head>`, `<title>`, paragraphs, lists, bold text, italics, links, and line breaks.

## Usage

To run the compiler, pass a `.lol` file as an argument.

```bash
cargo run -- path/to/your/file.lol
```
*(Or, if using the compiled binary directly: `./lolcompiler path/to/your/file.lol`)*

The compiler will generate an HTML file in the same directory (e.g., `file.html`) and attempt to open it automatically.

---

## Language Reference

The syntax relies on specific keywords (tags) to define structure, styles, and variables. 

### Document Structure
Every valid `.lol` file must start with `#HAI` and end with `#KBYE`.

```lol
#HAI
  #MAEK HEAD
    #GIMMEH TITLE My Page #OIC
  #MKAY

  #OBTW This is a comment and will be ignored #TLDR

  #MAEK PARAGRAF
    Welcome to my webpage!
  #MKAY
#KBYE
```

### Variables
You can define variables using `#IHAZ`, assign their value with `#ITIZ`, and close the definition with `#MKAY`. To use the variable later, use `#LEMMESEE`.

```lol
#IHAZ greeting #ITIZ Hello #MKAY
#MAEK PARAGRAF
  #LEMMESEE greeting #OIC World!
#MKAY
```

### Typography and Links
Formatting relies on the `#GIMMEH` keyword followed by the style, and must be closed with `#OIC`.

* **Bold:** `#GIMMEH BOLD Important Text #OIC`
* **Italics:** `#GIMMEH ITALICS Fancy Text #OIC`
* **Links:** `#GIMMEH LINX https://rust-lang.org #OIC`
* **Line Break:** `#GIMMEH NEWLINE`

### Lists
Lists are created using `#MAEK LIST` and closed with `#MKAY`. Each item is defined with `#GIMMEH ITEM` and closed with `#OIC`.

```lol
#MAEK LIST
  #GIMMEH ITEM First item #OIC
  #GIMMEH ITEM Second item #OIC
#MKAY
```

---

## Example

### Input (`example.lol`)

```lol
#HAI
  #MAEK HEAD
    #GIMMEH TITLE My LOL Page #OIC
  #MKAY
  
  #IHAZ language #ITIZ Rust #MKAY

  #MAEK PARAGRAF
    This page was compiled using #GIMMEH BOLD #LEMMESEE language #OIC #OIC!
    #GIMMEH NEWLINE
    #GIMMEH ITALICS Pretty cool, right? #OIC
  #MKAY

  #MAEK LIST
    #GIMMEH ITEM Fast #OIC
    #GIMMEH ITEM Safe #OIC
  #MKAY
#KBYE
```

### Output (`example.html`)

```html
<html>
<head>
<title>My LOL Page </title>
</head>
<body>
<p>This page was compiled using <b>Rust </b> ! <br>
 <i>Pretty cool, right? </i> </p>
<ul><li>Fast </li>
<li>Safe </li>
</ul>
</body>
</html>
```

## Architecture

LolCompiler is broken down into four distinct phases:
1. **Lexical Analyzer (Lexer):** Traverses the raw source string character by character, clustering them into valid tokens while filtering out arbitrary whitespace.
2. **Syntax Analyzer (Parser):** Validates the token sequence against the language's grammar rules (e.g., ensuring `#OIC` properly closes a `#GIMMEH BOLD`).
3. **Semantic Analyzer:** Manages scopes and variable assignments utilizing HashMaps to ensure variables are defined before they are used.
4. **Compiler:** Orchestrates the entire process, aggregating string outputs into an HTML stack and writing them to the final file.
