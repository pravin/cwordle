use rand::{prelude::ThreadRng, Rng};
use std::sync::OnceLock;

static WORD_LIST_RAW: &'static str = include_str!("word-list.txt");
static WORD_LIST: OnceLock<Vec<&'static str>> = OnceLock::new();

fn get_word_list() -> &'static Vec<&'static str> {
    WORD_LIST.get_or_init(|| {
        let mut words: Vec<&str> = WORD_LIST_RAW
            .lines()
            .map(|w| w.trim())
            .filter(|w| w.len() == 5 && w.chars().all(|c| c.is_ascii_lowercase()))
            .collect();
        words.sort_unstable();
        words
    })
}

pub fn choose_word() -> &'static str {
    /*!
    Returns a random word from the pre-parsed word list
    */
    let mut rng: ThreadRng = rand::thread_rng();
    let words = get_word_list();
    let index = rng.gen_range(0..words.len());

    words[index]
}

pub fn word_isvalid(word: &str) -> bool {
    // Check if word is exactly 5 characters and exists in dictionary
    if word.len() != 5 {
        return false;
    }

    // Use binary search on the pre-sorted word list for better performance
    get_word_list().binary_search(&word).is_ok()
}

pub fn get_word_count() -> usize {
    get_word_list().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_validation() {
        // Test valid words
        assert!(word_isvalid("abaci"));
        assert!(word_isvalid("aback"));

        // Test invalid words
        assert!(!word_isvalid("xyz"));    // not in dictionary
        assert!(!word_isvalid("abac"));   // too short
        assert!(!word_isvalid("abacis")); // too long
        assert!(!word_isvalid("ABACI"));  // wrong case

        // Test partial matches (should not work)
        assert!(!word_isvalid("abac"));   // partial match of "abaci"
    }

    #[test]
    fn test_word_selection() {
        // Test that choose_word returns valid words
        for _ in 0..10 {
            let word = choose_word();
            assert_eq!(word.len(), 5);
            assert!(word_isvalid(word));
        }
    }
}
