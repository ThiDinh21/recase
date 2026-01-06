//! # ReCase
//!
//! `recase` is a text processing utility that changes the input text into desired convention cases.

use unicode_segmentation::UnicodeSegmentation;

mod utils;

/// An instance that holds the text to be re-cased.
/// # Example
/// ```
/// let recase = recase::ReCase::new(String::from("Example String"));
/// assert_eq!(recase.snake_case(), String::from("example_string"));          
/// assert_eq!(recase.upper_snake_case(), String::from("EXAMPLE_STRING"));
/// ```
#[derive(Debug)]
pub struct ReCase {
    original_text: String,
    words: Vec<String>,
}

impl ReCase {
    /// Create a new ReCase instance. Once created, it can be used repeatedly to convert the input text into
    /// supported convention cases.
    /// The method takes a &str or a String as an input.
    pub fn new<S: Into<String>>(original_text: S) -> ReCase {
        let original_text = original_text.into();
        let words = utils::slice_into_words(original_text.clone());
        ReCase {
            original_text,
            words,
        }
    }

    /// Create a new ReCase instance. Once created, it can be used repeatedly to convert the input text into
    /// supported convention cases.
    /// The method takes a &str as an input.
    #[deprecated(since = "0.3.0", note = "Please use the \"new()\" constructor instead")]
    pub fn new_from_str(original_text: &str) -> ReCase {
        ReCase::new(original_text.to_string())
    }

    /// Returns a clone of the original String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new(String::from("Example String"));
    /// assert_eq!(recase.original_case(), String::from("Example String"));
    /// ```
    pub fn original_case(&self) -> String {
        self.original_text.clone()
    }

    /// Returns a `normal case` version of the original String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new(String::from("Example String"));
    /// assert_eq!(recase.normal_case(), String::from("example string"));
    /// ```
    pub fn normal_case(&self) -> String {
        self.words.join(" ")
    }

    /// Returns a `camelCase` version of the original String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new(String::from("Example String"));
    /// assert_eq!(recase.camel_case(), String::from("exampleString"));
    /// ```
    pub fn camel_case(&self) -> String {
        match self.words.split_first() {
            None => "".to_owned(),
            Some((first_word, the_rest)) => the_rest
                .iter()
                .map(|s| utils::uppercase_first_letter(s))
                .fold(first_word.to_owned(), |mut acc, s| {
                    acc.push_str(&s);
                    acc
                }),
        }
    }

    /// Returns a `PascalCase` version of the original String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new(String::from("Example String"));
    /// assert_eq!(recase.pascal_case(), String::from("ExampleString"));
    /// ```
    pub fn pascal_case(&self) -> String {
        match self.words.split_first() {
            None => "".to_owned(),
            Some((first_word, the_rest)) => the_rest
                .iter()
                .map(|s| utils::uppercase_first_letter(s))
                .fold(
                    utils::uppercase_first_letter(first_word).to_owned(),
                    |mut acc, s| {
                        acc.push_str(&s);
                        acc
                    },
                ),
        }
    }

    /// Returns a `snake_case` version of the original String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new(String::from("Example String"));
    /// assert_eq!(recase.snake_case(), String::from("example_string"));
    /// ```
    pub fn snake_case(&self) -> String {
        self.words.join("_")
    }

    /// Returns a `kebab-case` version of the original String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new(String::from("Example String"));
    /// assert_eq!(recase.kebab_case(), String::from("example-string"));
    /// ```
    pub fn kebab_case(&self) -> String {
        self.words.join("-")
    }

    /// Returns a `dot.case` version of the original String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new(String::from("Example String"));
    /// assert_eq!(recase.dot_case(), String::from("example.string"));
    /// ```
    pub fn dot_case(&self) -> String {
        self.words.join(".")
    }

    /// Returns a `path/case` version of the original String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new(String::from("Example String"));
    /// assert_eq!(recase.path_case(), String::from("example/string"));
    /// ```
    pub fn path_case(&self) -> String {
        self.words.join("/")
    }

    /// Returns a `windows\path\case` version of the original String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new(String::from("Example String"));
    /// assert_eq!(recase.windows_path_case(), String::from("example\\string"));
    /// ```
    pub fn windows_path_case(&self) -> String {
        self.words.join("\\")
    }

    /// Returns a `Sentence case` version of the original String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new(String::from("Example String"));
    /// assert_eq!(recase.sentence_case(), String::from("Example string"));
    /// ```
    pub fn sentence_case(&self) -> String {
        match self.words.split_first() {
            None => "".to_owned(),
            Some((first_word, the_rest)) => {
                let mut res = utils::uppercase_first_letter(first_word);
                for word in the_rest {
                    res.push_str(" ");
                    res.push_str(word);
                }
                res
            }
        }
    }

    /// Returns a `Title Case` version of the original String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new(String::from("Example String"));
    /// assert_eq!(recase.title_case(), String::from("Example String"));
    /// ```
    pub fn title_case(&self) -> String {
        self.words
            .iter()
            .map(|w| utils::uppercase_first_letter(&w))
            .collect::<Vec<String>>()
            .join(" ")
    }

    /// Returns a `Header-Case` version of the original String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new(String::from("Example String"));
    /// assert_eq!(recase.header_case(), String::from("Example-String"));
    /// ```
    pub fn header_case(&self) -> String {
        self.words
            .iter()
            .map(|w| utils::uppercase_first_letter(&w))
            .collect::<Vec<String>>()
            .join("-")
    }

    /// Returns a `UPPER_SNAKE_CASE` version of the original String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new(String::from("Example String"));
    /// assert_eq!(recase.upper_snake_case(), String::from("EXAMPLE_STRING"));
    /// ```
    pub fn upper_snake_case(&self) -> String {
        self.words
            .iter()
            .map(|w| w.to_uppercase())
            .collect::<Vec<String>>()
            .join("_")
    }

    /// Returns a `AlTeRnAtInG cAsE` version of the original String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new(String::from("Example String"));
    /// assert_eq!(recase.alternating_case(), String::from("eXaMpLe StRiNg"));
    /// ```
    pub fn alternating_case(&self) -> String {
        let mut uppercase = true;

        self.words
            .iter()
            .map(|w| {
                // Alternately recasing each letter of each word
                let chars = w.graphemes(true);
                chars
                    .map(|c| {
                        uppercase = !uppercase;
                        if uppercase {
                            c.to_uppercase()
                        } else {
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
    use crate::ReCase;

    #[test]
    fn test_constructor() {
        let recase = ReCase::new("TestInput");
        assert_eq!(recase.words, vec!["test".to_string(), "input".to_string()]);

        let recase = ReCase::new("test_input".to_string());
        assert_eq!(recase.words, vec!["test".to_string(), "input".to_string()]);

        let recase = ReCase::new("Test-input/Ütf8 ütf8".to_string());
        assert_eq!(
            recase.words,
            vec![
                "test".to_string(),
                "input".to_string(),
                "ütf8".to_string(),
                "ütf8".to_string()
            ]
        );

        let recase = ReCase::new("Test-input/Ütf8 ütf8");
        assert_eq!(
            recase.words,
            vec![
                "test".to_string(),
                "input".to_string(),
                "ütf8".to_string(),
                "ütf8".to_string()
            ]
        );
    }

    #[test]
    fn test_normal_case() {
        let recase = ReCase::new("long_random_text".to_string());
        assert_eq!(recase.normal_case(), "long random text");

        let recase = ReCase::new("誰_long_random_text".to_string());
        assert_eq!(recase.normal_case(), "誰 long random text");

        let recase = ReCase::new("LONG_random_text".to_string());
        assert_eq!(recase.normal_case(), "l o n g random text");

        let recase = ReCase::new("ßlong_random_text".to_string());
        assert_eq!(recase.normal_case(), "ßlong random text");
    }

    #[test]
    fn test_camel_case() {
        let recase = ReCase::new("random_text".to_string());
        assert_eq!(recase.camel_case(), "randomText");

        let recase = ReCase::new("誰_randomText".to_string());
        assert_eq!(recase.camel_case(), "誰RandomText");

        let recase = ReCase::new("RANdom text".to_string());
        assert_eq!(recase.camel_case(), "rANdomText");

        let recase = ReCase::new("ßändom ßext".to_string());
        assert_eq!(recase.camel_case(), "ßändomSSext");
    }

    #[test]
    fn test_upper_snake_case() {
        let recase = ReCase::new("random_TEXT".to_string());
        assert_eq!(recase.upper_snake_case(), "RANDOM_T_E_X_T");
    }

    #[test]
    fn test_alternating_case() {
        let recase = ReCase::new("random Text".to_string());
        assert_eq!(recase.alternating_case(), "rAnDoM tExT");

        let recase = ReCase::new("誰_random Text".to_string());
        assert_eq!(recase.alternating_case(), "誰 RaNdOm TeXt");
    }
}
