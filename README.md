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
recase = "0.3.0"
```

<p>&nbsp</p>

## Example:

```rust
use recase::ReCase;

fn main() {
    const INPUT: &str = "Löng and meaningless-Ẽxample_Text";

    let recase1 = ReCase::new(INPUT);
    let recase2 = ReCase::new(String::from(INPUT));

    println!("{}", recase1.snake_case());     // Prints "löng_and_meaningless_ẽxample_text"
    println!("{}", recase2.camel_case());     // Prints "löngAndMeaninglessẼxampleText"
}
```

<p>&nbsp</p>

## All supported convention cases:

-   camelCase
-   snake_case
-   PascalCase
-   kebab-case
-   dot.case
-   path/case
-   windows\path\case
-   normal case
-   Title Case
-   Sentence case
-   Header-Case
-   UPPER_CASE_SNAKE_CASE
-   aLtErNaTiNg CaSe

<p>&nbsp</p>

## Limitations

-   The crate has not undergone any runtime optimization.
-   Some UTF-8 characters can't be lowercased, like "SS" which is the uppercased form of "ß". There might be more cases that I failed to notice.

<p>&nbsp</p>

## Acknowledgements

Heavily influenced by [ReCase](https://pub.dev/packages/recase) from [techniboogie-dart](https://github.com/techniboogie-dart).
