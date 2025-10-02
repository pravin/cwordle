// Integration tests for cwordle
// These tests verify the interaction between different modules

use cwordle::*;

mod game_flow_tests {
    use super::*;

    #[test]
    fn test_complete_winning_game() {
        // Create a test configuration
        let config = config::Config {
            hard_mode: false,
            max_guesses: 6,
            stats: false,
            reset_stats: false,
            word: Some("apple".to_string()),
            no_color: false,
        };

        assert!(config.validate().is_ok());

        // Initialize game
        let mut game = gamestate::GameData::new(&config);
        assert_eq!(game.status(), gamestate::GameStatus::PLAYING);

        // Make incorrect guesses
        game.new_guess(&"tests".to_string()).unwrap();
        assert_eq!(game.status(), gamestate::GameStatus::PLAYING);
        assert_eq!(game.guess_count(), 1);

        game.new_guess(&"bread".to_string()).unwrap();
        assert_eq!(game.status(), gamestate::GameStatus::PLAYING);
        assert_eq!(game.guess_count(), 2);

        // Make winning guess
        game.new_guess(&"apple".to_string()).unwrap();
        assert_eq!(game.status(), gamestate::GameStatus::WON);
        assert_eq!(game.guess_count(), 3);
    }

    #[test]
    fn test_complete_losing_game() {
        let config = config::Config {
            hard_mode: false,
            max_guesses: 3,
            stats: false,
            reset_stats: false,
            word: Some("apple".to_string()),
            no_color: false,
        };

        let mut game = gamestate::GameData::new(&config);

        // Use all guesses without winning
        game.new_guess(&"tests".to_string()).unwrap();
        game.new_guess(&"bread".to_string()).unwrap();
        game.new_guess(&"lemon".to_string()).unwrap();

        assert_eq!(game.status(), gamestate::GameStatus::LOST);
        assert_eq!(game.guess_count(), 3);
    }

    #[test]
    fn test_hard_mode_full_game() {
        let config = config::Config {
            hard_mode: true,
            max_guesses: 6,
            stats: false,
            reset_stats: false,
            word: Some("apple".to_string()),
            no_color: false,
        };

        let mut game = gamestate::GameData::new(&config);

        // First guess reveals 'a', 'l', 'e'
        game.new_guess(&"ankle".to_string()).unwrap();
        assert!(game.revealed_letters().contains(&'a'));
        assert!(game.revealed_letters().contains(&'l'));
        assert!(game.revealed_letters().contains(&'e'));

        // Second guess must use revealed letters
        let result = game.new_guess(&"birds".to_string());
        assert!(result.is_err()); // Doesn't contain 'a', 'l', or 'e'

        // Valid guess in hard mode
        let result = game.new_guess(&"table".to_string());
        assert!(result.is_ok());
    }
}

mod word_validation_tests {
    use super::*;

    #[test]
    fn test_word_list_integrity() {
        // Verify word list is loaded
        let word = words::choose_word();
        assert_eq!(word.len(), 5);
        assert!(words::word_isvalid(word));
    }

    #[test]
    fn test_random_word_selection() {
        // Test that multiple calls return valid words
        let mut words_seen = std::collections::HashSet::new();

        for _ in 0..20 {
            let word = words::choose_word();
            assert_eq!(word.len(), 5);
            assert!(words::word_isvalid(word));
            words_seen.insert(word);
        }

        // Should have some variety (at least a few different words)
        assert!(words_seen.len() > 1);
    }

    #[test]
    fn test_word_validation_consistency() {
        // Test common valid words
        let valid_words = ["apple", "tests", "bread", "table", "chair"];
        for word in &valid_words {
            assert!(words::word_isvalid(word), "Word '{}' should be valid", word);
        }

        // Test invalid words
        let invalid_words = ["xyz", "zzzzz", "test", "testing", "APPLE"];
        for word in &invalid_words {
            assert!(!words::word_isvalid(word), "Word '{}' should be invalid", word);
        }
    }
}

mod stats_integration_tests {
    use super::*;

    #[test]
    fn test_stats_with_game_results() {
        let mut stats = stats::GameStats::default();

        // Simulate a game session
        stats.record_game(true, Some(3));
        stats.record_game(true, Some(4));
        stats.record_game(false, None);

        assert_eq!(stats.games_played, 3);
        assert_eq!(stats.games_won, 2);
        assert_eq!(stats.current_streak, 0); // Broken by loss
        assert_eq!(stats.max_streak, 2);
    }

    #[test]
    fn test_stats_guess_distribution_accuracy() {
        let mut stats = stats::GameStats::default();

        // Record various winning games
        for i in 1..=6 {
            stats.record_game(true, Some(i));
        }

        // Verify distribution
        for i in 1..=6 {
            assert_eq!(stats.guess_distribution.get(&(i as u32)), Some(&1));
        }

        assert_eq!(stats.average_guesses(), 3.5); // (1+2+3+4+5+6)/6 = 3.5
    }
}

mod config_integration_tests {
    use super::*;

    #[test]
    fn test_config_with_game_initialization() {
        let config = config::Config {
            hard_mode: true,
            max_guesses: 4,
            stats: false,
            reset_stats: false,
            word: Some("tests".to_string()),
            no_color: false,
        };

        assert!(config.validate().is_ok());

        let game = gamestate::GameData::new(&config);
        assert_eq!(game.max_guesses(), 4);
        assert_eq!(game.winning_word(), "tests");
        assert!(game.is_hard_mode());
    }

    #[test]
    fn test_invalid_config_catches_errors() {
        let mut config = config::Config {
            hard_mode: false,
            max_guesses: 0,
            stats: false,
            reset_stats: false,
            word: None,
            no_color: false,
        };

        assert!(config.validate().is_err());

        config.max_guesses = 6;
        config.word = Some("TEST".to_string()); // Uppercase
        assert!(config.validate().is_err());

        config.word = Some("test".to_string()); // Too short
        assert!(config.validate().is_err());

        config.word = Some("tests".to_string()); // Valid
        assert!(config.validate().is_ok());
    }
}

mod keyboard_helper_tests {
    use super::*;

    #[test]
    fn test_keyboard_state_progression() {
        let config = config::Config {
            hard_mode: false,
            max_guesses: 6,
            stats: false,
            reset_stats: false,
            word: Some("apple".to_string()),
            no_color: false,
        };

        let mut game = gamestate::GameData::new(&config);

        // First guess: "plumb" - 'p' and 'l' are in "apple" but wrong positions
        game.new_guess(&"plumb".to_string()).unwrap();

        let keyboard = game.keyboard_helper();
        let p_index = 'p' as usize - config::ASCII_LOWERCASE_A;
        let l_index = 'l' as usize - config::ASCII_LOWERCASE_A;
        let u_index = 'u' as usize - config::ASCII_LOWERCASE_A;

        // 'p' should be AMBER (exists but wrong position)
        assert_eq!(keyboard[p_index], gamestate::KeyBoardHelperState::AMBER);

        // 'l' should be AMBER (exists but wrong position)
        assert_eq!(keyboard[l_index], gamestate::KeyBoardHelperState::AMBER);

        // 'u' should be GRAY (not in word)
        assert_eq!(keyboard[u_index], gamestate::KeyBoardHelperState::GRAY);
    }
}

mod edge_case_tests {
    use super::*;

    #[test]
    fn test_win_on_last_guess() {
        let config = config::Config {
            hard_mode: false,
            max_guesses: 3,
            stats: false,
            reset_stats: false,
            word: Some("apple".to_string()),
            no_color: false,
        };

        let mut game = gamestate::GameData::new(&config);

        // Use first two guesses
        game.new_guess(&"tests".to_string()).unwrap();
        game.new_guess(&"bread".to_string()).unwrap();

        // Win on last guess
        game.new_guess(&"apple".to_string()).unwrap();

        assert_eq!(game.status(), gamestate::GameStatus::WON);
        assert_eq!(game.guess_count(), 3);
    }

    #[test]
    fn test_word_with_repeated_letters() {
        let config = config::Config {
            hard_mode: false,
            max_guesses: 6,
            stats: false,
            reset_stats: false,
            word: Some("level".to_string()),
            no_color: false,
        };

        let mut game = gamestate::GameData::new(&config);

        // Test guess with repeated letters
        game.new_guess(&"eerie".to_string()).unwrap();

        let keyboard = game.keyboard_helper();
        let e_index = 'e' as usize - config::ASCII_LOWERCASE_A;

        // 'e' appears in the word, so should be at least AMBER
        assert!(
            keyboard[e_index] == gamestate::KeyBoardHelperState::AMBER
                || keyboard[e_index] == gamestate::KeyBoardHelperState::GREEN
        );
    }

    #[test]
    fn test_immediate_win() {
        let config = config::Config {
            hard_mode: false,
            max_guesses: 6,
            stats: false,
            reset_stats: false,
            word: Some("tests".to_string()),
            no_color: false,
        };

        let mut game = gamestate::GameData::new(&config);

        // Win on first guess
        game.new_guess(&"tests".to_string()).unwrap();

        assert_eq!(game.status(), gamestate::GameStatus::WON);
        assert_eq!(game.guess_count(), 1);
    }
}
