#[derive(Debug)]
pub struct Recase {
    original_text: String,
    _words: Vec<String>,
}

impl Recase {
    pub const SYMBOLS: [&'static str; 6] = [" ", ".", "/", "_", "-", "\\"];

    fn slice_into_words(input: String) -> Vec<String> {
        use unicode_segmentation::UnicodeSegmentation;

        let mut words: Vec<String> = vec![];
        let mut temp_word: Vec<&str> = vec![];

        for c in input.graphemes(true) {
            if Recase::SYMBOLS.contains(&c) {
                if !temp_word.is_empty() {
                    words.push(temp_word.clone().into_iter().collect::<String>());
                    temp_word.clear();
                }
                continue;
            }
            temp_word.push(c);
        }
        if !temp_word.is_empty() {
            words.push(temp_word.clone().into_iter().collect::<String>());
        }

        words
    }

    pub fn new(input: String) -> Recase {
        Recase {
            original_text: input.clone(),
            _words: Recase::slice_into_words(input),
        }
    }
}

impl Recase {
    pub fn original_case(&self) -> String {
        self.original_text.clone()
    }
}

#[cfg(test)]
mod tests {
    mod test_slice_words {
        use crate::Recase;

        #[test]
        fn test_slice_words_by_symbols() {
            let test_input = [
                String::from("God Matsuri"),
                String::from("God Matsuri?"),
                String::from("God Matsuri ahihihi"),
                String::from("God+Matsuri"),
                String::from("God   / Matsuri"),
            ];

            let test_expected_output = [
                vec![String::from("God"), String::from("Matsuri")],
                vec![String::from("God"), String::from("Matsuri?")],
                vec![
                    String::from("God"),
                    String::from("Matsuri"),
                    String::from("ahihihi"),
                ],
                vec![String::from("God+Matsuri")],
                vec![String::from("God"), String::from("Matsuri")],
            ];

            let mut test_output: Vec<Vec<String>> = vec![];

            for input in test_input {
                test_output.push(Recase::slice_into_words(input));
            }

            assert_eq!(test_output, test_expected_output);
        }

        #[test]
        fn test_slice_words_by_symbols_with_utf8() {
            let test_input = [
                String::from("Göd MÄtsuri?"),
                String::from("Kami まつり"),
                String::from("Gød Mætsuri a hí hí hí hí"),
            ];

            let test_expected_output = [
                vec![String::from("Göd"), String::from("MÄtsuri?")],
                vec![String::from("Kami"), String::from("まつり")],
                vec![
                    String::from("Gød"),
                    String::from("Mætsuri"),
                    String::from("a"),
                    String::from("hí"),
                    String::from("hí"),
                    String::from("hí"),
                    String::from("hí"),
                ],
            ];

            let mut test_output: Vec<Vec<String>> = vec![];

            for input in test_input {
                test_output.push(Recase::slice_into_words(input));
            }

            assert_eq!(test_output, test_expected_output);
        }
    }
}
