use super::{
    deck::Deck,
    mahjong::{Mahjong, Suit},
};

pub struct Player {
    pub id: u64,                       // 玩家 ID
    pub hand: Vec<Mahjong>,            // 手牌
    pub drawn_card: Option<Mahjong>,   // 摸到的牌
    pub melds: Vec<Meld>,              // 副露的牌
    pub discarded_cards: Vec<Mahjong>, // 打出的牌
    pub seat_position: SeatPosition,   // 座位位置
}

// 添加一个枚举类型表示吃碰杠的组合
pub enum Meld {
    Chi(Vec<Mahjong>),
    Pon(Vec<Mahjong>),
    Kan(Vec<Mahjong>),
}

#[derive(PartialEq)]
pub enum Action {
    Chi(Vec<Mahjong>),
    Pon(Mahjong),
    Minkan(Mahjong),
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum SeatPosition {
    East,
    South,
    West,
    North,
}

impl Player {
    pub fn new(id: u64, sit_pos: SeatPosition) -> Self {
        Player {
            id,
            hand: Vec::with_capacity(13),
            drawn_card: None,
            melds: Vec::new(),
            discarded_cards: Vec::new(),
            seat_position: sit_pos,
        }
    }

    // 从牌堆中摸一张牌到 drawn_card 中
    pub fn draw_card(&mut self, deck: &mut Deck) {
        if let Some(drawn) = deck.get_one_card() {
            self.drawn_card = Some(drawn);
        } else {
            println!("No more cards left in the deck!");
        };
    }

    // 从牌堆中获得一张牌到 hand 中
    pub fn add_hand(&mut self, deck: &mut Deck) {
        if let Some(drawn) = deck.get_one_card() {
            self.hand.push(drawn);
            self.sort_hand();
        } else {
            panic!("No more cards left in the deck!");
        };
    }

    pub fn discard(&mut self, index: usize) -> Option<Mahjong> {
        if index == 0 {
            self.drawn_card.take()
        } else {
            let discard = self.hand.remove(index - 1);
            if let Some(drawn) = self.drawn_card {
                self.hand.push(drawn);
                self.sort_hand();
            } else {
                panic!("No card in drawn_card!")
            }

            Some(discard)
        }
    }

    pub fn push_in_discarded(&mut self, card: Mahjong) {
        self.discarded_cards.push(card);
    }

    // 对手牌进行排序
    pub fn sort_hand(&mut self) {
        self.hand.sort_by(|a, b| a.cmp(b));
    }

    pub fn print_hand(&self) -> String {
        let mut hand = String::new();
        let mut iter = self.hand.iter().peekable();

        while let Some(this_card) = iter.next() {
            hand.push_str(&format!("{}", this_card.value));

            if let Some(next_card) = iter.peek() {
                if next_card.suit != this_card.suit {
                    hand.push_str(&format!("{} ", this_card.suit));
                }
            } else {
                hand.push_str(&format!("{}", this_card.suit));
            }
        }
        hand
    }

    // 添加其他方法以支持玩家的游戏功能，例如摸牌、打牌、吃碰杠等
}

impl Player {
    /// 检查当前玩家是否可以吃上家的牌, 返回所有可能的吃牌组合
    pub fn can_chi(&self, card: &Mahjong) -> Vec<Vec<Mahjong>> {
        // 检查当前玩家是否可以吃上家的牌
        match (&self.seat_position, card.belongs_to) {
            (SeatPosition::East, Some(SeatPosition::North)) => (),
            (SeatPosition::South, Some(SeatPosition::East)) => (),
            (SeatPosition::West, Some(SeatPosition::South)) => (),
            (SeatPosition::North, Some(SeatPosition::West)) => (),
            _ => return Vec::new(),
        }

        if card.suit == Suit::Z {
            return Vec::new(); // 字牌不能吃
        }

        let mut possible_combinations = Vec::new();

        for i in -1..=1 {
            let sequence = vec![
                Mahjong {
                    suit: card.suit,
                    value: card.value + i - 1,
                    belongs_to: card.belongs_to,
                },
                Mahjong {
                    suit: card.suit,
                    value: card.value + i,
                    belongs_to: card.belongs_to,
                },
                Mahjong {
                    suit: card.suit,
                    value: card.value + i + 1,
                    belongs_to: card.belongs_to,
                },
            ];

            if sequence
                .iter()
                .all(|mahjong| mahjong.value >= 1 && mahjong.value <= 9)
                && sequence[0..2]
                    .iter()
                    .all(|mahjong| self.hand.contains(mahjong))
            {
                possible_combinations.push(sequence);
            }
        }

        possible_combinations
    }

    /// 碰牌    
    pub fn pon(&mut self, card: Mahjong) -> bool {
        // 确保玩家不能碰自己打出的牌
        if let Some(belongs) = card.belongs_to {
            if self.seat_position == belongs {
                return false;
            }
        }

        // 检查玩家手中是否有两张与打出的牌相同的牌
        let matching_cards: Vec<Mahjong> = self
            .hand
            .iter()
            .filter(|mahjong| mahjong.is_same_mahjong(&card))
            .cloned()
            .collect();
        if matching_cards.len() < 2 {
            return false;
        }

        // 移除手牌中的两张匹配牌，并将碰牌组合添加到碰杠组中

        self.melds.push(Meld::Pon(vec![card, card, card]));

        true
    }

    pub fn get_available_actions(
        &self,
        card: &Mahjong,
        discard_seat_position: &SeatPosition,
    ) -> Vec<Action> {
        let mut available_actions = Vec::new();

        // 检查是否可以吃牌
        let chi_combinations = self.can_chi(card);
        if !chi_combinations.is_empty() {
            for combination in chi_combinations {
                available_actions.push(Action::Chi(combination));
            }
        }

        // 检查相同牌的数量
        let matching_cards: Vec<Mahjong> = self
            .hand
            .iter()
            .filter(|&mahjong| mahjong == card)
            .cloned()
            .collect();

        // 检查是否可以碰牌
        if matching_cards.len() == 2 && &self.seat_position != discard_seat_position {
            available_actions.push(Action::Pon(card.clone()));
        }

        // 检查是否可以明杠牌
        if matching_cards.len() == 3 {
            available_actions.push(Action::Minkan(card.clone()));
        }

        // TODO: 检查是否可以立直
        // 判断玩家的手牌是否符合立直的条件，然后将立直动作添加到 available_actions 中。

        // TODO: 检查是否可以和牌（荣和）
        // 判断玩家的手牌是否符合荣和的条件，然后将和牌动作添加到 available_actions 中。

        // TODO: 检查是否可以自摸和牌（Tsumo）
        // 判断玩家摸到一张牌后的手牌是否符合自摸和牌的条件，然后将和牌动作添加到 available_actions 中。

        available_actions
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut hand = String::new();
        for card in &self.hand {
            hand.push_str(&format!("{} ", card));
        }
        write!(f, "{}", hand)
    }
}
