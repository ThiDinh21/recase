use recase::ReCase;

fn main() {
    const INPUT: &str = "Löng and meaningless-Ẽxample_Text";

    let recase = ReCase::new(String::from(INPUT));

    println!("{}", recase.snake_case()); // Prints "löng_and_meaningless_ẽxample_text"
    println!("{}", recase.camel_case()); // Prints "löngAndMeaninglessẼxampleText"
}
