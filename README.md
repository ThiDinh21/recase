# ReCase

[![License](https://img.shields.io/badge/License-BSD_2--Clause-orange.svg)](https://opensource.org/licenses/BSD-2-Clause)

Changes the input text to the desired convention case.

<p>&nbsp</p>

## Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
recase = "0.1.1"
```

<p>&nbsp</p>

## Example:

```rust
use recase::ReCase;

fn main() {
    const INPUT: &str = "Löng and meaningless-Ẽxample_Text";

    let recase = ReCase::new(String::from(INPUT));

    println!("{}", recase.snake_case());     // Prints "löng_and_meaningless_ẽxample_text"
    println!("{}", recase.camel_case());     // Prints "löngAndMeaninglessẼxampleText"
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
-   Some UTF-8 characters can't be lowercased, like "SS" which is the uppercased form of "ß". There might be some more cases that I failed to detect.

<p>&nbsp</p>

## Acknowledgements

Heavily influenced by [ReCase](https://pub.dev/packages/recase) from [techniboogie-dart](https://github.com/techniboogie-dart).
