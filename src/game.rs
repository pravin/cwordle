use pancurses::{
    cbreak, endwin, has_colors, init_pair, initscr, noecho, start_color, use_default_colors, Input,
    Window, COLOR_BLACK, COLOR_BLUE, COLOR_GREEN, COLOR_PAIR, COLOR_WHITE, COLOR_YELLOW,
};

pub fn init_game() -> Window {
    let win: Window = initscr();

    if win.get_max_y() < 20 || win.get_max_x() < 50 {
        panic!("You need at least a 30x20 terminal window to play this game.");
    }

    if !has_colors() {
        panic!("You need a terminal that can do color.");
    }

    start_color();
    use_default_colors();
    init_pair(1, COLOR_WHITE, COLOR_BLACK);
    init_pair(2, COLOR_BLACK, COLOR_WHITE);
    init_pair(3, COLOR_BLACK, COLOR_GREEN);
    init_pair(4, COLOR_BLACK, COLOR_YELLOW);
    init_pair(5, COLOR_WHITE, COLOR_BLUE);

    cbreak();
    noecho();

    draw_header(&win);
    draw_keyboard(&win);
    draw_footer(&win);
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

fn draw_keyboard(win: &Window) {
    let keyboard = "qwertyuiopasdfghjklzxcvbnm";
    let mut x_pos: i32 = (win.get_max_x() - 36) / 2; // 9 letters per line
    let mut y_pos: i32 = win.get_max_y() - 6;

    win.attrset(COLOR_PAIR(2)); // reset colors
    for j in 0..26 {
        win.mvaddstr(
            y_pos,
            x_pos,
            format!(" {} ", keyboard.as_bytes()[j] as char),
        );
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
    win.mvaddstr(win.get_max_y() - 1, 2, " Press ? for help ");
}

pub fn game_loop(win: &Window) {
    let mut winner = false;
    let mut count = 0;
    while !winner && count < 6 {
        let word = get_valid_word(win, count);
        count += 1;
    }
}

fn get_valid_word(win: &Window, row: i32) -> String {
    let mut count: i32 = 0;
    let x_pos: i32 = (win.get_max_x() - 20) / 2; // 20 = 5 chars * 4 spaces for each char
    let y_pos: i32 = (win.get_max_y() / 2) - 8 + row * 2; // 6 rows * 2 spaces for each row
    let mut input_array: [char; 5] = [' ', ' ', ' ', ' ', ' '];

    win.attrset(COLOR_PAIR(5));
    loop {
        match win.getch() {
            Some(Input::Character(ch)) => {
                if ch == '\n' && count == 5 {
                    // Enter Key
                    break;
                }
                if !ch.is_alphabetic() || count >= 5 {
                    continue;
                }
                input_array[count as usize] = ch;
                win.mvaddstr(y_pos, count * 4 + x_pos, format!(" {} ", ch));
            }
            Some(Input::KeyEnter) => {
                win.printw("HELLO");
            }
            Some(_input) => continue,
            None => continue,
        }
        count += 1;
    }
    win.attrset(COLOR_PAIR(1)); // Reset colors

    // Check if word is valid
    let input_word: String = input_array.iter().collect();
    input_word
}

pub fn end_game(win: &Window) {
    endwin();
}
