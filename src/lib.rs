use unicode_segmentation::UnicodeSegmentation;

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

    pub fn snake_case(&self) -> String {
        self.words
            .clone()
            .into_iter()
            .collect::<Vec<String>>()
            .join("_")
    }

    pub fn kebab_case(&self) -> String {
        self.words
            .clone()
            .into_iter()
            .collect::<Vec<String>>()
            .join("-")
    }

    pub fn dot_case(&self) -> String {
        self.words
            .clone()
            .into_iter()
            .collect::<Vec<String>>()
            .join(".")
    }

    pub fn path_case(&self) -> String {
        self.words
            .clone()
            .into_iter()
            .collect::<Vec<String>>()
            .join("/")
    }

    pub fn windows_path_case(&self) -> String {
        self.words
            .clone()
            .into_iter()
            .collect::<Vec<String>>()
            .join("\\")
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
        self.words
            .clone()
            .into_iter()
            .map(|w| utils::uppercase_first_letter(&w))
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn header_case(&self) -> String {
        self.words
            .clone()
            .into_iter()
            .map(|w| utils::uppercase_first_letter(&w))
            .collect::<Vec<String>>()
            .join("-")
    }

    pub fn upper_snake_case(&self) -> String {
        self.words
            .clone()
            .into_iter()
            .map(|w| w.to_uppercase())
            .collect::<Vec<String>>()
            .join("_")
    }

    pub fn alternating_case(&self) -> String {
        let mut uppercase = true;

        self.words
            .clone()
            .into_iter()
            .map(|w| {
                // uppercase = !uppercase;

                // Alternately recasing each letter of each word
                let chars = w.graphemes(true);
                chars
                    .map(|c| {
                        if uppercase {
                            uppercase = !uppercase;
                            c.to_uppercase()
                        } else {
                            uppercase = !uppercase;
                            c.to_lowercase()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join(" ")
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
    fn test_snake_case() {
        let recase = Recase::new("who_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.snake_case(), "who_is_god_and_why_is_she_matsuri");

        let recase = Recase::new("誰_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.snake_case(), "誰_is_god_and_why_is_she_matsuri");

        let recase = Recase::new("WHO_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.snake_case(), "w_h_o_is_god_and_why_is_she_matsuri");

        let recase = Recase::new("ßho_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.snake_case(), "ßho_is_god_and_why_is_she_matsuri");
    }

    #[test]
    fn test_kebab_case() {
        let recase = Recase::new("who_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.kebab_case(), "who-is-god-and-why-is-she-matsuri");

        let recase = Recase::new("誰_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.kebab_case(), "誰-is-god-and-why-is-she-matsuri");

        let recase = Recase::new("WHO_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.kebab_case(), "w-h-o-is-god-and-why-is-she-matsuri");

        let recase = Recase::new("ßho_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.kebab_case(), "ßho-is-god-and-why-is-she-matsuri");
    }

    #[test]
    fn test_dot_path_winpath_case() {
        let recase = Recase::new("who_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.dot_case(), "who.is.god.and.why.is.she.matsuri");

        let recase = Recase::new("WHO_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.path_case(), "w/h/o/is/god/and/why/is/she/matsuri");

        let recase = Recase::new("ßho_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(
            recase.windows_path_case(),
            "ßho\\is\\god\\and\\why\\is\\she\\matsuri"
        );
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
    fn test_title_header_case() {
        let recase = Recase::new("who_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.title_case(), "Who Is God And Why Is She Matsuri");

        let recase = Recase::new("誰_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(recase.title_case(), "誰 Is God And Why Is She Matsuri");

        let recase = Recase::new("WHO_is_god_and_Why is she_Matsuri".to_string());
        assert_eq!(recase.header_case(), "W-H-O-Is-God-And-Why-Is-She-Matsuri");

        let recase = Recase::new("ßho_is_god_and_why_is_ßhe?_Mätßuri".to_string());
        assert_eq!(recase.header_case(), "SSho-Is-God-And-Why-Is-SShe?-Mätßuri");
    }

    #[test]
    fn test_upper_snake_case() {
        let recase = Recase::new("who_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(
            recase.upper_snake_case(),
            "WHO_IS_GOD_AND_WHY_IS_SHE_MATSURI"
        );

        let recase = Recase::new("誰_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(
            recase.upper_snake_case(),
            "誰_IS_GOD_AND_WHY_IS_SHE_MATSURI"
        );

        let recase = Recase::new("WHO_is_god_and_Why is she_Matsuri".to_string());
        assert_eq!(
            recase.upper_snake_case(),
            "W_H_O_IS_GOD_AND_WHY_IS_SHE_MATSURI"
        );

        let recase = Recase::new("ßho_is_god_and_why_is_ßhe?_Mätßuri".to_string());
        assert_eq!(
            recase.upper_snake_case(),
            "SSHO_IS_GOD_AND_WHY_IS_SSHE?_MÄTSSURI"
        );
    }

    #[test]
    fn test_alternating_case() {
        let recase = Recase::new("who_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(
            recase.alternating_case(),
            "WhO iS gOd AnD wHy Is ShE mAtSuRi"
        );

        let recase = Recase::new("誰_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(
            recase.alternating_case(),
            "誰 iS gOd AnD wHy Is ShE mAtSuRi"
        );

        let recase = Recase::new("WHO_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(
            recase.alternating_case(),
            "W h O iS gOd AnD wHy Is ShE mAtSuRi"
        );

        let recase = Recase::new("ßho_is_god_and_why_is_she_Matsuri".to_string());
        assert_eq!(
            recase.alternating_case(),
            "SShO iS gOd AnD wHy Is ShE mAtSuRi"
        );
    }
}
