use recase::ReCase;

fn main() {
    const INPUT: &str = "Löng and meaningless-Ẽxample_Text";

    let recase1 = ReCase::new(INPUT);
    let recase2 = ReCase::new(String::from(INPUT));

    println!("{}", recase1.snake_case()); // Prints "löng_and_meaningless_ẽxample_text"
    println!("{}", recase2.camel_case()); // Prints "löngAndMeaninglessẼxampleText"
}
