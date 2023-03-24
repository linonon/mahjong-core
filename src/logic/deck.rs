use rand::{seq::SliceRandom, thread_rng, SeedableRng};

use super::mahjong::Mahjong;

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
        let mut deck = Vec::<Mahjong>::with_capacity(136);

        for suit in &[|v| Mahjong::M(v), |v| Mahjong::P(v), |v| Mahjong::S(v)] {
            deck.push(suit(0));
            for value in 0..=9 {
                let num_copies = match value {
                    0 | 5 => 3,
                    _ => 4,
                };
                for _ in 0..num_copies {
                    deck.push(suit(value));
                }
            }
        }

        for value in 1..=7 {
            for _ in 0..4 {
                deck.push(Mahjong::Z(value));
            }
        }

        self.cards = deck;
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
