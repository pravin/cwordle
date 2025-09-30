use std::io;
use std::process;

mod config;
mod gamestate;
mod gameui;
mod stats;
mod words;

fn main() -> io::Result<()> {
    // Parse command-line arguments
    let config = config::Config::parse_args();

    // Validate configuration
    if let Err(e) = config.validate() {
        eprintln!("Configuration error: {}", e);
        process::exit(1);
    }

    // Initialize statistics manager
    let mut stats_manager = match stats::StatsManager::new() {
        Ok(sm) => sm,
        Err(e) => {
            eprintln!("Warning: Could not load statistics: {}", e);
            return Ok(());
        }
    };

    // Handle special commands
    if config.stats {
        let win = gameui::init_game();
        gameui::show_stats(&win, &stats_manager);
        return Ok(());
    }

    if config.reset_stats {
        if let Err(e) = stats_manager.reset_stats() {
            eprintln!("Error resetting statistics: {}", e);
            process::exit(1);
        }
        println!("Statistics reset successfully!");
        return Ok(());
    }

    // Game loop
    loop {
        if !play_game(&config, &mut stats_manager)? {
            break;
        }
    }

    Ok(())
}

fn play_game(config: &config::Config, stats_manager: &mut stats::StatsManager) -> io::Result<bool> {
    // Init game screen or panic
    let win = gameui::init_game();

    // Clear the screen for a fresh game
    win.clear();
    win.refresh();

    // Redraw the header and initial UI
    gameui::redraw_game_screen(&win);

    // Init GameState
    let mut game_data = gamestate::GameData::new(config);

    // Initial UI setup
    gameui::update_footer(&win, &game_data);

    // Loop while game hasn't finished
    while game_data.status() == gamestate::GameStatus::PLAYING {
        // Draw / Update keyboard and UI
        gameui::draw_keyboard(&win, &game_data);
        gameui::update_footer(&win, &game_data);
        gameui::draw_empty_boxes(&win, &game_data); // Draw empty boxes for current input

        // Get User Input
        let user_input: String;
        loop {
            let input = gameui::get_user_input(&win, &game_data);

            if !words::word_isvalid(&input) {
                gameui::clear_incorrect_word(&win, &game_data, "Word not in dictionary");
                continue;
            }

            // Check hard mode constraints and update game state
            match game_data.new_guess(&input) {
                Ok(_) => {
                    user_input = input;
                    break;
                }
                Err(e) => {
                    gameui::clear_incorrect_word(&win, &game_data, &e);
                    continue;
                }
            }
        }

        // Update UI
        gameui::color_input_word(&win, &game_data, &user_input);
    }

    // End game and check if player wants to play again
    Ok(gameui::end_game(&win, game_data, stats_manager))
}
