use unicode_segmentation::UnicodeSegmentation;

pub fn slice_into_words(input: String) -> Vec<String> {
    pub const SYMBOLS: [&str; 6] = [" ", ".", "/", "_", "-", "\\"];

    let mut words: Vec<String> = vec![];
    let mut temp_word: Vec<&str> = vec![];

    let vec_to_lowercase = |vec: &Vec<&str>| {
        (*vec)
            .clone()
            .into_iter()
            .collect::<String>()
            .to_lowercase()
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
