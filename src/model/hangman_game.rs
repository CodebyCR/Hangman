
use crate::language::Language;

pub struct HangmanGame {
    pub(crate)
    language: Language,
    lives: u8,
    current_word: String,
    guessed_letters: Vec<char>,
}

impl HangmanGame {
    pub fn new(language: Language) -> Self {
        HangmanGame {
            language,
            lives: 3,
            current_word: language.get_random_word(),
            guessed_letters: Vec::new(),
        }
    }

    pub(crate) fn start_game(&mut self) {
        self.play_game();
    }


    fn play_game(&mut self) {
        let mut correct_guesses = Vec::new();
        let mut wrong_guesses = Vec::new();

        while self.lives > 0 {
            self.display_word();

            if !self.current_word.contains('_') {
                println!("Congratulations! You guessed the word!");
                return;
            }

            println!("Lives: {}", self.lives);

            let guessed_letter = self.get_guess();

            if self.current_word.contains(guessed_letter) {
                correct_guesses.push(guessed_letter);
            } else {
                wrong_guesses.push(guessed_letter);
                self.lives -= 1;
                println!("Wrong guess! You lost a life.");
            }
        }

        println!(
            "Game over! The word was: {}",
            self.current_word
        );
    }

    fn display_word(&self) {
        for c in self.current_word.chars() {
            print!("{c} ");
        }
        println!();
    }

    fn get_guess(&mut self) -> char {
        loop {
            println!("Enter your guess (a single letter):");
            let mut guess = String::new();
            std::io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line.");

            match guess.trim().chars().next() {
                Some(c) if c.is_ascii_alphabetic() => {
                    if self.guessed_letters.contains(&c) {
                        println!("You already guessed that letter. Try again.");
                    } else {
                        self.guessed_letters.push(c);
                        return c;
                    }
                }
                _ => println!("Invalid input. Try again."),
            }
        }
    }
}
