// Responsible for managing game state

use super::words;
use super::config::{Config, ALPHABET_SIZE, ASCII_LOWERCASE_A, WORD_LENGTH};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GameStatus {
    PLAYING = 0,
    WON = 1,
    LOST = 2,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum KeyBoardHelperState {
    NONE = 0,
    GRAY = 1,
    AMBER = 2,
    GREEN = 3,
}
pub struct GameData {
    winning_word: String,
    guess_count: usize,
    max_guesses: usize,
    words_guessed: Vec<String>,
    game_state: GameStatus,
    keyboard_helper: [KeyBoardHelperState; ALPHABET_SIZE],
    hard_mode: bool,
    revealed_letters: Vec<char>, // Letters that must be used in hard mode
}

impl GameData {
    pub fn new(config: &Config) -> GameData {
        let winning_word = if let Some(ref custom_word) = config.word {
            custom_word.clone()
        } else {
            words::choose_word().to_string()
        };

        GameData {
            winning_word,
            guess_count: 0,
            max_guesses: config.max_guesses,
            words_guessed: Vec::new(),
            game_state: GameStatus::PLAYING,
            keyboard_helper: [KeyBoardHelperState::NONE; ALPHABET_SIZE],
            hard_mode: config.hard_mode,
            revealed_letters: Vec::new(),
        }
    }

    pub fn new_guess(&mut self, guess: &String) -> Result<GameStatus, String> {
        if self.guess_count >= self.max_guesses {
            return Err("Maximum guesses exceeded".to_string());
        }

        // Check hard mode constraints
        if self.hard_mode && !self.revealed_letters.is_empty() {
            for &letter in &self.revealed_letters {
                if !guess.contains(letter) {
                    return Err(format!("Hard mode: must use revealed letter '{}'", letter));
                }
            }
        }

        // Always increment guess count and add to guessed words
        self.words_guessed.push(guess.clone());
        self.guess_count += 1;

        if guess.eq(&self.winning_word) {
            self.game_state = GameStatus::WON;
        } else if self.guess_count >= self.max_guesses {
            self.game_state = GameStatus::LOST;
        }

        // Update keyboard helper and revealed letters for hard mode
        let guess_chars: Vec<char> = guess.chars().collect();
        let winning_chars: Vec<char> = self.winning_word.chars().collect();
        for i in 0..WORD_LENGTH {
            let ch = guess_chars[i];
            // Only process lowercase ASCII letters to avoid panic
            if !ch.is_ascii_lowercase() {
                continue;
            }
            let char_index = ch as usize - ASCII_LOWERCASE_A;
            if char_index >= ALPHABET_SIZE {
                continue; // Skip if somehow out of range
            }

            if self.keyboard_helper[char_index] == KeyBoardHelperState::GREEN {
                continue;
            }
            if guess_chars[i] == winning_chars[i] {
                self.keyboard_helper[char_index] = KeyBoardHelperState::GREEN;
                // Add to revealed letters for hard mode
                if self.hard_mode && !self.revealed_letters.contains(&ch) {
                    self.revealed_letters.push(ch);
                }
                continue;
            }

            if self.keyboard_helper[char_index] == KeyBoardHelperState::AMBER {
                continue;
            }
            if self.winning_word.contains(guess_chars[i]) {
                self.keyboard_helper[char_index] = KeyBoardHelperState::AMBER;
                // Add to revealed letters for hard mode
                if self.hard_mode && !self.revealed_letters.contains(&ch) {
                    self.revealed_letters.push(ch);
                }
                continue;
            }

            self.keyboard_helper[char_index] = KeyBoardHelperState::GRAY;
        }

        Ok(self.game_state)
    }

    pub fn status(&self) -> GameStatus {
        self.game_state
    }

    pub fn winning_word(&self) -> &str {
        &self.winning_word
    }

    pub fn max_guesses(&self) -> usize {
        self.max_guesses
    }

    pub fn is_hard_mode(&self) -> bool {
        self.hard_mode
    }

    pub fn revealed_letters(&self) -> &[char] {
        &self.revealed_letters
    }

    pub fn guess_count(&self) -> usize {
        self.guess_count
    }

    pub fn keyboard_helper(&self) -> [KeyBoardHelperState; ALPHABET_SIZE] {
        self.keyboard_helper
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config(hard_mode: bool, max_guesses: usize, word: Option<String>) -> Config {
        Config {
            hard_mode,
            max_guesses,
            stats: false,
            reset_stats: false,
            word,
            no_color: false,
        }
    }

    #[test]
    fn test_new_game_initialization() {
        let config = create_test_config(false, 6, Some("tests".to_string()));
        let game = GameData::new(&config);

        assert_eq!(game.status(), GameStatus::PLAYING);
        assert_eq!(game.guess_count(), 0);
        assert_eq!(game.winning_word(), "tests");
        assert_eq!(game.max_guesses(), 6);
        assert!(!game.is_hard_mode());
        assert_eq!(game.revealed_letters().len(), 0);
    }

    #[test]
    fn test_winning_guess() {
        let config = create_test_config(false, 6, Some("apple".to_string()));
        let mut game = GameData::new(&config);

        let result = game.new_guess(&"apple".to_string());
        assert!(result.is_ok());
        assert_eq!(game.status(), GameStatus::WON);
        assert_eq!(game.guess_count(), 1);
    }

    #[test]
    fn test_losing_game() {
        let config = create_test_config(false, 3, Some("apple".to_string()));
        let mut game = GameData::new(&config);

        game.new_guess(&"tests".to_string()).unwrap();
        assert_eq!(game.status(), GameStatus::PLAYING);

        game.new_guess(&"bread".to_string()).unwrap();
        assert_eq!(game.status(), GameStatus::PLAYING);

        game.new_guess(&"lemon".to_string()).unwrap();
        assert_eq!(game.status(), GameStatus::LOST);
        assert_eq!(game.guess_count(), 3);
    }

    #[test]
    fn test_keyboard_helper_correct_position() {
        let config = create_test_config(false, 6, Some("apple".to_string()));
        let mut game = GameData::new(&config);

        game.new_guess(&"ankle".to_string()).unwrap();

        let keyboard = game.keyboard_helper();
        let a_index = 'a' as usize - ASCII_LOWERCASE_A;
        assert_eq!(keyboard[a_index], KeyBoardHelperState::GREEN); // 'a' is correct
    }

    #[test]
    fn test_keyboard_helper_wrong_position() {
        let config = create_test_config(false, 6, Some("apple".to_string()));
        let mut game = GameData::new(&config);

        game.new_guess(&"plumb".to_string()).unwrap();

        let keyboard = game.keyboard_helper();
        let p_index = 'p' as usize - ASCII_LOWERCASE_A;
        assert_eq!(keyboard[p_index], KeyBoardHelperState::AMBER); // 'p' exists but wrong position
    }

    #[test]
    fn test_keyboard_helper_incorrect_letter() {
        let config = create_test_config(false, 6, Some("apple".to_string()));
        let mut game = GameData::new(&config);

        game.new_guess(&"tests".to_string()).unwrap();

        let keyboard = game.keyboard_helper();
        let t_index = 't' as usize - ASCII_LOWERCASE_A;
        assert_eq!(keyboard[t_index], KeyBoardHelperState::GRAY); // 't' not in word
    }

    #[test]
    fn test_hard_mode_revealed_letters() {
        let config = create_test_config(true, 6, Some("apple".to_string()));
        let mut game = GameData::new(&config);

        game.new_guess(&"ankle".to_string()).unwrap();

        assert!(game.revealed_letters().contains(&'a'));
        assert!(game.revealed_letters().contains(&'l'));
        assert!(game.revealed_letters().contains(&'e'));
    }

    #[test]
    fn test_hard_mode_constraint_violation() {
        let config = create_test_config(true, 6, Some("apple".to_string()));
        let mut game = GameData::new(&config);

        game.new_guess(&"ankle".to_string()).unwrap();

        // Try to guess a word without revealed letters
        let result = game.new_guess(&"birds".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must use revealed letter"));
    }

    #[test]
    fn test_hard_mode_constraint_satisfied() {
        let config = create_test_config(true, 6, Some("apple".to_string()));
        let mut game = GameData::new(&config);

        game.new_guess(&"ankle".to_string()).unwrap();

        // Guess contains revealed letters 'a', 'l', 'e'
        let result = game.new_guess(&"abler".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_guesses() {
        let config = create_test_config(false, 6, Some("table".to_string()));
        let mut game = GameData::new(&config);

        game.new_guess(&"tests".to_string()).unwrap();
        assert_eq!(game.guess_count(), 1);
        assert_eq!(game.status(), GameStatus::PLAYING);

        game.new_guess(&"train".to_string()).unwrap();
        assert_eq!(game.guess_count(), 2);
        assert_eq!(game.status(), GameStatus::PLAYING);

        game.new_guess(&"table".to_string()).unwrap();
        assert_eq!(game.guess_count(), 3);
        assert_eq!(game.status(), GameStatus::WON);
    }

    #[test]
    fn test_max_guesses_customization() {
        let config = create_test_config(false, 2, Some("apple".to_string()));
        let mut game = GameData::new(&config);

        game.new_guess(&"tests".to_string()).unwrap();
        assert_eq!(game.status(), GameStatus::PLAYING);

        game.new_guess(&"bread".to_string()).unwrap();
        assert_eq!(game.status(), GameStatus::LOST);
        assert_eq!(game.guess_count(), 2);
    }

    #[test]
    fn test_keyboard_helper_priority() {
        // Test that GREEN takes priority over AMBER
        let config = create_test_config(false, 6, Some("aback".to_string()));
        let mut game = GameData::new(&config);

        // First guess: 'a' is in wrong position (should be AMBER)
        game.new_guess(&"beans".to_string()).unwrap();

        // Second guess: 'a' is in correct position (should be GREEN)
        game.new_guess(&"abler".to_string()).unwrap();

        let keyboard = game.keyboard_helper();
        let a_index = 'a' as usize - ASCII_LOWERCASE_A;
        assert_eq!(keyboard[a_index], KeyBoardHelperState::GREEN);
    }
}
