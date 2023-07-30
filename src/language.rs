use rand::seq::SliceRandom;

#[derive(Copy, Clone)]
pub enum Language {
    German { difficulty: u8 },
    English { difficulty: u8 },
}

impl Language {
    fn get_german_word_list() -> Vec<&'static str> {
        vec![
            "Haus", "Dach", "Aal", "Hochdruckreiniger", "Krankenversicherung", "Pommesschranke",
        ]
    }

    fn get_english_word_list() -> Vec<&'static str> {
        vec![
            "House", "Roof", "Institution", "Homework", "Pencil", "Computer", "MegaAwesomeWord",
        ]
    }

    pub fn get_word_list(&self) -> Vec<&'static str> {
        match self {
            Language::German { difficulty } => Self::get_german_word_list(),
            Language::English { difficulty } => Self::get_english_word_list(),
        }
    }

    pub fn get_random_word(&self) -> String {
        let word_list = self.get_word_list();

        let difficulty = match self {
            Language::German { difficulty } => *difficulty,
            Language::English { difficulty } => *difficulty,
        };

        let filtered_words: Vec<&str> = word_list
            .iter()
            .filter(|&&word| word.len() >= difficulty as usize)
            .copied()
            .collect();

        if filtered_words.is_empty() {
            return String::new();
        }

        let random_word = filtered_words.choose(&mut rand::thread_rng());

        match random_word {
            Some(&word) => word.to_string(),
            None => String::new(),
        }
    }
}
