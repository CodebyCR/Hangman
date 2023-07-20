mod language;

use std::io;
use std::io::Read;
use crate::language::Language;
use crate::model::hangman_game::HangmanGame;

mod model {
    pub mod hangman_game;
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

fn get_main_menu() -> Language {
    let mut buffer = [0u8; 1];
    io::stdin().read_exact(&mut buffer).unwrap();
    let taste = buffer[0] as char;
    println!("Welcome to Hangwoman!{taste}");
    // chose language and save in config file
    println!("Please choose a difficulty:");
    println!("1 - Easy");
    println!("2 - Medium");
    println!("3 - Hard");
    let difficulty = get_difficulty();

    return Language::English{difficulty};
}

fn main() {

    loop {
        let language: Language = get_main_menu();

        let mut game: HangmanGame = HangmanGame::new( language);

        // TODO: implement Win/Lose
        game.start_game();
    }
}
