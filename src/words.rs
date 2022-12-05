use rand::{prelude::ThreadRng, Rng};

static WORD_LIST: &'static str = include_str!("word-list.txt");

pub fn choose_word() -> &'static str {
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

pub fn word_isvalid(word: &str) -> bool {
    WORD_LIST.contains(word) // FIXME: Fragile. Searches across word boundaries
}
