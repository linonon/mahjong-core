use rand::{seq::SliceRandom, thread_rng, SeedableRng};

use super::mahjong::{Mahjong, Suit};

pub struct Deck {
    pub cards: Vec<Mahjong>,
}

impl Deck {
    pub fn new() -> Self {
        Deck { cards: Vec::new() }
    }

    pub fn new_game() -> Self {
        let mut deck = Deck::new();
        deck.generate_cards();
        deck.shuffle();
        deck
    }

    pub fn new_fixed_game() -> Self {
        let mut deck = Deck::new();
        deck.generate_fixed_cards(100);
        deck.shuffle();
        deck
    }

    pub fn generate_cards(&mut self) {
        let mut cards = Vec::<Mahjong>::with_capacity(136);

        for suit in &[Suit::M, Suit::P, Suit::S] {
            cards.push(Mahjong::new(*suit, 0, None));
            for value in 1..=9 {
                if value == 5 {
                    for _ in 0..3 {
                        cards.push(Mahjong::new(*suit, value, None));
                    }
                } else {
                    for _ in 0..4 {
                        cards.push(Mahjong::new(*suit, value, None));
                    }
                }
            }
        }

        for value in 1..=7 {
            for _ in 0..4 {
                cards.push(Mahjong::new(Suit::Z, value, None));
            }
        }

        self.cards = cards
    }

    #[allow(dead_code)]
    pub fn generate_fixed_cards(&mut self, seed: u64) {
        self.generate_cards();
        let mut rng = rand_pcg::Pcg64::seed_from_u64(seed);
        self.cards.shuffle(&mut rng);
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn get_one_card(&mut self) -> Option<Mahjong> {
        if self.cards.is_empty() {
            None
        } else {
            Some(self.cards.remove(0))
        }
    }
}
