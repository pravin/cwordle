//! Word management module
//! Handles word selection and validation

use rand::{prelude::ThreadRng, Rng};

/// The word list containing all valid 5-letter words
/// Each word is exactly 5 characters followed by a newline
static WORD_LIST: &str = include_str!("word-list.txt");

/// Word length constant
const WORD_LENGTH: usize = 5;
/// Line length constant (word + newline)
const LINE_LENGTH: usize = WORD_LENGTH + 1;

/// Returns a random word from WORD_LIST
/// 
/// Uses efficient indexing based on the fixed-length nature of the words.
/// Each word is exactly 5 letters + newline (6 bytes total), allowing
/// direct calculation of word positions.
#[inline]
pub fn choose_word() -> &'static str {
    let mut rng: ThreadRng = rand::thread_rng();
    
    // Calculate total number of words
    let word_count = WORD_LIST.len() / LINE_LENGTH;
    
    // Select random word position
    let word_index = rng.gen_range(0..word_count);
    let start = word_index * LINE_LENGTH;
    
    // Return slice containing just the word (excluding newline)
    &WORD_LIST[start..start + WORD_LENGTH]
}

/// Validates if a word exists in the word list
/// 
/// # Arguments
/// * `word` - The word to validate
/// 
/// # Returns
/// `true` if the word is in the list, `false` otherwise
pub fn word_isvalid(word: &str) -> bool {
    // Quick length check
    if word.len() != WORD_LENGTH {
        return false;
    }

    // Check if word contains only lowercase letters
    if !word.chars().all(|c| c.is_ascii_lowercase()) {
        return false;
    }

    // Search for word in list using line-based iteration
    WORD_LIST.as_bytes()
        .chunks(LINE_LENGTH)
        .any(|line| line[..WORD_LENGTH].eq(word.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choose_word_length() {
        let word = choose_word();
        assert_eq!(word.len(), WORD_LENGTH);
    }

    #[test]
    fn test_choose_word_lowercase() {
        let word = choose_word();
        assert!(word.chars().all(|c| c.is_ascii_lowercase()));
    }

    #[test]
    fn test_choose_word_distribution() {
        // Test that we get different words
        let mut words = std::collections::HashSet::new();
        for _ in 0..100 {
            words.insert(choose_word());
        }
        // We should get at least 50 different words in 100 tries
        assert!(words.len() > 50);
    }

    #[test]
    fn test_word_validation() {
        // Get a known valid word from the list
        let valid_word = choose_word();
        assert!(word_isvalid(valid_word));
        
        // Test invalid words
        assert!(!word_isvalid(""));
        assert!(!word_isvalid("ab"));
        assert!(!word_isvalid("toolong"));
        assert!(!word_isvalid("12345"));
        assert!(!word_isvalid("UPPER"));
        assert!(!word_isvalid("mix@d"));
    }

    #[test]
    fn test_word_boundaries() {
        // Test that partial matches within other words don't count
        let word = choose_word();
        let partial = &word[0..3];
        assert!(!word_isvalid(partial));
        
        // Test that exact matches work
        assert!(word_isvalid(word));
    }

    #[test]
    fn test_word_list_format() {
        // Verify word list format
        for line in WORD_LIST.lines() {
            assert_eq!(line.len(), WORD_LENGTH);
            assert!(line.chars().all(|c| c.is_ascii_lowercase()));
        }
    }
}
