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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> Config {
        Config {
            hard_mode: false,
            max_guesses: 6,
            stats: false,
            reset_stats: false,
            word: None,
            no_color: false,
        }
    }

    #[test]
    fn test_default_config_valid() {
        let config = create_test_config();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_max_guesses_validation_zero() {
        let mut config = create_test_config();
        config.max_guesses = 0;

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Max guesses must be between 1 and 10"));
    }

    #[test]
    fn test_max_guesses_validation_too_high() {
        let mut config = create_test_config();
        config.max_guesses = 11;

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Max guesses must be between 1 and 10"));
    }

    #[test]
    fn test_max_guesses_validation_valid_range() {
        for max_guesses in 1..=10 {
            let mut config = create_test_config();
            config.max_guesses = max_guesses;
            assert!(config.validate().is_ok());
        }
    }

    #[test]
    fn test_word_validation_correct_length() {
        let mut config = create_test_config();
        config.word = Some("apple".to_string());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_word_validation_too_short() {
        let mut config = create_test_config();
        config.word = Some("test".to_string());

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 5 letters"));
    }

    #[test]
    fn test_word_validation_too_long() {
        let mut config = create_test_config();
        config.word = Some("testing".to_string());

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exactly 5 letters"));
    }

    #[test]
    fn test_word_validation_uppercase() {
        let mut config = create_test_config();
        config.word = Some("APPLE".to_string());

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("lowercase letters"));
    }

    #[test]
    fn test_word_validation_mixed_case() {
        let mut config = create_test_config();
        config.word = Some("Apple".to_string());

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("lowercase letters"));
    }

    #[test]
    fn test_word_validation_numbers() {
        let mut config = create_test_config();
        config.word = Some("test1".to_string());

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("lowercase letters"));
    }

    #[test]
    fn test_word_validation_special_chars() {
        let mut config = create_test_config();
        config.word = Some("test!".to_string());

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("lowercase letters"));
    }

    #[test]
    fn test_hard_mode_flag() {
        let mut config = create_test_config();
        config.hard_mode = true;
        assert!(config.validate().is_ok());
        assert!(config.hard_mode);
    }

    #[test]
    fn test_stats_flag() {
        let mut config = create_test_config();
        config.stats = true;
        assert!(config.validate().is_ok());
        assert!(config.stats);
    }

    #[test]
    fn test_reset_stats_flag() {
        let mut config = create_test_config();
        config.reset_stats = true;
        assert!(config.validate().is_ok());
        assert!(config.reset_stats);
    }

    #[test]
    fn test_no_color_flag() {
        let mut config = create_test_config();
        config.no_color = true;
        assert!(config.validate().is_ok());
        assert!(config.no_color);
    }

    #[test]
    fn test_combined_options() {
        let mut config = create_test_config();
        config.hard_mode = true;
        config.max_guesses = 4;
        config.word = Some("table".to_string());

        assert!(config.validate().is_ok());
        assert!(config.hard_mode);
        assert_eq!(config.max_guesses, 4);
        assert_eq!(config.word, Some("table".to_string()));
    }

    #[test]
    fn test_constants() {
        assert_eq!(WORD_LENGTH, 5);
        assert_eq!(DEFAULT_MAX_GUESSES, 6);
        assert_eq!(ALPHABET_SIZE, 26);
        assert_eq!(ASCII_LOWERCASE_A, 97);
        assert_eq!(MIN_TERMINAL_WIDTH, 50);
        assert_eq!(MIN_TERMINAL_HEIGHT, 20);
        assert_eq!(CHAR_SPACING, 4);
        assert_eq!(LINE_SPACING, 2);
    }
}