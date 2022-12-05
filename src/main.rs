use std::io;
mod gamestate;
mod gameui;
mod words;


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
