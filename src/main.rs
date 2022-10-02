use std::io;
mod game;
mod words;

/*
fn get_input() -> io::Result<String> {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input = input[0..5].to_lowercase();
                if input.len() == 5 && WORD_LIST.contains(&input) {
                    return Ok(input);
                }

                print!("\x1b[1A\tPlease enter a valid 5-letter word\n");
            }
            Err(e) => return Err(e),
        }
    }
}

fn intro() {
    println!();
    print!("       ");
    print_text("cwordle".to_string(), [1, 0, 2, 2, 1, 0, 1].to_vec());
    println!("\n\nType your first guess and hit Enter.\n");
}

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

fn print_text(text: String, colors: Vec<i32>) {
    for (i, c) in text.chars().enumerate() {
        let paddedchar = format!(" {} ", c);
        if colors[i] == 1 {
            print!("{} ", paddedchar.green().reversed());
        } else if colors[i] == 2 {
            print!("{} ", paddedchar.yellow().reversed());
        } else {
            print!("{} ", paddedchar.reversed());
        }
    }
}
 */
fn main() -> io::Result<()> {
    let win = game::init_game();
    game::game_loop(&win);
    game::end_game(&win);

    Ok(())
}
