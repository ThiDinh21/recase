mod utils;

#[derive(Debug)]
pub struct Recase {
    original_text: String,
    _words: Vec<String>,
}

impl Recase {
    pub const SYMBOLS: [&'static str; 6] = [" ", ".", "/", "_", "-", "\\"];

    pub fn new(input: String) -> Recase {
        Recase {
            original_text: input.clone(),
            _words: utils::slice_into_words(input),
        }
    }
}

impl Recase {
    pub fn original_case(&self) -> String {
        self.original_text.clone()
    }
}
