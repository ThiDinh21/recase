# ReCase

[![crates-io](https://img.shields.io/crates/v/recase.svg)](https://crates.io/crates/recase)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A2BE2)](CHANGELOG.md)
[![api-docs](https://docs.rs/recase/badge.svg)](https://docs.rs/recase)
[![License](https://img.shields.io/badge/License-BSD_2--Clause-orange.svg)](https://opensource.org/licenses/BSD-2-Clause)

Changes the input text to the desired convention case.

<p>&nbsp</p>

## Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
recase = "0.5.1"
```

<p>&nbsp</p>

## Features

- **Zero Allocation Logic**: Heavily optimized runtime and memory usage. It allocates exactly once (for the result string).
- **Unicode Aware**: Handles complex graphemes, and acronyms correctly aside from emojis, they are treated the same as lowercase characters for now.
- **Acronyms**: now support acronyms and will not split them into seperated characters anymore.

<p>&nbsp</p>

## Example:

```rust
use recase::{ReCase, Casing};

fn main() {
    const INPUT: &str = "Löng and meaningless-HTML_Text";

    // Using the Casing Trait
    println!("{}", INPUT.to_kebab_case()); // Prints "löng-and-meaningless-html-text"

    let recase = ReCase::new(INPUT);

    println!("{}", recase.snake_case()); // Prints "löng_and_meaningless_html_text"
    println!("{}", recase.camel_case()); // Prints "löngAndMeaninglessHTMLText"
}
```

<p>&nbsp</p>

## All supported convention cases:

| Convention Case       | Trait Method (on `&str` / `String`) | Struct Method (`ReCase`) | Example Result   |
| :-------------------- | :---------------------------------- | :----------------------- | :--------------- |
| **camelCase**         | `.to_camel_case()`                  | `.camel_case()`          | `exampleString`  |
| **snake_case**        | `.to_snake_case()`                  | `.snake_case()`          | `example_string` |
| **PascalCase**        | `.to_pascal_case()`                 | `.pascal_case()`         | `ExampleString`  |
| **kebab-case**        | `.to_kebab_case()`                  | `.kebab_case()`          | `example-string` |
| **dot.case**          | `.to_dot_case()`                    | `.dot_case()`            | `example.string` |
| **path/case**         | `.to_path_case()`                   | `.path_case()`           | `example/string` |
| **windows\path\case** | `.to_windows_path_case()`           | `.windows_path_case()`   | `example\string` |
| **normal case**       | `.to_normal_case()`                 | `.normal_case()`         | `example string` |
| **Title Case**        | `.to_title_case()`                  | `.title_case()`          | `Example String` |
| **Sentence case**     | `.to_sentence_case()`               | `.sentence_case()`       | `Example string` |
| **Header-Case**       | `.to_header_case()`                 | `.header_case()`         | `Example-String` |
| **UPPER_SNAKE_CASE**  | `.to_upper_snake_case()`            | `.upper_snake_case()`    | `EXAMPLE_STRING` |
| **AlTeRnAtInG cAsE**  | `.to_alternating_case()`            | `.alternating_case()`    | `eXaMpLe StRiNg` |

<p>&nbsp</p>

## Limitations

- Emojis are treated the same as lEmoji will be counted as lowercase characterowercase characters.
- Some UTF-8 characters can't be lowercased, like "SS" which is the uppercased form of "ß" or the dotless I (I) will turn into a normal i. There might be more cases that I failed to notice.

<p>&nbsp</p>

## Acknowledgements

Heavily influenced by [ReCase](https://pub.dev/packages/recase) from [techniboogie-dart](https://github.com/techniboogie-dart).
