use std::io;

use colored::Colorize;

use crate::sanitize_word;

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

impl Iterator for WordleGame {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_next() {
            println!("{}", "Enter your word (5 letters) and guess:".bright_blue());
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            input = match sanitize_word(input.as_str()) {
                Ok(str) => str,
                Err(_) => {
                    println!("{}", "The length of input characters is incorrect.".red());
                    return Some(())
                }
            };
            self.next(input);
            Some(())
        } else {
            None
        }
    }
}

impl WordleGame {
    pub fn new(word: String) -> Self {
        Self { word, guesses: Vec::new() }
    }

    fn next(&mut self, word: String) {
        self.guesses.push(word);
        self.display_invaild_letter();
    }

    fn has_next(&self) -> bool {
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