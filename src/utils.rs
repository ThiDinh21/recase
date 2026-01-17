use itertools::{Itertools, MultiPeek};
use unicode_segmentation::{GraphemeIndices, UnicodeSegmentation};

pub const SYMBOLS: [&str; 6] = [" ", ".", "/", "_", "-", "\\"];

#[derive(Debug)]
pub struct WordSplit<'a> {
    graphemes: MultiPeek<GraphemeIndices<'a>>,
}

impl<'heystack_> WordSplit<'heystack_> {
    pub fn new(heystack: &'heystack_ str) -> Self {
        WordSplit {
            // reminder: heystack.grapheme_indices(is_extended),
            graphemes: heystack.grapheme_indices(true).multipeek(),
        }
    }
}

impl<'heystack> Iterator for WordSplit<'heystack> {
    type Item = (usize, usize); // Start and end index of a word

    fn next(&mut self) -> Option<Self::Item> {
        // In this method c0, c1, c2 are the 3 next characters, not the current one
        // i.e. c0 would be the result of graphemes.next()
        let graphemes = self.graphemes.by_ref();
        let mut word_start_index = 0;
        let mut can_start_new_word = true;

        // Loop until got 1 complete word then return Some(word)
        // None if no word found
        // Ignore all listed special characters
        loop {
            // Analyze c0
            // If c0 is None -> end of str -> is boundary
            // If c0 is a symbol -> is boundary
            let peek0 = graphemes.peek();
            let is_c0_boundary = peek0.map_or(true, |(_, c)| SYMBOLS.contains(c));
            let is_c0_uppercase = peek0.map_or(false, |(_, c)| is_uppercase(c));
            let c0_len = peek0.map_or(0, |(_, c)| c.len());

            // Analyze c1
            let peek1 = graphemes.peek();
            let is_c1_none = peek1.is_none();
            let is_c1_uppercase = peek1.map_or(false, |(_, c)| is_uppercase(c));

            // Analyze c2
            let peek2 = graphemes.peek();
            // Check if c2 is neither an uppercase letter nor special char nor end of str
            let is_c2_lowercase =
                peek2.map_or(false, |(_, c)| !is_uppercase(c) && !SYMBOLS.contains(c));

            // 1. Check boundary
            // slice when a symbol is detected or end of str
            if is_c0_boundary {
                if let Some((index, _boundary)) = graphemes.next() {
                    // Ignore boundaries at the start of the word
                    if can_start_new_word {
                        continue;
                    }
                    // Return the option to the current word's indexes since a boundary is reached
                    return Some((word_start_index, index));
                }
                return None;
            }

            // 2. Check end of str
            // Check if c1 is end of str
            // Ex: hello world -> currently at "l", c0 is at "d", c1 is None
            if is_c1_none {
                if let Some((index, _)) = graphemes.next() {
                    // Edge case: only 1 letter as last word
                    // Ex: hello_world-x
                    if can_start_new_word {
                        word_start_index = index;
                    }
                    return Some((word_start_index, index + c0_len));
                }
                return None;
            }

            // 3. Check acronym
            // If UPPER - UPPER - LOWER -> is a boundary
            // i.e. HTMLFile -> c0 is at "L" , c1 at "F", c2 at "i"
            if is_c0_uppercase && is_c1_uppercase && is_c2_lowercase {
                if let Some((index, _)) = graphemes.next() {
                    return Some((word_start_index, index + c0_len));
                }
                return None;
            }

            // 4. Check camel case boundary
            // If LOWER - UPPER -> is a boundary
            // i.e. helloWorld  -> c0 is "o", c1 is "W"
            if !is_c0_uppercase && is_c1_uppercase {
                if let Some((index, _)) = graphemes.next() {
                    // Edge case: only 1 letter before this boundary and the last one
                    // Ex: .cD
                    if can_start_new_word {
                        word_start_index = index;
                    }
                    return Some((word_start_index, index + c0_len));
                }
                return None;
            }

            // Handle lowercase character
            let (index, _) = graphemes.next().unwrap();
            if can_start_new_word {
                word_start_index = index;
                can_start_new_word = false;
            }
        }
    }
}

pub fn is_uppercase(character: &str) -> bool {
    let len = character.graphemes(true).count();
    if len != 1 {
        panic!("is_uppercase only take 1 character");
    }
    character == character.to_uppercase() && character != character.to_lowercase()
}

#[cfg(test)]
mod utils_tests {
    mod test_word_split_iter {
        use crate::utils::WordSplit;

        #[test]
        fn test_iter_basic() {
            let s = "hello world/1234";
            let words: Vec<_> = WordSplit::new(s).map(|(x, y)| &s[x..y]).collect();
            assert_eq!(words, vec!["hello", "world", "1234"]);
        }

        #[test]
        fn test_iter_delim_at_beginning_end() {
            let s = "_hello  ...world-";
            let words: Vec<_> = WordSplit::new(s).map(|(x, y)| &s[x..y]).collect();
            assert_eq!(words, vec!["hello", "world"]);
        }

        #[test]
        fn test_iter_uppercase() {
            let s = "- -helloWorld";
            let words: Vec<_> = WordSplit::new(s).map(|(x, y)| &s[x..y]).collect();
            assert_eq!(words, vec!["hello", "World"]);
        }

        #[ignore = "Emoji too hard :("]
        #[test]
        fn test_iter_complex_graphemes() {
            // 🦀 is 4 bytes, 👩‍👩‍👧‍👦 is 25 bytes!
            let s = "🦀Family👩‍👩‍👧‍👦";
            let words: Vec<_> = WordSplit::new(s).map(|(x, y)| &s[x..y]).collect();
            // Assuming symbols like emojis separate words, or stay attached
            assert_eq!(words, vec!["🦀", "Family", "👩‍👩‍👧‍👦"]);
        }

        #[test]
        fn test_iter_acronym_at_end() {
            let s = "myHTML";
            let words: Vec<_> = WordSplit::new(s).map(|(x, y)| &s[x..y]).collect();
            assert_eq!(words, vec!["my", "HTML"]);
        }

        #[test]
        fn test_iter_acronym_start() {
            let s = "HTMLParser";
            let words: Vec<_> = WordSplit::new(s).map(|(x, y)| &s[x..y]).collect();
            assert_eq!(words, vec!["HTML", "Parser"]);
        }

        #[test]
        fn test_iter_consecutive_symbols() {
            let s = "---hello___world  ";
            let words: Vec<_> = WordSplit::new(s).map(|(x, y)| &s[x..y]).collect();
            assert_eq!(words, vec!["hello", "world"]);
        }

        #[test]
        fn test_iter_numbers() {
            let s = "v1.2.3Release";
            let words: Vec<_> = WordSplit::new(s).map(|(x, y)| &s[x..y]).collect();
            // This depends on your 'is_boundary' definition, but common expectation:
            assert_eq!(words, vec!["v1", "2", "3", "Release"]);
        }

        #[test]
        fn test_iter_empty_and_whitespace() {
            assert_eq!(WordSplit::new("").next(), None);
            assert_eq!(WordSplit::new("   ").next(), None);
            assert_eq!(WordSplit::new("---").next(), None);
        }

        #[test]
        fn test_iter_mixed_suite() {
            // The "All-in-One" Benchmark
            let s = "JSONParser_v2-beta__HTMLFile/path.to.mixedCase_ID";
            let words: Vec<_> = WordSplit::new(s).map(|(x, y)| &s[x..y]).collect();

            assert_eq!(
                words,
                vec![
                    "JSON",   // Acronym boundary detected
                    "Parser", // Standard Pascal
                    "v2",     // Alphanumeric kept together (assuming numbers aren't separators)
                    "beta",   // Separator skipped
                    "HTML",   // Acronym boundary
                    "File",   // Pascal
                    "path",   // Slash separator
                    "to",     // Dot separator
                    "mixed",  // camelCase start
                    "Case",   // camelCase split
                    "ID"      // Trailing acronym
                ]
            );
        }

        #[test]
        fn test_iter_messy_separators() {
            // Consecutive separators
            let s = "double__under..score";
            let words: Vec<_> = WordSplit::new(s).map(|(x, y)| &s[x..y]).collect();
            assert_eq!(words, vec!["double", "under", "score"]);
        }

        #[test]
        fn test_iter_utf8_basic() {
            let s = "Noël_München";
            let words: Vec<_> = WordSplit::new(s).map(|(x, y)| &s[x..y]).collect();
            assert_eq!(words, vec!["Noël", "München"]);
        }

        #[test]
        fn test_iter_single_char() {
            let s = "a";
            let words: Vec<_> = WordSplit::new(s).map(|(x, y)| &s[x..y]).collect();
            assert_eq!(words, vec!["a"]);
        }

        #[test]
        fn test_iter_multiple_chars() {
            let s = "aB/cD_e f";
            let words: Vec<_> = WordSplit::new(s).map(|(x, y)| &s[x..y]).collect();
            assert_eq!(words, vec!["a", "B", "c", "D", "e", "f"]);
        }
    }

    mod test_uppercase {
        use crate::utils::*;

        #[test]
        #[should_panic]
        fn is_uppercase_zero_char() {
            assert!(is_uppercase(""));
        }

        #[test]
        #[should_panic]
        fn is_uppercase_two_plus_chars() {
            assert!(is_uppercase("SS"));
            assert!(is_uppercase("Lmao"));
        }

        #[test]
        fn is_uppercase_one_char_ascii() {
            assert!(is_uppercase("S"));
            assert!(!is_uppercase("s"));
            assert!(!is_uppercase("i"));
            assert!(is_uppercase("I"));
            assert!(!is_uppercase("."));
            assert!(!is_uppercase("?"));
            assert!(!is_uppercase("9"));
        }

        #[test]
        fn is_uppercase_one_char_utf8() {
            assert!(is_uppercase("Ä"));
            assert!(!is_uppercase("ä"));
            assert!(!is_uppercase("ö"));
            assert!(is_uppercase("Å"));
            assert!(!is_uppercase("ß"));
            assert!(!is_uppercase("と"));
            assert!(!is_uppercase("á"));
        }

        #[test]
        fn uppercase_std() {
            assert_eq!("ß".to_uppercase(), "SS".to_string());
        }
    }
}
