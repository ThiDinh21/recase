use recase::{Casing, ReCase};

fn main() {
    const INPUT: &str = "Löng and meaningless-HTML_Text";

    // Using the Casing Trait
    println!("{}", INPUT.to_kebab_case()); // Prints "löng-and-meaningless-html-text"

    let recase = ReCase::new(INPUT);

    println!("{}", recase.snake_case()); // Prints "löng_and_meaningless_html_text"
    println!("{}", recase.camel_case()); // Prints "löngAndMeaninglessHTMLText"
}
