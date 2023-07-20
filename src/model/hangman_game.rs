use crate::Language;

pub struct HangmanGame {
    word_list_german: Vec<&'static str>,
    word_list_english: Vec<&'static str>,
    difficulty: u8,
    language: Language,
    lives: u8,
    current_word: String,
    guessed_letters: Vec<char>,
}

impl HangmanGame {
    pub fn new(difficulty: u8, language: Language) -> Self {
        let word_list_german = vec![
            // Liste der deutschen Wörter hier einfügen
        ];

        let word_list_english = vec![
            // Liste der englischen Wörter hier einfügen
        ];

        HangmanGame {
            word_list_german,
            word_list_english,
            difficulty,
            language,
            lives: 3,
            current_word: String::new(),
            guessed_letters: Vec::new(),
        }
    }

    fn start_game(&mut self) {
        self.choose_word();
        self.play_game();
    }

    fn choose_word(&mut self) {
        let word_list = match self.language {
            Language::German => &self.word_list_german,
            Language::English => &self.word_list_english,
        };

        let filtered_words: Vec<&str> = word_list
            .iter()
            .filter(|&&word| word.len() >= self.difficulty as usize)
            .copied()
            .collect();

        let random_word = filtered_words.choose(&mut rand::thread_rng());

        self.current_word = match random_word {
            Some(&word) => word.to_string(),
            None => String::new(),
        };
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
