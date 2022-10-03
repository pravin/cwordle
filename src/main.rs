use std::io;
mod game;
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
    let win = game::init_game();
    game::game_loop(&win);
    game::end_game();

    Ok(())
}
