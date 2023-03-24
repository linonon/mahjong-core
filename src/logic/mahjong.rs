use rand::{seq::SliceRandom, thread_rng, SeedableRng};

use super::player::SeatPosition;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Suit {
    M, // 万子
    P, // 饼子
    S, // 索子
    Z, // 字牌（东、南、西、北、中、发、白）
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suit = match self {
            Suit::M => "m",
            Suit::P => "p",
            Suit::S => "s",
            Suit::Z => "z",
        };

        write!(f, "{}", suit)
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Mahjong {
    pub suit: Suit,
    pub value: i8,
    pub belongs_to: Option<SeatPosition>,
}

impl Mahjong {
    pub fn new(suit: Suit, value: i8, belongs_to: Option<SeatPosition>) -> Self {
        assert!(
            (0..=9).contains(&value) || (suit == Suit::Z && (1..=7).contains(&value)),
            "Invalid value for the given suit"
        );

        Mahjong { suit, value, belongs_to }
    }

    pub fn cmp(&self, b: &Mahjong) -> std::cmp::Ordering {
        let a = self;

        let suit_order = (a.suit as u8).cmp(&(b.suit as u8));
        if suit_order != std::cmp::Ordering::Equal {
            suit_order
        } else {
            let mut a = a.value;
            let mut b = b.value;
            if a == 0 {
                a = 5;
            }
            if b == 0 {
                b = 5;
            }

            a.cmp(&b)
        }
    }

    pub fn is_same_mahjong(&self, b: &Mahjong) -> bool {
        self.suit == b.suit && self.value == b.value
    }
}

impl std::fmt::Display for Mahjong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self.value {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            _ => unreachable!(),
        };

        write!(f, "{}{}", value, self.suit)
    }
}
