mod model::HangmanGame::*;

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
            println!("Please enter a number!");
            get_difficulty()
        }
    };
    difficulty
}

fn main() {
    let mut buffer = [0u8; 1]; // Puffer mit einer Bytegröße erstellen

    // Auf die Eingabe einer Taste warten
    io::stdin().read_exact(&mut buffer).unwrap();

    // Puffer auswerten
    let taste = buffer[0] as char;
    println!("Welcome to Hangwoman!{taste}");

    println!("Please choose a difficulty:");
    println!("1 - Easy");
    println!("2 - Medium");
    println!("3 - Hard");

    let difficulty = get_difficulty();

    let mut game = HangmanGame::new(3, Language::English);
    game.start_game();
}
