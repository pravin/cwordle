use pancurses::{
    cbreak, endwin, has_colors, init_pair, initscr, noecho, start_color, use_default_colors, Input,
    Window, COLOR_BLACK, COLOR_BLUE, COLOR_GREEN, COLOR_PAIR, COLOR_RED, COLOR_WHITE, COLOR_YELLOW,
    OK,
};

use super::gamestate::{GameData, GameStatus, KeyBoardHelperState};

pub fn init_game() -> Window {
    let win: Window = initscr();

    if win.get_max_y() < 20 || win.get_max_x() < 50 {
        panic!("You need at least a 50x20 terminal window to play this game.");
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
    init_pair(2, COLOR_BLACK, COLOR_WHITE);
    init_pair(3, COLOR_BLACK, COLOR_GREEN);
    init_pair(4, COLOR_BLACK, COLOR_YELLOW);
    init_pair(5, COLOR_WHITE, COLOR_BLUE);
    init_pair(6, COLOR_WHITE, COLOR_RED);

    cbreak();
    noecho();

    draw_header(&win);
    draw_footer(&win);

    // Load valid 5-letter words in memory

    win
}

fn draw_header(win: &Window) {
    const TITLE: &str = "cwordle";
    let title_len: i32 = TITLE.chars().count() as i32;
    let color_array = [3, 2, 4, 4, 3, 2, 3].to_vec();
    let mut x_pos = (win.get_max_x() - (title_len * 4)) / 2;

    for (i, c) in TITLE.chars().enumerate() {
        win.attrset(COLOR_PAIR(color_array[i]));
        win.mvaddstr(1, x_pos, format!(" {} ", c));
        x_pos = x_pos + 4;
    }
    win.attrset(COLOR_PAIR(1)); // reset colors
}

pub fn draw_keyboard(win: &Window, game_data: &GameData) {
    let keyboard = "qwertyuiopasdfghjklzxcvbnm";
    let mut x_pos: i32 = (win.get_max_x() - 36) / 2; // 9 letters per line
    let mut y_pos: i32 = win.get_max_y() - 6;

    let keyboard_helper = game_data.keyboard_helper();
    win.attrset(COLOR_PAIR(2)); // reset colors
    for j in 0..26 {
        let the_char = keyboard.as_bytes()[j] as char;
        let key_index = the_char as usize - 97;
        // Get color of char
        match keyboard_helper[key_index] {
            KeyBoardHelperState::GRAY => win.attrset(COLOR_PAIR(6)),
            KeyBoardHelperState::AMBER => win.attrset(COLOR_PAIR(4)),
            KeyBoardHelperState::GREEN => win.attrset(COLOR_PAIR(3)),
            KeyBoardHelperState::NONE => win.attrset(COLOR_PAIR(2)),
        };
        win.mvaddstr(y_pos, x_pos, format!(" {} ", the_char));
        x_pos += 4;
        if (j + 1) % 10 == 0 {
            y_pos += 2;
            x_pos = (win.get_max_x() - 36) / 2;
            if j == 19 {
                // center the last line
                x_pos += 8;
            }
        }
    }
    win.attrset(COLOR_PAIR(1)); // reset colors
}

fn draw_footer(win: &Window) {
    win.mvaddstr(win.get_max_y() - 1, 2, " Press Ctrl+C to quit ");
}

pub fn get_user_input(win: &Window, game_data: &GameData) -> String {
    // Move cursor to the right position
    let (x_pos, y_pos) = get_xy_pos(&win, &game_data);

    let mut count: i32 = 0;
    let mut input_array: [char; 5] = [' ', ' ', ' ', ' ', ' '];

    win.attrset(COLOR_PAIR(5));
    win.mv(y_pos, x_pos);
    loop {
        match win.getch() {
            Some(Input::Character(ch)) => {
                if ch == '\n' && count == 5 {
                    // Enter Key
                    break;
                }
                let keycode: u32 = ch.into();
                if keycode == 127 && count > 0 {
                    // Backspace Key
                    count -= 1;
                    win.attrset(COLOR_PAIR(1));
                    win.mvaddstr(y_pos, count * 4 + x_pos, "   ");
                    win.mv(y_pos, count * 4 + x_pos);
                    win.attrset(COLOR_PAIR(5));
                }
                if !ch.is_alphabetic() || count >= 5 {
                    continue;
                }
                input_array[count as usize] = ch;
                win.mvaddstr(y_pos, count * 4 + x_pos, format!(" {} ", ch));
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

pub fn clear_incorrect_word(win: &Window, game_data: &GameData) {
    let (x_pos, y_pos) = get_xy_pos(&win, &game_data);

    show_error(win);
    // Print blank line
    win.mvaddstr(y_pos, x_pos, "                    ");
    win.mv(y_pos, x_pos);
}

fn get_xy_pos(win: &Window, game_data: &GameData) -> (i32, i32) {
    let x_pos: i32 = (win.get_max_x() - 20) / 2; // 20 = 5 chars * 4 spaces for each char
    let y_pos: i32 = (win.get_max_y() / 2) - 8 + (game_data.guess_count() * 2) as i32; // 6 rows * 2 spaces for each row
    (x_pos, y_pos)
}

pub fn color_input_word(win: &Window, game_data: &GameData, input: &str) {
    let (x_pos, y_pos) = get_xy_pos(&win, &game_data);
    let choice = game_data.winning_word();
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

    for (i, c) in input.chars().enumerate() {
        if colors[i] == 1 {
            win.attrset(COLOR_PAIR(3));
            win.mvaddstr(y_pos, (i as i32) * 4 + x_pos, format!(" {} ", c));
            win.attrset(COLOR_PAIR(1));
        } else if colors[i] == 2 {
            win.attrset(COLOR_PAIR(4));
            win.mvaddstr(y_pos, (i as i32) * 4 + x_pos, format!(" {} ", c));
            win.attrset(COLOR_PAIR(1));
        } else {
            win.attrset(COLOR_PAIR(2));
            win.mvaddstr(y_pos, (i as i32) * 4 + x_pos, format!(" {} ", c));
            win.attrset(COLOR_PAIR(1));
        }
    }
}

pub fn end_game(win: &Window, game_data: GameData) {
    assert!(game_data.status() != GameStatus::PLAYING);
    if game_data.status() == GameStatus::WON {
        win.attrset(COLOR_PAIR(3));
        win.mvaddstr(win.get_max_y() - 1, 2, "Congratulations! You win :-)");
    } else {
        win.attrset(COLOR_PAIR(2));
        win.mvaddstr(
            win.get_max_y() - 1,
            2,
            format!(
                "The word was {}. Sorry! You lost :-(",
                game_data.winning_word(),
            ),
        );
    }
    win.getch();
    win.attrset(COLOR_PAIR(1));
    endwin();
}

fn show_error(win: &Window) {
    win.attrset(COLOR_PAIR(6));
    win.mvaddstr(win.get_max_y() - 1, 2, "Please input a valid word");
    win.getch();
    win.attrset(COLOR_PAIR(1));
    win.mvaddstr(win.get_max_y() - 1, 2, "                         ");
    draw_footer(win)
}
