
use crate::language::Language;

pub struct HangmanGame {
    pub(crate)
    language: Language,
    lives: u8,
    current_word: String,
    guessed_letters: Vec<char>,
    correct_guesses: Vec<char>,
    wrong_guesses: Vec<char>,
}

impl HangmanGame {
    pub fn new(language: Language) -> Self {
        HangmanGame {
            language,
            lives: 10,
            current_word: language.get_random_word(),
            guessed_letters: Vec::new(),
            correct_guesses: Vec::new(),
            wrong_guesses: Vec::new(),
        }
    }

    pub(crate) fn start_game(&mut self) {
        self.play_game();
    }


    fn play_game(&mut self) {


        while self.lives > 0 {
            self.display_word();
            self.print_guessed_letters();

            if self.word_is_guessed() {
                println!("Congratulations! You guessed the word!");
                return;
            }

            println!("Lives: {}", self.lives);

            let guessed_letter = self.get_guess();

            if self.contains_to_ignore_case(guessed_letter) {
                self.correct_guesses.push(guessed_letter);
            }
            else {
                self.wrong_guesses.push(guessed_letter);
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
            let small_c = c.to_ascii_lowercase();
            if self.contains_to_ignore_case(small_c) {
                print!("{c} ");
            } else {
                print!("_ ");
            }
        }
        println!();
    }

    fn print_guessed_letters(&self) {
        print!("Guessed letters: ");
        // print guessed letters in alphabetical order
        let mut guessed_letters = self.guessed_letters.clone();
        guessed_letters.sort();
        for c in guessed_letters {
            print!("{c} ");
        }
        println!();
    }

    fn word_is_guessed(&self) -> bool {
        for c in self.current_word.chars() {
            if !self.guessed_letters.contains(&c) {
                return false;
            }
        }
        true
    }

    fn contains_to_ignore_case(&self, c: char) -> bool {
        let lowercase_word = self.current_word.to_lowercase();
        let lowercase_char = c.to_ascii_lowercase();

        lowercase_word.contains(lowercase_char) && self.guessed_letters.contains(&c)
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
