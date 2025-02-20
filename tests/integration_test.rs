use cwordle::gamestate::{GameData, GameStatus};
use cwordle::words;

#[test]
fn test_game_flow() {
    let mut game = GameData::new();
    let winning_word = game.winning_word();
    
    // First try a wrong guess
    let wrong_guess = if winning_word == "happy" { 
        "world".to_string() 
    } else { 
        "happy".to_string() 
    };
    
    game.new_guess(&wrong_guess);
    assert_eq!(game.status(), GameStatus::PLAYING);
    assert_eq!(game.guess_count(), 1);
    
    // Then win with correct word
    game.new_guess(&winning_word.to_string());
    assert_eq!(game.status(), GameStatus::WON);
}

#[test]
fn test_word_validation_integration() {
    // Test that chosen words are always valid
    let word = words::choose_word();
    assert!(words::word_isvalid(word));
}

#[test]
fn test_invalid_input_handling() {
    let game = GameData::new();
    let invalid_inputs = vec!["", "test", "toolong", "12345"];
    
    for input in invalid_inputs {
        assert!(!words::word_isvalid(input));
    }
} 