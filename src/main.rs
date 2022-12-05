use std::io;
mod gamestate;
mod gameui;
mod words;

/*

fn show_help() {
    let mut color_array = [0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1];
    color_array.shuffle(&mut rand::thread_rng());
    println!();
    print_text("How to Play".to_string(), color_array.to_vec());
    println!("\n\nGuess the word in 6 tries to win.");
    println!("After each guess, the colors will help show how close your guess was.");
    println!("For example, if the word was glued and you typed grape\n");

    print_text("grape".to_string(), [1, 0, 0, 0, 2].to_vec());
    println!("\n1. The letter g is in the right place.");
    println!("\n2. The letter e is present in the word, but in the wrong place.");
    println!("\n3. The letters r, a and p aren't present in the word.");
    std::process::exit(0);
}

 */
fn main() -> io::Result<()> {
    // Init game screen or panic
    let win = gameui::init_game();
    // Init GameState
    let mut game_data = gamestate::GameData::new();
    // Loop while game hasn't finished
    while game_data.status() == gamestate::GameStatus::PLAYING {
        // Draw / Update keyboard
        gameui::draw_keyboard(&win, &game_data);

        // Get User Input
        let mut user_input: String;
        loop {
            user_input = gameui::get_user_input(&win, &game_data);
            if words::word_isvalid(&user_input) {
                break;
            } else {
                gameui::clear_incorrect_word(&win, &game_data);
            }
        }

        // Update UI
        gameui::color_input_word(&win, &game_data, &user_input);

        // Update State
        game_data.new_guess(&user_input);
    }
    // Update UI with Game State
    // Play Again?
    //let game_data1 = gameui::game_loop(&win);
    gameui::end_game(&win, game_data);

    Ok(())
}
