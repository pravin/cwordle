// Responsible for managing game state

use super::words;
use super::config::{Config, ALPHABET_SIZE, ASCII_LOWERCASE_A, WORD_LENGTH};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameStatus {
    PLAYING = 0,
    WON = 1,
    LOST = 2,
}

#[derive(Copy, Clone, PartialEq, Eq)]
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
