use pancurses::{
    cbreak, endwin, has_colors, init_pair, initscr, noecho, start_color, use_default_colors, Input,
    Window, COLOR_BLACK, COLOR_BLUE, COLOR_GREEN, COLOR_PAIR, COLOR_RED, COLOR_WHITE, COLOR_YELLOW,
    OK,
};

use super::gamestate::{GameData, GameStatus, KeyBoardHelperState};
use super::config::{Config, MIN_TERMINAL_WIDTH, MIN_TERMINAL_HEIGHT, CHAR_SPACING, LINE_SPACING, ALPHABET_SIZE, ASCII_LOWERCASE_A, WORD_LENGTH};
use super::stats::StatsManager;

pub fn init_game() -> Window {
    let win: Window = initscr();

    if win.get_max_y() < MIN_TERMINAL_HEIGHT || win.get_max_x() < MIN_TERMINAL_WIDTH {
        panic!("You need at least a {}x{} terminal window to play this game.", MIN_TERMINAL_WIDTH, MIN_TERMINAL_HEIGHT);
    }

    if !has_colors() {
        panic!("You need a terminal that can do color.");
    }

    start_color();
    if use_default_colors() == OK {
        init_pair(1, COLOR_WHITE, -1);
    } else {
        init_pair(1, COLOR_WHITE, COLOR_BLACK);
    }
    init_pair(2, COLOR_BLACK, COLOR_WHITE);  // Normal text
    init_pair(3, COLOR_BLACK, COLOR_GREEN);  // Correct position (green)
    init_pair(4, COLOR_BLACK, COLOR_YELLOW); // Wrong position (yellow)
    init_pair(5, COLOR_WHITE, COLOR_BLUE);   // Input cursor
    init_pair(6, COLOR_WHITE, COLOR_RED);    // Error messages
    init_pair(7, COLOR_WHITE, COLOR_BLACK);  // Incorrect letters (dark)

    cbreak();
    noecho();

    draw_header(&win);

    // Load valid 5-letter words in memory

    win
}

fn draw_header(win: &Window) {
    const TITLE: &str = "cwordle";
    let title_len: i32 = TITLE.chars().count() as i32;
    let color_array = [3, 2, 4, 4, 3, 2, 3].to_vec();
    let mut x_pos = (win.get_max_x() - (title_len * CHAR_SPACING)) / 2;

    for (i, c) in TITLE.chars().enumerate() {
        win.attrset(COLOR_PAIR(color_array[i]));
        win.mvaddstr(1, x_pos, format!(" {} ", c));
        x_pos = x_pos + CHAR_SPACING;
    }
    win.attrset(COLOR_PAIR(1)); // reset colors
}

pub fn draw_keyboard(win: &Window, game_data: &GameData) {
    let keyboard = "qwertyuiopasdfghjklzxcvbnm";
    let mut x_pos: i32 = (win.get_max_x() - 36) / 2; // 9 letters per line
    let mut y_pos: i32 = win.get_max_y() - 6;

    let keyboard_helper = game_data.keyboard_helper();
    win.attrset(COLOR_PAIR(2)); // reset colors
    for j in 0..ALPHABET_SIZE {
        let the_char = keyboard.as_bytes()[j] as char;
        // Safety check for valid lowercase ASCII
        if !the_char.is_ascii_lowercase() {
            continue;
        }
        let key_index = the_char as usize - ASCII_LOWERCASE_A;
        // Get color of char
        match keyboard_helper[key_index] {
            KeyBoardHelperState::GRAY => win.attrset(COLOR_PAIR(7)), // Dark background for incorrect
            KeyBoardHelperState::AMBER => win.attrset(COLOR_PAIR(4)), // Yellow background for wrong position
            KeyBoardHelperState::GREEN => win.attrset(COLOR_PAIR(3)), // Green background for correct
            KeyBoardHelperState::NONE => win.attrset(COLOR_PAIR(2)), // White background for unused
        };
        win.mvaddstr(y_pos, x_pos, format!(" {} ", the_char));
        x_pos += CHAR_SPACING;
        if (j + 1) % 10 == 0 {
            y_pos += LINE_SPACING;
            x_pos = (win.get_max_x() - 36) / 2;
            if j == 19 {
                // center the last line
                x_pos += 8;
            }
        }
    }
    win.attrset(COLOR_PAIR(1)); // reset colors
}

fn draw_footer(win: &Window, game_data: &GameData) {
    let mut footer_text = format!(" Guess {}/{} ", game_data.guess_count() + 1, game_data.max_guesses());

    if game_data.is_hard_mode() {
        footer_text.push_str("| HARD MODE ");
    }

    if !game_data.revealed_letters().is_empty() {
        footer_text.push_str(&format!("| Must use: {} ",
            game_data.revealed_letters().iter().collect::<String>()));
    }

    footer_text.push_str("| Press Ctrl+C to quit");

    win.mvaddstr(win.get_max_y() - 1, 2, &footer_text);
}

pub fn get_user_input(win: &Window, game_data: &GameData) -> String {
    // Move cursor to the right position
    let (x_pos, y_pos) = get_xy_pos(&win, &game_data);

    let mut count: i32 = 0;
    let mut input_array: [char; WORD_LENGTH] = [' '; WORD_LENGTH];

    win.attrset(COLOR_PAIR(5));
    win.mv(y_pos, x_pos);
    loop {
        match win.getch() {
            Some(Input::Character(ch)) => {
                if ch == '\n' && count == WORD_LENGTH as i32 {
                    // Enter Key
                    break;
                }
                let keycode: u32 = ch.into();
                if keycode == 127 && count > 0 {
                    // Backspace Key
                    count -= 1;
                    input_array[count as usize] = ' '; // Clear the character from array
                    // Keep the white background box but clear the letter
                    win.attrset(COLOR_PAIR(2)); // White background with black text
                    win.mvaddstr(y_pos, count * CHAR_SPACING + x_pos, "   ");
                    win.mv(y_pos, count * CHAR_SPACING + x_pos);
                    win.attrset(COLOR_PAIR(5)); // Reset to input cursor color
                }
                if !ch.is_alphabetic() || count >= WORD_LENGTH as i32 {
                    continue;
                }
                input_array[count as usize] = ch;
                win.mvaddstr(y_pos, count * CHAR_SPACING + x_pos, format!(" {} ", ch));
            }
            Some(_input) => continue,
            None => continue,
        }
        count += 1;
    }
    win.attrset(COLOR_PAIR(1)); // Reset colors

    let input_word: String = input_array.iter().collect();

    input_word.to_ascii_lowercase()
}

pub fn clear_incorrect_word(win: &Window, game_data: &GameData, error_msg: &str) {
    show_error(win, error_msg);
    // Redraw empty boxes for current input
    draw_empty_boxes(win, game_data);
}

fn get_xy_pos(win: &Window, game_data: &GameData) -> (i32, i32) {
    let x_pos: i32 = (win.get_max_x() - (WORD_LENGTH as i32 * CHAR_SPACING)) / 2;
    let y_pos: i32 = (win.get_max_y() / 2) - 8 + (game_data.guess_count() as i32 * LINE_SPACING);
    (x_pos, y_pos)
}

pub fn color_input_word(win: &Window, game_data: &GameData, input: &str) {
    // Use the previous guess count position since new_guess was already called
    let x_pos: i32 = (win.get_max_x() - (WORD_LENGTH as i32 * CHAR_SPACING)) / 2;
    let y_pos: i32 = (win.get_max_y() / 2) - 8 + ((game_data.guess_count() - 1) as i32 * LINE_SPACING);
    let choice = game_data.winning_word();
    let mut vchoice: Vec<char> = choice.chars().collect();
    let mut colors: Vec<i32> = vec![0; WORD_LENGTH];

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

    for (i, c) in input.chars().enumerate() {
        if colors[i] == 1 {
            win.attrset(COLOR_PAIR(3));
            win.mvaddstr(y_pos, (i as i32) * CHAR_SPACING + x_pos, format!(" {} ", c));
            win.attrset(COLOR_PAIR(1));
        } else if colors[i] == 2 {
            win.attrset(COLOR_PAIR(4));
            win.mvaddstr(y_pos, (i as i32) * CHAR_SPACING + x_pos, format!(" {} ", c));
            win.attrset(COLOR_PAIR(1));
        } else {
            win.attrset(COLOR_PAIR(7)); // Use dark background for incorrect letters
            win.mvaddstr(y_pos, (i as i32) * CHAR_SPACING + x_pos, format!(" {} ", c));
            win.attrset(COLOR_PAIR(1));
        }
    }
}

pub fn end_game(win: &Window, game_data: GameData, stats_manager: &mut StatsManager) -> bool {
    assert!(game_data.status() != GameStatus::PLAYING);

    let won = game_data.status() == GameStatus::WON;
    let guess_count = if won { Some(game_data.guess_count()) } else { None };

    // Record game statistics
    if let Err(e) = stats_manager.record_game(won, guess_count) {
        eprintln!("Failed to save statistics: {}", e);
    }

    if won {
        win.attrset(COLOR_PAIR(3));
        let message = if game_data.guess_count() == 1 {
            "Incredible! You got it in 1 guess!"
        } else {
            &format!("Congratulations! You won in {} guesses!", game_data.guess_count())
        };
        win.mvaddstr(win.get_max_y() - 1, 2, message);
    } else {
        win.attrset(COLOR_PAIR(2));
        win.mvaddstr(
            win.get_max_y() - 1,
            2,
            &format!(
                "The word was '{}'. Better luck next time!",
                game_data.winning_word(),
            ),
        );
    }

    win.mvaddstr(win.get_max_y() - 2, 2, "Press 'r' to play again, 's' for stats, or any other key to quit");

    let choice = win.getch();
    win.attrset(COLOR_PAIR(1));

    match choice {
        Some(Input::Character('r')) | Some(Input::Character('R')) => {
            true // Play again
        }
        Some(Input::Character('s')) | Some(Input::Character('S')) => {
            show_stats(win, stats_manager);
            false // Quit after showing stats
        }
        _ => {
            endwin();
            false // Quit
        }
    }
}

fn show_error(win: &Window, error_msg: &str) {
    win.attrset(COLOR_PAIR(6));
    win.mvaddstr(win.get_max_y() - 1, 2, error_msg);
    win.getch();
    win.attrset(COLOR_PAIR(1));
    // Clear the error message
    let spaces = " ".repeat(error_msg.len() + 10);
    win.mvaddstr(win.get_max_y() - 1, 2, &spaces);
}

pub fn show_stats(win: &Window, stats_manager: &StatsManager) {
    win.clear();
    let stats = stats_manager.get_stats();

    win.attrset(COLOR_PAIR(3));
    win.mvaddstr(2, 2, "GAME STATISTICS");
    win.attrset(COLOR_PAIR(1));

    win.mvaddstr(4, 2, &format!("Games Played: {}", stats.games_played));
    win.mvaddstr(5, 2, &format!("Win Rate: {:.1}%", stats.win_percentage()));
    win.mvaddstr(6, 2, &format!("Current Streak: {}", stats.current_streak));
    win.mvaddstr(7, 2, &format!("Max Streak: {}", stats.max_streak));
    win.mvaddstr(8, 2, &format!("Average Guesses: {:.1}", stats.average_guesses()));

    win.mvaddstr(10, 2, "Guess Distribution:");
    for guess in 1..=6 {
        let count = stats.guess_distribution.get(&(guess as u32)).unwrap_or(&0);
        let percentage = if stats.games_won > 0 {
            (*count as f32 / stats.games_won as f32) * 100.0
        } else {
            0.0
        };
        win.mvaddstr(10 + guess, 4, &format!("{}: {} ({:.1}%)", guess, count, percentage));
    }

    win.mvaddstr(win.get_max_y() - 2, 2, "Press any key to continue...");
    win.getch();
}

pub fn update_footer(win: &Window, game_data: &GameData) {
    // Clear the footer area
    let spaces = " ".repeat(win.get_max_x() as usize - 4);
    win.mvaddstr(win.get_max_y() - 1, 2, &spaces);
    draw_footer(win, game_data);
}

pub fn draw_empty_boxes(win: &Window, game_data: &GameData) {
    let (x_pos, y_pos) = get_xy_pos(&win, &game_data);

    win.attrset(COLOR_PAIR(2)); // White background with black text
    for i in 0..WORD_LENGTH {
        win.mvaddstr(y_pos, (i as i32) * CHAR_SPACING + x_pos, "   ");
    }
    win.attrset(COLOR_PAIR(1));
}

pub fn redraw_game_screen(win: &Window) {
    // Redraw the header for a fresh game
    draw_header(win);
    win.refresh();
}
