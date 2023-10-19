use super::{deck::Deck, mahjong::Mahjong};
use anyhow::{Ok, Result};

pub struct Player {
    pub id: u64,                       // 玩家 ID
    pub hand: Vec<Mahjong>,            // 手牌
    pub drawn_card: Option<Mahjong>,   // 摸到的牌
    pub melds: Vec<Meld>,              // 副露的牌
    pub discarded_cards: Vec<Mahjong>, // 打出的牌
    pub seat_position: SeatPosition,   // 座位位置
}

// 添加一个枚举类型表示吃碰杠的组合
#[derive(PartialEq, Debug)]
pub enum Meld {
    Chi(Vec<Mahjong>, SeatPosition),
    Pon(Vec<Mahjong>, SeatPosition),
    Kan(Vec<Mahjong>, SeatPosition),
}

impl Meld {
    fn string_meld_info(&self) -> String {
        let mut meld_info = String::new();
        let (mahjongs, seat_position) = match self {
            Meld::Chi(cards, s) => (cards, s),
            Meld::Pon(cards, s) => (cards, s),
            Meld::Kan(cards, s) => (cards, s),
        };

        for mahjong in mahjongs {
            meld_info.push_str(&format!("{}", mahjong.get_value()));
        }
        meld_info.push_str(&format!("{}", seat_position));
        meld_info
    }
}

#[derive(PartialEq)]
pub enum Action {
    Chi(Vec<Mahjong>),
    Pon(Mahjong),
    Minkan(Mahjong),
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum SeatPosition {
    East,
    South,
    West,
    North,
}

impl std::fmt::Display for SeatPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SeatPosition::East => write!(f, "e"),
            SeatPosition::South => write!(f, "s"),
            SeatPosition::West => write!(f, "w"),
            SeatPosition::North => write!(f, "n"),
        }
    }
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

    // 从牌堆中获得一张牌到 hand 中, 通常用在游戏开始时
    pub fn add_hand(&mut self, deck: &mut Deck) -> Result<()> {
        match deck.get_one_card() {
            Some(card) => {
                self.hand.push(card);
                self.sort_hand();
                return Ok(());
            }
            None => return Err(anyhow::anyhow!("drawn_card is None!")),
        }
    }

    pub fn discard(&mut self, index: usize) -> Result<Mahjong> {
        if self.drawn_card.is_none() {
            return Err(anyhow::anyhow!("drawn_card is None!"));
        }

        let drawn_card = self.drawn_card.take().unwrap();

        // 将抽到手上的牌打出
        if index == 0 {
            return Ok(drawn_card);
        }

        if index > self.hand.len() {
            self.drawn_card = Some(drawn_card); // 恢复 drawn_card
            return Err(anyhow::anyhow!("index out of range!"));
        }

        let discarded_card = self.hand.remove(index - 1);
        self.hand.push(drawn_card);
        self.sort_hand();

        Ok(discarded_card)
    }

    pub fn push_in_discarded(&mut self, card: Mahjong) {
        self.discarded_cards.push(card);
    }

    // 对手牌进行排序
    pub fn sort_hand(&mut self) {
        self.hand.sort_by(|a, b| a.cmp(b));
    }

    pub fn info(&self) -> String {
        let mut info = String::new();
        let mut iter = self.hand.iter().peekable();

        info.push_str("手牌: ");
        while let Some(this_mahjong) = iter.next() {
            info.push_str(&format!("{}", this_mahjong.get_value()));

            if let Some(next_card) = iter.peek() {
                if next_card.get_suit_order() != this_mahjong.get_suit_order() {
                    info.push_str(&format!("{} ", this_mahjong.get_suit_string()));
                }
            } else {
                // 最后一张牌
                info.push_str(&format!("{}", this_mahjong.get_suit_string()));
            }
        }

        info.push_str(" 副露: ");
        for meld in &self.melds {
            info.push_str(&format!("{} ", meld.string_meld_info()));
        }

        info
    }

    // 添加其他方法以支持玩家的游戏功能，例如摸牌、打牌、吃碰杠等
}

impl Player {
    /// 检查当前玩家是否可以吃上家的牌, 返回所有可能的吃牌组合
    pub fn can_chi(&self, mahjong: &Mahjong, seat_position: &SeatPosition) -> Vec<Vec<Mahjong>> {
        // 检查当前玩家是否可以吃上家的牌
        match (self.seat_position, seat_position) {
            (SeatPosition::East, SeatPosition::North) => (),
            (SeatPosition::South, SeatPosition::East) => (),
            (SeatPosition::West, SeatPosition::South) => (),
            (SeatPosition::North, SeatPosition::West) => (),
            _ => return Vec::new(),
        }

        if mahjong.is_z() {
            return Vec::new(); // 字牌不能吃
        }

        let mut possible_combinations = Vec::new();

        for i in -1..=1 {
            let sequence: Vec<Mahjong> = match mahjong {
                Mahjong::M(value) | Mahjong::P(value) | Mahjong::S(value) => {
                    vec![
                        mahjong.with_value(value + i - 1),
                        mahjong.with_value(value + i),
                        mahjong.with_value(value + i + 1),
                    ]
                }
                Mahjong::Z(_) => continue,
            };

            if sequence
                .iter()
                .all(|mahjong| mahjong.get_value() >= 1 && mahjong.get_value() <= 9)
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
    pub fn pon(&mut self, card: Mahjong, discard_position: &SeatPosition) -> bool {
        // 确保玩家不能碰自己打出的牌
        if &self.seat_position == discard_position {
            return false;
        }

        // 检查玩家手中是否有两张与打出的牌相同的牌
        let matching_cards: Vec<Mahjong> = self
            .hand
            .iter()
            .filter(|mahjong| mahjong == &&card)
            .cloned()
            .collect();
        if matching_cards.len() != 2 {
            return false;
        }

        // 移除手牌中的两张匹配牌，并将碰牌组合添加到碰杠组中
        self.hand.retain(|mahjong| mahjong != &card);
        self.melds
            .push(Meld::Pon(vec![card, card, card], discard_position.clone()));

        true
    }

    pub fn get_available_actions(
        &self,
        card: &Mahjong,
        discard_seat_position: &SeatPosition,
    ) -> Vec<Action> {
        let mut available_actions = Vec::new();

        // 检查是否可以吃牌
        let chi_combinations = self.can_chi(card, discard_seat_position);
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
