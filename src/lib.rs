mod utils;

#[derive(Debug)]
pub struct Recase {
    original_text: String,
    words: Vec<String>,
}

impl Recase {
    pub fn new(original_text: String) -> Recase {
        let words = utils::slice_into_words(original_text.clone());
        if words.len() < 1 {
            panic!("Unable to separate words from input");
        }
        Recase {
            original_text,
            words,
        }
    }
}

impl Recase {
    pub fn original_case(&self) -> String {
        self.original_text.clone()
    }

    pub fn sentence_case(&self) -> String {
        let mut words = self.words.clone().into_iter();
        match words.next() {
            None => panic!("The field \"words\" is empty"),
            Some(s) => {
                utils::uppercase_first_letter(&s) + " " + &words.collect::<Vec<String>>().join(" ")
            }
        }
    }
}

#[cfg(test)]
mod recase_tests {
    use crate::Recase;

    #[test]
    #[ignore]
    #[should_panic(expected = "Unable to separate words from input")]
    fn test_constructor() {
        let recase = Recase::new("TestInput".to_string());
        assert_eq!(recase.words, vec!["test".to_string(), "input".to_string()]);

        let recase = Recase::new("test_input".to_string());
        assert_eq!(recase.words, vec!["test".to_string(), "input".to_string()]);

        let recase = Recase::new("Test-input/Ütf8 ütf8".to_string());
        assert_eq!(
            recase.words,
            vec![
                "test".to_string(),
                "input".to_string(),
                "ütf8".to_string(),
                "ütf8".to_string()
            ]
        );
        Recase::new("__ /".to_string());
    }

    #[test]
    fn test_sentence_case() {
        let recase = Recase::new("who_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.sentence_case(), "Who is god and why is she matsuri");

        let recase = Recase::new("誰_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.sentence_case(), "誰 is god and why is she matsuri");

        let recase = Recase::new("WHO_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(
            recase.sentence_case(),
            "W h o is god and why is she matsuri"
        );

        let recase = Recase::new("ßho_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.sentence_case(), "SSho is god and why is she matsuri");
    }
}
