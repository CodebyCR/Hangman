use std::io;
use std::io::Read;
use crate::model::hangman_game;

mod model {
    pub mod hangman_game;
}


enum Language {
    German,
    English,
}

fn get_difficulty() -> u8 {
    let mut difficulty = String::new();
    io::stdin()
        .read_line(&mut difficulty)
        .expect("Failed to read line");
    let difficulty: u8 = match difficulty.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter the difficult level!");
            get_difficulty()
        }
    };
    difficulty
}

fn main() {
    let mut buffer = [0u8; 1];

    io::stdin().read_exact(&mut buffer).unwrap();

    let taste = buffer[0] as char;
    println!("Welcome to Hangwoman!{taste}");

    println!("Please choose a difficulty:");
    println!("1 - Easy");
    println!("2 - Medium");
    println!("3 - Hard");

    let difficulty = get_difficulty();

    let mut game = hangman_game::new(difficulty, Language::English);
    game.start_game();
}
