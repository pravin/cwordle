//! Game state management module
//! Handles the core game logic and state tracking

use super::words;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GameStatus {
    PLAYING,
    WON,
    LOST,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum KeyBoardHelperState {
    NONE,
    GRAY,
    AMBER,
    GREEN,
}

/// Represents the current state of the game
pub struct GameData {
    winning_word: &'static str,
    guess_count: usize,
    words_guessed: Vec<String>,
    game_state: GameStatus,
    keyboard_helper: [KeyBoardHelperState; 26],
}

impl GameData {
    /// Creates a new game with a randomly chosen word
    pub fn new() -> Self {
        Self {
            winning_word: words::choose_word(),
            guess_count: 0,
            words_guessed: Vec::with_capacity(6), // Pre-allocate for max guesses
            game_state: GameStatus::PLAYING,
            keyboard_helper: [KeyBoardHelperState::NONE; 26],
        }
    }

    /// Processes a new guess and updates the game state
    /// Returns the current game status
    /// 
    /// # Arguments
    /// * `guess` - The player's guess word
    /// 
    /// # Panics
    /// Panics if more than 6 guesses are attempted
    pub fn new_guess(&mut self, guess: &str) -> GameStatus {
        assert!(self.guess_count < 6, "Maximum number of guesses exceeded");
        
        if guess == self.winning_word {
            self.game_state = GameStatus::WON;
        } else {
            self.words_guessed.push(guess.to_string());
            self.guess_count += 1;
            if self.guess_count >= 6 {
                self.game_state = GameStatus::LOST;
            }
        }

        self.update_keyboard_helper(guess);
        self.game_state
    }

    /// Updates the keyboard helper based on the current guess
    fn update_keyboard_helper(&mut self, guess: &str) {
        let winning_chars: Vec<char> = self.winning_word.chars().collect();
        let mut used_positions = [false; 5]; // Track matched positions

        // First pass: Mark exact matches (GREEN)
        for (i, c) in guess.chars().enumerate() {
            let char_index = c as usize - b'a' as usize;
            if c == winning_chars[i] {
                self.keyboard_helper[char_index] = KeyBoardHelperState::GREEN;
                used_positions[i] = true;
            }
        }

        // Second pass: Mark partial matches (AMBER) and misses (GRAY)
        for (i, c) in guess.chars().enumerate() {
            let char_index = c as usize - b'a' as usize;
            
            // Skip if already marked GREEN
            if self.keyboard_helper[char_index] == KeyBoardHelperState::GREEN {
                continue;
            }

            // Check if letter exists in an unused position
            let exists = winning_chars.iter().enumerate().any(|(pos, &wc)| {
                wc == c && !used_positions[pos] && winning_chars[i] != c
            });

            self.keyboard_helper[char_index] = if exists {
                KeyBoardHelperState::AMBER
            } else {
                KeyBoardHelperState::GRAY
            };
        }
    }

    /// Returns the current game status
    #[inline]
    pub fn status(&self) -> GameStatus {
        self.game_state
    }

    /// Returns the winning word
    #[inline]
    pub fn winning_word(&self) -> &'static str {
        self.winning_word
    }

    /// Returns the number of guesses made
    #[inline]
    pub fn guess_count(&self) -> usize {
        self.guess_count
    }

    /// Returns the current keyboard helper state
    #[inline]
    pub fn keyboard_helper(&self) -> [KeyBoardHelperState; 26] {
        self.keyboard_helper
    }

    /// Returns a reference to the guessed words
    #[inline]
    pub fn guessed_words(&self) -> &[String] {
        &self.words_guessed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = GameData::new();
        assert_eq!(game.status(), GameStatus::PLAYING);
        assert_eq!(game.guess_count(), 0);
        assert_eq!(game.winning_word().len(), 5);
        assert!(game.guessed_words().is_empty());
    }

    #[test]
    fn test_correct_guess_wins() {
        let mut game = GameData::new();
        let winning_word = game.winning_word();
        
        let status = game.new_guess(winning_word);
        assert_eq!(status, GameStatus::WON);
        assert_eq!(game.status(), GameStatus::WON);
        assert_eq!(game.guess_count(), 0);
    }

    #[test]
    fn test_six_wrong_guesses_loses() {
        let mut game = GameData::new();
        let wrong_word = String::from("wrong");
        
        for i in 0..6 {
            let status = game.new_guess(&wrong_word);
            if i < 5 {
                assert_eq!(status, GameStatus::PLAYING);
            } else {
                assert_eq!(status, GameStatus::LOST);
            }
        }
        assert_eq!(game.guess_count(), 6);
    }

    #[test]
    fn test_keyboard_helper_updates() {
        let mut game = GameData::new();
        let winning_word = game.winning_word();
        
        // Test exact match (GREEN)
        let first_char = winning_word.chars().next().unwrap();
        let guess = format!("{}aaaa", first_char);
        game.new_guess(&guess);
        
        let helper = game.keyboard_helper();
        let first_idx = first_char as usize - b'a' as usize;
        assert_eq!(helper[first_idx], KeyBoardHelperState::GREEN);
        
        // Test partial match (AMBER)
        if let Some(c) = winning_word.chars().skip(1).next() {
            let guess = format!("{}aaaa", c);
            game.new_guess(&guess);
            let idx = c as usize - b'a' as usize;
            assert_eq!(helper[idx], KeyBoardHelperState::AMBER);
        }
    }

    #[test]
    #[should_panic(expected = "Maximum number of guesses exceeded")]
    fn test_too_many_guesses_panics() {
        let mut game = GameData::new();
        let wrong_word = "wrong";
        
        for _ in 0..7 {
            game.new_guess(wrong_word);
        }
    }
}
