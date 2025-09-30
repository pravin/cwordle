use clap::Parser;

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Enable hard mode (revealed letters must be used in subsequent guesses)
    #[arg(long, short = 'H')]
    pub hard_mode: bool,

    /// Maximum number of guesses allowed (default: 6)
    #[arg(long, short, default_value_t = 6)]
    pub max_guesses: usize,

    /// Show game statistics
    #[arg(long, short)]
    pub stats: bool,

    /// Reset all statistics
    #[arg(long)]
    pub reset_stats: bool,

    /// Use a specific word for testing (5 letters only)
    #[arg(long, value_name = "WORD")]
    pub word: Option<String>,

    /// Disable colors (for accessibility)
    #[arg(long)]
    pub no_color: bool,
}

impl Config {
    pub fn parse_args() -> Self {
        Config::parse()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.max_guesses == 0 || self.max_guesses > 10 {
            return Err("Max guesses must be between 1 and 10".to_string());
        }

        if let Some(ref word) = self.word {
            if word.len() != 5 {
                return Err("Custom word must be exactly 5 letters".to_string());
            }
            if !word.chars().all(|c| c.is_ascii_lowercase()) {
                return Err("Custom word must contain only lowercase letters".to_string());
            }
        }

        Ok(())
    }
}

// Game constants
pub const WORD_LENGTH: usize = 5;
pub const DEFAULT_MAX_GUESSES: usize = 6;
pub const ALPHABET_SIZE: usize = 26;
pub const ASCII_LOWERCASE_A: usize = 97;

// UI constants
pub const MIN_TERMINAL_WIDTH: i32 = 50;
pub const MIN_TERMINAL_HEIGHT: i32 = 20;
pub const CHAR_SPACING: i32 = 4;
pub const LINE_SPACING: i32 = 2;