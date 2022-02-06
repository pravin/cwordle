use owo_colors::OwoColorize;
use rand::{
    prelude::{SliceRandom, ThreadRng},
    Rng,
};
use std::{env, io};

static WORD_LIST: &'static str = include_str!("word-list.txt");

fn choose_word() -> &'static str {
    /*!
    Returns a random word from WORD_LIST. We use number magic here.
    As all words have 5 letters + possibly trailing \n, we can just pick a
    random number between (0, WORD_LIST.len()) and then divide by 6 to find
    the words starting position
    */
    let mut range: ThreadRng = rand::thread_rng();
    let num: usize = range.gen_range(5000..10000);
    let start: usize = (num / 6) * 6;

    &WORD_LIST[start..start + 5]
}

fn get_input() -> io::Result<String> {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input = input.to_lowercase();
                if input.len() == 6 {
                    input = input[0..5].to_string();
                    if WORD_LIST.contains(&input) {
                        return Ok(input);
                    }
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

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--help" {
        show_help();
    }
    intro();
    let choice = choose_word().to_owned();

    let mut winner = false;
    let mut count = 0;
    while !winner && count < 6 {
        let input = get_input().unwrap();

        print!("\x1b[2K\x1b[1A"); // Delete line, move up

        if input == choice {
            winner = true;
            print!("\x1b[2K\x1b[1A");
            println!();
        } else {
            let mut vchoice: Vec<char> = choice.chars().collect();
            let mut colors: Vec<i32> = [0, 0, 0, 0, 0].to_vec();

            // First, check letters in the correct position
            for (i, c) in input.chars().enumerate() {
                if c == vchoice[i] {
                    colors[i] = 1; // 1 = Right letter, right place
                    vchoice[i] = '.';
                }
            }
            // Next check for letters which are present in the word, but in the wrong position
            for (i, c) in input.chars().enumerate() {
                if colors[i] == 0 && vchoice.contains(&c) {
                    colors[i] = 2; // 2 = Right letter, wrong place
                    if let Some(idx) = vchoice.iter().position(|x| x == &c) {
                        vchoice[idx] = '.';
                    }
                }
            }

            print_text(input, colors);

            println!("\t{} tries left", 5 - count);
        }
        count += 1;
    }

    if winner {
        println!();
        print_text(choice, [1, 1, 1, 1, 1].to_vec());
        println!("\tWell done!");
    } else {
        println!();
        print_text(choice, [2, 2, 2, 2, 2].to_vec());
        println!("\tTry again!");
    }
    Ok(())
}
