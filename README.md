# ReCase

[![crates-io](https://img.shields.io/crates/v/recase.svg)](https://crates.io/crates/recase)
[![api-docs](https://docs.rs/recase/badge.svg)](https://docs.rs/recase)
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)
[![License](https://img.shields.io/badge/License-BSD_2--Clause-orange.svg)](https://opensource.org/licenses/BSD-2-Clause)

Changes the input text to the desired convention case.

<p>&nbsp</p>

## Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
recase = "0.4.0"
```

<p>&nbsp</p>

## Example:

```rust
use recase::{ReCase, Casing};

fn main() {
    const INPUT: &str = "Löng and meaningless-Ẽxample_Text";

    // Using the Casing Trait
    println!("{}", INPUT.to_kebab_case());   // Prints "löng-and-meaningless-ẽxample-text"

    let recase1 = ReCase::new(INPUT);
    let recase2 = ReCase::new(String::from(INPUT));

    println!("{}", recase1.snake_case());     // Prints "löng_and_meaningless_ẽxample_text"
    println!("{}", recase2.camel_case());     // Prints "löngAndMeaninglessẼxampleText"
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

-   The crate uses a single-buffer allocation strategy for most transformations to minimize memory overhead, but still not fully optimized.
-   Some UTF-8 characters can't be lowercased, like "SS" which is the uppercased form of "ß". There might be more cases that I failed to notice.

<p>&nbsp</p>

## Acknowledgements

Heavily influenced by [ReCase](https://pub.dev/packages/recase) from [techniboogie-dart](https://github.com/techniboogie-dart).
