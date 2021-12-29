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

    pub fn normal_case(&self) -> String {
        self.words
            .clone()
            .into_iter()
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn camel_case(&self) -> String {
        let mut words = self.words.clone().into_iter();
        match words.next() {
            None => panic!("The field \"words\" is empty"),
            Some(s) => {
                s + &words
                    .map(|w| utils::uppercase_first_letter(&w))
                    .collect::<Vec<String>>()
                    .join("")
            }
        }
    }

    pub fn pascal_case(&self) -> String {
        let mut words = self.words.clone().into_iter();
        match words.next() {
            None => panic!("The field \"words\" is empty"),
            Some(s) => {
                utils::uppercase_first_letter(&s)
                    + &words
                        .map(|w| utils::uppercase_first_letter(&w))
                        .collect::<Vec<String>>()
                        .join("")
            }
        }
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

    pub fn title_case(&self) -> String {
        let mut words = self.words.clone().into_iter();
        match words.next() {
            None => panic!("The field \"words\" is empty"),
            Some(s) => {
                utils::uppercase_first_letter(&s)
                    + " "
                    + &words
                        .map(|w| utils::uppercase_first_letter(&w))
                        .collect::<Vec<String>>()
                        .join(" ")
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
    fn test_normal_case() {
        let recase = Recase::new("who_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.normal_case(), "who is god and why is she matsuri");

        let recase = Recase::new("誰_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.normal_case(), "誰 is god and why is she matsuri");

        let recase = Recase::new("WHO_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.normal_case(), "w h o is god and why is she matsuri");

        let recase = Recase::new("ßho_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.normal_case(), "ßho is god and why is she matsuri");
    }

    #[test]
    fn test_camel_case() {
        let recase = Recase::new("who_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.camel_case(), "whoIsGodAndWhyIsSheMatsuri");

        let recase = Recase::new("誰_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.camel_case(), "誰IsGodAndWhyIsSheMatsuri");

        let recase = Recase::new("WHO_is_god_and_Why is she_Matsuri".to_string());
        assert_eq!(recase.camel_case(), "wHOIsGodAndWhyIsSheMatsuri");

        let recase = Recase::new("ßho_is_god_and_why_is_ßhe?_Mätßuri".to_string());
        assert_eq!(recase.camel_case(), "ßhoIsGodAndWhyIsSShe?Mätßuri");
    }

    #[test]
    fn test_pascal_case() {
        let recase = Recase::new("who_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.pascal_case(), "WhoIsGodAndWhyIsSheMatsuri");

        let recase = Recase::new("誰_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.pascal_case(), "誰IsGodAndWhyIsSheMatsuri");

        let recase = Recase::new("WHO_is_god_and_Why is she_Matsuri".to_string());
        assert_eq!(recase.pascal_case(), "WHOIsGodAndWhyIsSheMatsuri");

        let recase = Recase::new("ßho_is_god_and_why_is_ßhe?_Mätßuri".to_string());
        assert_eq!(recase.pascal_case(), "SShoIsGodAndWhyIsSShe?Mätßuri");
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

    #[test]
    fn test_title_case() {
        let recase = Recase::new("who_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.title_case(), "Who Is God And Why Is She Matsuri");

        let recase = Recase::new("誰_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.title_case(), "誰 Is God And Why Is She Matsuri");

        let recase = Recase::new("WHO_is_god_and_Why is she_Matsuri".to_string());
        assert_eq!(recase.title_case(), "W H O Is God And Why Is She Matsuri");

        let recase = Recase::new("ßho_is_god_and_why_is_ßhe?_Mätßuri".to_string());
        assert_eq!(recase.title_case(), "SSho Is God And Why Is SShe? Mätßuri");
    }
}
