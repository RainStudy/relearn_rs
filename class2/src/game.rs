use colored::Colorize;

// 单词长度
pub const WORD_LENGTH: usize = 5;
// 可尝试次数
pub const MAX_TRIES: usize = 6;

pub struct WordleGame {
    // 要猜的单词
    word: String,
    // 猜过的词
    guesses: Vec<String>,
}

impl WordleGame {
    pub fn new(word: String) -> Self {
        Self { word, guesses: Vec::new() }
    }

    pub fn next(&mut self, word: String) {
        self.guesses.push(word);
        self.display_invaild_letter();
    }

    pub fn has_next(&self) -> bool {
        self.guesses.len() < MAX_TRIES && !self.is_complete()
    }

    pub fn is_complete(&self) -> bool {
        if let Some(str) = self.guesses.last() {
            str == &self.word
        } else {
            false
        }
    }

    fn display_invaild_letter(&self) {
        let str = self.guesses.last().unwrap();
        let chars: Vec<char> = self.word.chars().collect();
        for (index, char) in str.chars().enumerate() {
            let display = if chars[index] == char {
                format!("{}", char).bright_green().bold()
            } else if chars.contains(&char) {
                format!("{}", char).yellow().bold()
            } else {
                format!("{}", char).red().bold()
            };
            print!("{}", display);
        }
        println!();
    }
}