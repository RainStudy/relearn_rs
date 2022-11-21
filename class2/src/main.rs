use std::io;

use clap::{command, arg};
use colored::Colorize;
use game::WORD_LENGTH;
use error::{Resultish, AppError};

mod game;
mod error;

fn main() -> Resultish<()> {
    let matches = command!()
        .arg(arg!(--word <VALUE> "the target word to guess"))
        .get_matches();
    let word = if let Some(str) = matches.get_one::<String>("word") {
        str
    } else {
        "APPLE"
    };
    let word = sanitize_word(word)?;
    let mut game = game::WordleGame::new(word);
    let win = loop {
        if game.has_next() {
            println!("{}", "Enter your word (5 letters) and guess:".bright_blue());
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input = match sanitize_word(input.as_str()) {
                Ok(str) => str,
                Err(_) => {
                    println!("{}", "The input word is too long!".red());
                    continue;
                }
            };
            game.next(input);
        } else {
            break game.is_complete();
        }
    };
    if win {
        println!("{}", "Congraulations! You win this game!".bright_cyan())
    } else {
        println!("{}", "I'm sorry that you aren't win this game.".red())
    }
    Ok(())
}

// 用户输入预检
// 1. 去除空格
// 2. 全大写
// 3. 检查是否是ASCII标准定义的字符，如果不是它们将会被忽略
fn sanitize_word(word: &str) -> Resultish<String> {
    let str: String = word.trim()
        .to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();
    if str.len() != WORD_LENGTH {
        Err(AppError::new(format!("输入单词长度不正确, 应为: {}", WORD_LENGTH)))
    } else {
        Ok(str)
    }
}