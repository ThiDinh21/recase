use itertools::{Itertools, MultiPeek};
use unicode_segmentation::{GraphemeIndices, UnicodeSegmentation};

struct WordSplit<'a> {
    graphemes: MultiPeek<GraphemeIndices<'a>>,
}

impl<'heystack_> WordSplit<'heystack_> {
    fn new(heystack: &'heystack_ str) -> Self {
        WordSplit {
            // reminder: heystack.grapheme_indices(is_extended),
            graphemes: heystack.grapheme_indices(true).multipeek(),
        }
    }
}

impl<'heystack> Iterator for WordSplit<'heystack> {
    type Item = (usize, usize); // Start and end index of a word

    fn next(&mut self) -> Option<Self::Item> {
        pub const SYMBOLS: [&str; 6] = [" ", ".", "/", "_", "-", "\\"];
        let graphemes = self.graphemes.by_ref();
        let mut curr_word_start = 0;
        let mut can_start_new_word = true;

        // If c0 is None -> end of str -> is boudnary
        // If c0 is a symbol -> is boundary
        fn check_boundary((_, c): &(usize, &str)) -> bool {
            SYMBOLS.contains(c)
        }
        fn check_uppercase((_, c): &(usize, &str)) -> bool {
            is_uppercase(c)
        }

        loop {
            // slice when a symbol is detected or end of str
            let is_c0_boundary = graphemes.peek().map_or(true, |c| check_boundary(c));

            if is_c0_boundary {
                if let Some((index, _)) = graphemes.next() {
                    // Ignore boundaries at the start of the word
                    if can_start_new_word {
                        continue;
                    }
                    return Some((curr_word_start, index));
                }
                return None;
            }
            // Check if c1 is end of str
            // Ex: hello world -> c0 is at d, c1 is None
            if graphemes.peek().is_none() {
                if let Some((index, g)) = graphemes.next() {
                    // Edge case: only 1 letter as last word
                    // Ex: hello_world-x
                    if can_start_new_word {
                        curr_word_start = index;
                    }
                    return Some((curr_word_start, index + g.len()));
                }
                return None;
            }
            graphemes.reset_peek();

            let is_c0_uppercase = graphemes.peek().map_or(false, |c| check_uppercase(c));
            let is_c1_uppercase = graphemes.peek().map_or(false, |c| check_uppercase(c));
            // Check if c2 is neither an uppercase letter nor special char nor end of str
            let is_c2_lowercase = graphemes
                .peek()
                .map_or(false, |c| !check_uppercase(c) && !check_boundary(c));
            graphemes.reset_peek();

            // If UPPER - UPPER - LOWER -> is a boundary
            // Ex: HTMLFile -> c0 is at L
            if is_c0_uppercase && is_c1_uppercase && is_c2_lowercase {
                if let Some((index, g)) = graphemes.next() {
                    return Some((curr_word_start, index + g.len()));
                }
                return None;
            }

            // If LOWER - UPPER -> is a boundary
            if !is_c0_uppercase && is_c1_uppercase {
                if let Some((index, g)) = graphemes.next() {
                    // Edge case: only 1 letter before this boundary and the last one
                    // Ex: .cD
                    if can_start_new_word {
                        curr_word_start = index;
                    }
                    dbg!(index);
                    dbg!(g);
                    return Some((curr_word_start, index + g.len()));
                }
                return None;
            }
            dbg!(curr_word_start);
            let (index, _) = graphemes.next().unwrap();
            if can_start_new_word {
                curr_word_start = index;
                can_start_new_word = false;
            }
        }
    }
}

pub fn slice_into_words(input: String) -> Vec<String> {
    pub const SYMBOLS: [&str; 6] = [" ", ".", "/", "_", "-", "\\"];

    let mut words: Vec<String> = vec![];
    let mut temp_word: Vec<&str> = vec![];

    let vec_to_lowercase = |vec: &Vec<&str>| {
        vec.iter()
            .flat_map(|g| g.chars())
            .flat_map(|c| c.to_lowercase())
            .collect()
    };

    for c in input.graphemes(true) {
        // slice when a symbol is detected
        if SYMBOLS.contains(&c) {
            if !temp_word.is_empty() {
                words.push(vec_to_lowercase(&temp_word));
                temp_word.clear();
            }
            continue;
        }
        // slice when an uppercase letter is detected
        if is_uppercase(c) && !temp_word.is_empty() {
            words.push(vec_to_lowercase(&temp_word));
            temp_word.clear();
        }
        temp_word.push(c);
    }
    if !temp_word.is_empty() {
        words.push(vec_to_lowercase(&temp_word));
    }

    words
}

pub fn is_uppercase(character: &str) -> bool {
    let len = character.graphemes(true).count();
    if len != 1 {
        panic!("is_uppercase only take 1 character");
    }
    character == character.to_uppercase() && character != character.to_lowercase()
}

pub fn uppercase_first_letter(word: &str) -> String {
    let mut chars = word.graphemes(true);
    match chars.next() {
        None => panic!("Passing empty words"),
        Some(first_char) => {
            let mut res = first_char.to_uppercase();
            res.push_str(chars.as_str());
            res
        }
    }
}

#[cfg(test)]
mod utils_tests {
    mod uppercase_related {
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

        #[test]
        #[should_panic]
        fn test_uppercase_first_letter() {
            assert_eq!(
                uppercase_first_letter("ßenevolent"),
                "SSenevolent".to_string()
            );
            assert_eq!(uppercase_first_letter("ṁatsuri"), "Ṁatsuri".to_string());
            assert_eq!(
                uppercase_first_letter("夏色まつり"),
                "夏色まつり".to_string()
            );
            assert_eq!(
                uppercase_first_letter("normalForOnce"),
                "NormalForOnce".to_string()
            );
            assert_eq!(uppercase_first_letter("?"), "?".to_string());
            uppercase_first_letter("");
        }
    }

    mod test_slice_words {
        use crate::utils::*;

        use std::vec;

        #[test]
        fn slice_words_by_symbols() {
            let input = [
                String::from("god matsuri"),
                String::from("god.matsuri?"),
                String::from("god_matsuri_ahihihi"),
                String::from("god+matsuri"),
                String::from("god   / matsuri"),
            ];

            let expected_output = [
                vec![String::from("god"), String::from("matsuri")],
                vec![String::from("god"), String::from("matsuri?")],
                vec![
                    String::from("god"),
                    String::from("matsuri"),
                    String::from("ahihihi"),
                ],
                vec![String::from("god+matsuri")],
                vec![String::from("god"), String::from("matsuri")],
            ];

            let mut output: Vec<Vec<String>> = vec![];

            for s in input {
                output.push(slice_into_words(s));
            }

            assert_eq!(output, expected_output);
        }

        #[test]
        fn slice_words_by_symbols_with_utf8() {
            let input = [
                String::from("göd mätßurị?"),
                String::from("kami まつり"),
                String::from("gød mætsuri a hí hì hĩ hỉ hị"),
            ];

            let expected_output = [
                vec![String::from("göd"), String::from("mätßurị?")],
                vec![String::from("kami"), String::from("まつり")],
                vec![
                    String::from("gød"),
                    String::from("mætsuri"),
                    String::from("a"),
                    String::from("hí"),
                    String::from("hì"),
                    String::from("hĩ"),
                    String::from("hỉ"),
                    String::from("hị"),
                ],
            ];

            let mut output: Vec<Vec<String>> = vec![];

            for s in input {
                output.push(slice_into_words(s));
            }

            assert_eq!(output, expected_output);
        }

        #[test]
        fn slice_words_by_uppercase_with_utf8() {
            let input = [
                String::from("GodMatsuri"),
                String::from("GodÄtsuri?"),
                String::from("GodSatsuriAhihihi"),
                String::from("god"),
                String::from("God?"),
                String::from("ĞodMatsuRiÍsDaBét"),
            ];
            let expected_output = [
                vec![String::from("god"), String::from("matsuri")],
                vec![String::from("god"), String::from("ätsuri?")],
                vec![
                    String::from("god"),
                    String::from("satsuri"),
                    String::from("ahihihi"),
                ],
                vec![String::from("god")],
                vec![String::from("god?")],
                vec![
                    String::from("ğod"),
                    String::from("matsu"),
                    String::from("ri"),
                    String::from("ís"),
                    String::from("da"),
                    String::from("bét"),
                ],
            ];

            let mut output: Vec<Vec<String>> = vec![];

            for s in input {
                output.push(slice_into_words(s));
            }

            assert_eq!(output, expected_output);
        }

        #[test]
        fn slice_words_by_all_methods() {
            let input = [
                String::from("God.Äts.uri!________"),
                String::from("God Ṁatsuri 角巻わため"),
                String::from("_Ğod-Matsu-Ri-Ís_Da Bét  "),
            ];
            let expected_output = [
                vec![
                    String::from("god"),
                    String::from("äts"),
                    String::from("uri!"),
                ],
                vec![
                    String::from("god"),
                    String::from("ṁatsuri"),
                    String::from("角巻わため"),
                ],
                vec![
                    String::from("ğod"),
                    String::from("matsu"),
                    String::from("ri"),
                    String::from("ís"),
                    String::from("da"),
                    String::from("bét"),
                ],
            ];

            let mut output: Vec<Vec<String>> = vec![];

            for s in input {
                output.push(slice_into_words(s));
            }

            assert_eq!(output, expected_output);
        }
    }
}
