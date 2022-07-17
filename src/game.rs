use pancurses::{
    cbreak, endwin, has_colors, init_pair, initscr, noecho, start_color, use_default_colors, Input,
    Window, COLOR_BLACK, COLOR_GREEN, COLOR_PAIR, COLOR_WHITE, COLOR_YELLOW,
};

pub fn initgame() {
    let win: Window = initscr();

    if win.get_max_y() < 20 || win.get_max_x() < 30 {
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

    cbreak();
    noecho();

    draw_header(&win, win.get_max_x());
    draw_keyboard(&win, win.get_max_x(), win.get_max_y());

    win.getch();
    endwin();
}

fn draw_header(win: &Window, width: i32) {
    const TITLE: &str = "cwordle";
    let title_len: i32 = TITLE.chars().count() as i32;
    let color_array = [3, 2, 4, 4, 3, 2, 3].to_vec();
    let mut x_pos = (width - (title_len * 4)) / 2;

    for (i, c) in TITLE.chars().enumerate() {
        win.attrset(COLOR_PAIR(color_array[i]));
        win.mvaddstr(1, x_pos, format!(" {} ", c));
        x_pos = x_pos + 4;
    }
    win.attrset(COLOR_PAIR(1)); // reset colors
}

fn draw_keyboard(win: &Window, width: i32, height: i32) {
    let keyboard = "qwertyuiopasdfghjklzxcvbnm";
    let mut x_pos: i32 = (width - 18) / 2; // 9 letters per line
    let mut y_pos: i32 = height - 3;
    for j in 0..26 {
        win.mvaddch(y_pos, x_pos, keyboard.as_bytes()[j] as char);
        x_pos += 2;
        if (j + 1) % 10 == 0 {
            y_pos += 1;
            x_pos = (width - 18) / 2;
            if j == 19 {
                // center the last line
                x_pos += 4;
            }
        }
    }
}
