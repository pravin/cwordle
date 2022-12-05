// Responsible for managing game state

use super::words;

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
    winning_word: &'static str,
    guess_count: usize,
    words_guessed: Vec<String>,
    game_state: GameStatus,
    keyboard_helper: [KeyBoardHelperState; 26],
}

impl GameData {
    pub fn new() -> GameData {
        GameData {
            winning_word: words::choose_word(),
            guess_count: 0,
            words_guessed: Vec::new(),
            game_state: GameStatus::PLAYING,
            keyboard_helper: [KeyBoardHelperState::NONE; 26],
        }
    }

    pub fn new_guess(&mut self, guess: &String) -> GameStatus {
        assert!(self.guess_count < 6);
        if guess.eq(self.winning_word) {
            self.game_state = GameStatus::WON;
        } else {
            self.words_guessed.push(guess.clone());
            self.guess_count += 1;
            if self.guess_count > 5 {
                self.game_state = GameStatus::LOST;
            }
        }

        // Update keyboard helper
        let guess_chars: Vec<char> = guess.chars().collect();
        let winning_chars: Vec<char> = self.winning_word.chars().collect();
        for i in 0..5 {
            let char_index = guess_chars[i] as usize - 97; // 97 => lowercase 'a'
            if self.keyboard_helper[char_index] == KeyBoardHelperState::GREEN {
                continue;
            }
            if guess_chars[i] == winning_chars[i] {
                self.keyboard_helper[char_index] = KeyBoardHelperState::GREEN;
                continue;
            }

            if self.keyboard_helper[char_index] == KeyBoardHelperState::AMBER {
                continue;
            }
            if self.winning_word.contains(guess_chars[i]) {
                self.keyboard_helper[char_index] = KeyBoardHelperState::AMBER;
                continue;
            }

            self.keyboard_helper[char_index] = KeyBoardHelperState::GRAY;
        }

        self.game_state
    }

    pub fn status(&self) -> GameStatus {
        self.game_state
    }

    pub fn winning_word(&self) -> &'static str {
        self.winning_word
    }

    pub fn guess_count(&self) -> usize {
        self.guess_count
    }

    pub fn keyboard_helper(&self) -> [KeyBoardHelperState; 26] {
        self.keyboard_helper
    }
}
