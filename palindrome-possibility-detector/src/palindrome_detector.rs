use std::collections::HashMap;

pub trait PalindromePossibilityDetector {
    fn could_be_a_palindrome(&self, candidate: &str) -> bool;
}

pub struct EasyPalindromePossibilityDetector;

impl EasyPalindromePossibilityDetector {
    pub fn new() -> Self {
        EasyPalindromePossibilityDetector { }
    }
}

impl PalindromePossibilityDetector for EasyPalindromePossibilityDetector {
    fn could_be_a_palindrome(&self, candidate: &str) -> bool {
        let mut counts: HashMap<Vec<char>, usize> = HashMap::new();

        for ch in candidate.chars() {
            if ch.is_alphabetic() {
                let lowercase_ch = ch.to_lowercase();
                let mut chars = Vec::new();
                for ch in lowercase_ch {
                    chars.push(ch);
                }

                let mut new_char_count = 1;
                match counts.get(&chars) {
                    Some(v) => new_char_count = v + 1,
                    None => {},
                };
                counts.insert(chars, new_char_count);
            }
        }

        let mut found_an_odd_count = false;
        for value in counts.values() {
            if value % 2 == 1 {
                if found_an_odd_count {
                    return false;
                }

                found_an_odd_count = true;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_could_be_a_palindrome() {
        let candidate = String::new();
        let palindrome_detector = EasyPalindromePossibilityDetector::new();
        let result = palindrome_detector.could_be_a_palindrome(&candidate);
        assert_eq!(true, result);
    }

    #[test]
    fn single_character_string_could_be_a_palindrome() {
        let candidate = String::from("z");
        let palindrome_detector = EasyPalindromePossibilityDetector::new();
        let result = palindrome_detector.could_be_a_palindrome(&candidate);
        assert_eq!(true, result);
    }

    #[test]
    fn double_mismatched_character_string_could_not_be_a_palindrome() {
        let candidate = String::from("zx");
        let palindrome_detector = EasyPalindromePossibilityDetector::new();
        let result = palindrome_detector.could_be_a_palindrome(&candidate);
        assert_eq!(false, result);
    }

    #[test]
    fn double_matched_character_string_could_be_a_palindrome() {
        let candidate = String::from("zz");
        let palindrome_detector = EasyPalindromePossibilityDetector::new();
        let result = palindrome_detector.could_be_a_palindrome(&candidate);
        assert_eq!(true, result);
    }

    #[test]
    fn double_matched_different_case_character_string_could_be_a_palindrome() {
        let candidate = String::from("zZ");
        let palindrome_detector = EasyPalindromePossibilityDetector::new();
        let result = palindrome_detector.could_be_a_palindrome(&candidate);
        assert_eq!(true, result);
    }

    #[test]
    fn multi_character_string_could_be_a_palindrome() {
        let candidate = String::from("aBAb");
        let palindrome_detector = EasyPalindromePossibilityDetector::new();
        let result = palindrome_detector.could_be_a_palindrome(&candidate);
        assert_eq!(true, result);
    }

    #[test]
    fn space_included_character_string_could_be_a_palindrome() {
        let candidate = String::from("aBAm b");
        let palindrome_detector = EasyPalindromePossibilityDetector::new();
        let result = palindrome_detector.could_be_a_palindrome(&candidate);
        assert_eq!(true, result);
    }
}