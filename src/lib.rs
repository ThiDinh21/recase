//! # ReCase
//!
//! `recase` is a text processing utility that changes the input text into desired convention cases.

use crate::utils::WordSplit;

mod utils;

/// An instance that holds the text to be re-cased.
/// # Example
/// ```
/// let recase = recase::ReCase::new("Example String");
/// assert_eq!(recase.snake_case(), String::from("example_string"));          
/// assert_eq!(recase.upper_snake_case(), String::from("EXAMPLE_STRING"));
/// ```
#[derive(Debug)]
pub struct ReCase<'a> {
    original_text: &'a str,
}

// Push lowercase of c into s
macro_rules! push_lowercase {
    ($s:expr, $c:expr) => {
        for lc in $c.to_lowercase() {
            $s.push(lc);
        }
    };
}

// Push uppercase of chars into s
macro_rules! push_uppercase {
    ($s:expr, $c:expr) => {
        for uc in $c.to_uppercase() {
            $s.push(uc);
        }
    };
}

impl<'a> ReCase<'a> {
    pub fn new(original_text: &'a str) -> Self {
        ReCase { original_text }
    }

    fn words_iter(&self) -> impl Iterator<Item = &str> {
        WordSplit::new(self.original_text).map(|(x, y)| &self.original_text[x..y])
    }

    #[inline(always)]
    fn allocate_buffer(&self) -> String {
        String::with_capacity(self.original_text.len())
    }

    fn lowercase_with_delim(&self, delim: &str) -> String {
        self.words_iter()
            .fold(self.allocate_buffer(), |mut acc, s| {
                if !acc.is_empty() {
                    acc.push_str(delim);
                }
                for c in s.chars() {
                    push_lowercase!(acc, c);
                }
                acc
            })
    }

    /// Returns a `normal case` version of the input text as a new String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new("Example String");
    /// assert_eq!(recase.normal_case(), String::from("example string"));
    /// ```
    pub fn normal_case(&self) -> String {
        self.lowercase_with_delim(" ")
    }

    /// Returns a `camelCase` version of the input text as a new String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new("Example String");
    /// assert_eq!(recase.camel_case(), String::from("exampleString"));
    /// ```
    pub fn camel_case(&self) -> String {
        let words_iter = self.words_iter();
        let mut acc = self.allocate_buffer();

        for (i, word) in words_iter.enumerate() {
            let mut chars = word.chars();
            // Push first character
            if let Some(first_char) = chars.next() {
                if i == 0 {
                    push_lowercase!(acc, first_char);
                } else {
                    push_uppercase!(acc, first_char);
                }
            }
            // Push the rest
            chars.for_each(|c| {
                push_lowercase!(acc, c);
            });
        }

        acc
    }

    /// Returns a `PascalCase` version of the input text as a new String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new("Example String");
    /// assert_eq!(recase.pascal_case(), String::from("ExampleString"));
    /// ```
    pub fn pascal_case(&self) -> String {
        let words_iter = self.words_iter();
        words_iter.fold(self.allocate_buffer(), |mut acc, word| {
            let mut chars = word.chars();
            if let Some(first_char) = chars.next() {
                push_uppercase!(acc, first_char);
            }
            // Push the rest
            chars.for_each(|c| {
                push_lowercase!(acc, c);
            });
            acc
        })
    }

    /// Returns a `snake_case` version of the input text as a new String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new("Example String");
    /// assert_eq!(recase.snake_case(), String::from("example_string"));
    /// ```
    pub fn snake_case(&self) -> String {
        self.lowercase_with_delim("_")
    }

    /// Returns a `kebab-case` version of the input text as a new String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new("Example String");
    /// assert_eq!(recase.kebab_case(), String::from("example-string"));
    /// ```
    pub fn kebab_case(&self) -> String {
        self.lowercase_with_delim("-")
    }

    /// Returns a `dot.case` version of the input text as a new String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new("Example String");
    /// assert_eq!(recase.dot_case(), String::from("example.string"));
    /// ```
    pub fn dot_case(&self) -> String {
        self.lowercase_with_delim(".")
    }

    /// Returns a `path/case` version of the input text as a new String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new("Example String");
    /// assert_eq!(recase.path_case(), String::from("example/string"));
    /// ```
    pub fn path_case(&self) -> String {
        self.lowercase_with_delim("/")
    }

    /// Returns a `windows\path\case` version of the input text as a new String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new("Example String");
    /// assert_eq!(recase.windows_path_case(), String::from("example\\string"));
    /// ```
    pub fn windows_path_case(&self) -> String {
        self.lowercase_with_delim("\\")
    }

    /// Returns a `Sentence case` version of the input text as a new String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new("Example String");
    /// assert_eq!(recase.sentence_case(), String::from("Example string"));
    /// ```
    pub fn sentence_case(&self) -> String {
        let words_iter = self.words_iter();
        let mut acc = self.allocate_buffer();

        for (i, word) in words_iter.enumerate() {
            let mut chars = word.chars();
            if i != 0 {
                acc.push_str(" ");
            } else {
                // Push first character
                if let Some(first_char) = chars.next() {
                    push_uppercase!(acc, first_char);
                }
            }
            // Push the rest
            chars.for_each(|c| {
                push_lowercase!(acc, c);
            });
        }

        acc
    }

    /// Returns a `Title Case` version of the input text as a new String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new("Example String");
    /// assert_eq!(recase.title_case(), String::from("Example String"));
    /// ```
    pub fn title_case(&self) -> String {
        let words_iter = self.words_iter();
        let mut acc = self.allocate_buffer();

        for (i, word) in words_iter.enumerate() {
            let mut chars = word.chars();
            if i != 0 {
                acc.push_str(" ");
            }

            // Push first character
            if let Some(first_char) = chars.next() {
                push_uppercase!(acc, first_char);
            }

            // Push the rest
            chars.for_each(|c| {
                push_lowercase!(acc, c);
            });
        }

        acc
    }

    /// Returns a `Header-Case` version of the input text as a new String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new("Example String");
    /// assert_eq!(recase.header_case(), String::from("Example-String"));
    /// ```
    pub fn header_case(&self) -> String {
        let words_iter = self.words_iter();
        let mut acc = self.allocate_buffer();

        for (i, word) in words_iter.enumerate() {
            let mut chars = word.chars();
            if i != 0 {
                acc.push_str("-");
            }

            // Push first character
            if let Some(first_char) = chars.next() {
                push_uppercase!(acc, first_char);
            }

            // Push the rest
            chars.for_each(|c| {
                push_lowercase!(acc, c);
            });
        }

        acc
    }

    /// Returns a `UPPER_SNAKE_CASE` version of the input text as a new String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new("Example String");
    /// assert_eq!(recase.upper_snake_case(), String::from("EXAMPLE_STRING"));
    /// ```
    pub fn upper_snake_case(&self) -> String {
        let mut acc = self.allocate_buffer();

        for word in self.words_iter() {
            if !acc.is_empty() {
                acc.push_str("_");
            }
            let chars = word.chars();
            for c in chars {
                push_uppercase!(acc, c);
            }
        }

        acc
    }

    /// Returns a `AlTeRnAtInG cAsE` version of the input text as a new String
    /// ## Example
    /// ```
    /// let recase = recase::ReCase::new("Example String");
    /// assert_eq!(recase.alternating_case(), String::from("eXaMpLe StRiNg"));
    /// ```
    pub fn alternating_case(&self) -> String {
        let mut should_uppercase = false;
        let mut acc = self.allocate_buffer();

        for word in self.words_iter() {
            if !acc.is_empty() {
                acc.push_str(" ");
            }

            let chars = word.chars();
            for c in chars {
                if should_uppercase {
                    push_uppercase!(acc, c);
                } else {
                    push_lowercase!(acc, c);
                }
                should_uppercase = !should_uppercase;
            }
        }

        acc
    }
}

pub trait Casing {
    /// Returns a `normal case` version of the input text as a new String
    /// ## Example
    /// ```
    /// use recase::Casing;
    /// assert_eq!("Example String".to_normal_case(), "example string");
    /// ```
    fn to_normal_case(&self) -> String;

    /// Returns a `camelCase` version of the input text as a new String
    /// ## Example
    /// ```
    /// use recase::Casing;
    /// assert_eq!("Example String".to_camel_case(), String::from("exampleString"));
    /// ```
    fn to_camel_case(&self) -> String;

    /// Returns a `PascalCase` version of the input text as a new String
    /// ## Example
    /// ```
    /// use recase::Casing;
    /// assert_eq!("Example String".to_pascal_case(), String::from("ExampleString"));
    /// ```
    fn to_pascal_case(&self) -> String;

    /// Returns a `snake_case` version of the input text as a new String
    /// ## Example
    /// ```
    /// use recase::Casing;
    /// assert_eq!("Example String".to_snake_case(), String::from("example_string"));
    /// ```
    fn to_snake_case(&self) -> String;

    /// Returns a `kebab-case` version of the input text as a new String
    /// ## Example
    /// ```
    /// use recase::Casing;
    /// assert_eq!("Example String".to_kebab_case(), String::from("example-string"));
    /// ```
    fn to_kebab_case(&self) -> String;

    /// Returns a `dot.case` version of the input text as a new String
    /// ## Example
    /// ```
    /// use recase::Casing;
    /// assert_eq!("Example String".to_dot_case(), String::from("example.string"));
    /// ```
    fn to_dot_case(&self) -> String;

    /// Returns a `path/case` version of the input text as a new String
    /// ## Example
    /// ```
    /// use recase::Casing;
    /// assert_eq!("Example String".to_path_case(), String::from("example/string"));
    /// ```
    fn to_path_case(&self) -> String;

    /// Returns a `windows\path\case` version of the input text as a new String
    /// ## Example
    /// ```
    /// use recase::Casing;
    /// assert_eq!("Example String".to_windows_path_case(), String::from("example\\string"));
    /// ```
    fn to_windows_path_case(&self) -> String;

    /// Returns a `Sentence case` version of the input text as a new String
    /// ## Example
    /// ```
    /// use recase::Casing;
    /// assert_eq!("Example String".to_sentence_case(), String::from("Example string"));
    /// ```
    fn to_sentence_case(&self) -> String;

    /// Returns a `Title Case` version of the input text as a new String
    /// ## Example
    /// ```
    /// use recase::Casing;
    /// assert_eq!("Example String".to_title_case(), String::from("Example String"));
    /// ```
    fn to_title_case(&self) -> String;

    /// Returns a `Header-Case` version of the input text as a new String
    /// ## Example
    /// ```
    /// use recase::Casing;
    /// assert_eq!("Example String".to_header_case(), String::from("Example-String"));
    /// ```
    fn to_header_case(&self) -> String;

    /// Returns a `UPPER_SNAKE_CASE` version of the input text as a new String
    /// ## Example
    /// ```
    /// use recase::Casing;
    /// assert_eq!("Example String".to_upper_snake_case(), String::from("EXAMPLE_STRING"));
    /// ```
    fn to_upper_snake_case(&self) -> String;

    /// Returns a `AlTeRnAtInG cAsE` version of the input text as a new String
    /// ## Example
    /// ```
    /// use recase::Casing;
    /// assert_eq!("Example String".to_alternating_case(), String::from("eXaMpLe StRiNg"));
    /// ```
    fn to_alternating_case(&self) -> String;
}

impl Casing for str {
    fn to_normal_case(&self) -> String {
        ReCase::new(self).normal_case()
    }

    fn to_camel_case(&self) -> String {
        ReCase::new(self).camel_case()
    }

    fn to_pascal_case(&self) -> String {
        ReCase::new(self).pascal_case()
    }

    fn to_snake_case(&self) -> String {
        ReCase::new(self).snake_case()
    }

    fn to_kebab_case(&self) -> String {
        ReCase::new(self).kebab_case()
    }

    fn to_dot_case(&self) -> String {
        ReCase::new(self).dot_case()
    }

    fn to_path_case(&self) -> String {
        ReCase::new(self).path_case()
    }

    fn to_windows_path_case(&self) -> String {
        ReCase::new(self).windows_path_case()
    }

    fn to_sentence_case(&self) -> String {
        ReCase::new(self).sentence_case()
    }

    fn to_title_case(&self) -> String {
        ReCase::new(self).title_case()
    }

    fn to_header_case(&self) -> String {
        ReCase::new(self).header_case()
    }

    fn to_upper_snake_case(&self) -> String {
        ReCase::new(self).upper_snake_case()
    }

    fn to_alternating_case(&self) -> String {
        ReCase::new(self).alternating_case()
    }
}

#[cfg(test)]
mod recase_tests {
    use crate::{Casing, ReCase};

    #[test]
    fn test_normal_case() {
        let recase = ReCase::new("long_random_text");
        assert_eq!(recase.normal_case(), "long random text");

        let recase = ReCase::new("誰_long_random_text");
        assert_eq!(recase.normal_case(), "誰 long random text");

        let recase = ReCase::new("LONG_random_text");
        assert_eq!(recase.normal_case(), "long random text");

        let recase = ReCase::new("ßlong_random_text");
        assert_eq!(recase.normal_case(), "ßlong random text");
    }

    #[test]
    fn test_camel_case() {
        let recase = ReCase::new("random_text");
        assert_eq!(recase.camel_case(), "randomText");

        let recase = ReCase::new("誰_randomText");
        assert_eq!(recase.camel_case(), "誰RandomText");

        let recase = ReCase::new("RanDom text");
        assert_eq!(recase.camel_case(), "ranDomText");

        let recase = ReCase::new("ßändom ßext");
        assert_eq!(recase.camel_case(), "ßändomSSext");
    }

    #[test]
    fn test_upper_snake_case() {
        let recase = ReCase::new("HTML_parser");
        assert_eq!(recase.upper_snake_case(), "HTML_PARSER");
    }

    #[test]
    fn test_alternating_case() {
        let recase = ReCase::new("random Text");
        assert_eq!(recase.alternating_case(), "rAnDoM tExT");

        let recase = ReCase::new("誰_random Text");
        assert_eq!(recase.alternating_case(), "誰 RaNdOm TeXt");
    }

    #[test]
    fn test_casing_trait() {
        let s = "Hello World";

        assert_eq!(s.to_kebab_case(), "hello-world");
        assert_eq!(s.to_snake_case(), "hello_world");
        assert_eq!(s.to_alternating_case(), "hElLo WoRlD");
    }
}
