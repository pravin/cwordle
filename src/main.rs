use owo_colors::OwoColorize;
use rand::{prelude::ThreadRng, Rng};
use std::{cmp, io};

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

fn get_input() -> String {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        if input.len() == 6 && WORD_LIST.contains(&input) {
            return input[0..5].to_lowercase();
        }
        print!("\x1b[1A\tPlease enter a valid 5-letter word\n");
    }
}
fn main() -> io::Result<()> {
    let choice = choose_word().to_owned();
    //let choice = "elder";
    //println!("{}", choice);
    let mut winner = false;
    let mut count = 0;
    while !winner && count < 6 {
        let input = get_input();

        print!("\x1b[2K\x1b[1A");

        if input == choice {
            winner = true;
        } else {
            let end = cmp::min(5, input.len());
            let mut vchoice: Vec<char> = choice.chars().collect();
            let mut colors: [i32; 5] = [0, 0, 0, 0, 0];

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

            for i in 0..end {
                let outchar = input.chars().nth(i).unwrap();
                if colors[i] == 1 {
                    print!("{}", outchar.to_string().green().reversed());
                } else if colors[i] == 2 {
                    print!("{}", outchar.to_string().yellow().reversed());
                } else {
                    print!("{}", outchar.to_string().reversed());
                }
            }
            println!("\t{} turns left", 5 - count);
        }
        count += 1;
    }

    if winner {
        println!("{}\tWell done!", choice.green().reversed());
    } else {
        println!("{}\tTry again!", choice.bright_red().reversed());
    }
    Ok(())
}
