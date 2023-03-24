use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mahjong {
    M(i8), // 万子
    P(i8), // 饼子
    S(i8), // 索子
    Z(i8), // 字牌（东、南、西、北、中、发、白）
}

impl Mahjong {
    /// 创建一个类型相同, 自订数值的 Mahjong
    pub fn with_value(&self, v: i8) -> Mahjong {
        match self {
            Mahjong::M(_) => Mahjong::M(v),
            Mahjong::P(_) => Mahjong::P(v),
            Mahjong::S(_) => Mahjong::S(v),
            Mahjong::Z(_) => Mahjong::Z(v),
        }
    }

    pub fn cmp(&self, b: &Mahjong) -> Ordering {
        let a = self;

        let suit_order = a.get_suit_order().cmp(&b.get_suit_order());
        if suit_order != Ordering::Equal {
            suit_order
        } else {
            let mut a = a.get_value();
            let mut b = b.get_value();
            if a == 0 {
                a = 5;
            }
            if b == 0 {
                b = 5;
            }

            a.cmp(&b)
        }
    }

    pub fn get_suit_order(&self) -> u8 {
        match self {
            Mahjong::M(_) => 0,
            Mahjong::P(_) => 1,
            Mahjong::S(_) => 2,
            Mahjong::Z(_) => 3,
        }
    }

    pub fn get_suit_string(&self) -> &str {
        match self {
            Mahjong::M(_) => "m",
            Mahjong::P(_) => "p",
            Mahjong::S(_) => "s",
            Mahjong::Z(_) => "z",
        }
    }

    pub fn is_z(&self) -> bool {
        match self {
            Mahjong::Z(_) => true,
            _ => false,
        }
    }

    pub fn get_value(&self) -> i8 {
        match self {
            Mahjong::M(value) => *value,
            Mahjong::P(value) => *value,
            Mahjong::S(value) => *value,
            Mahjong::Z(value) => *value,
        }
    }
    pub fn set_value(&mut self, new_value: i8) {
        *self = match *self {
            Mahjong::M(_) => Mahjong::M(new_value),
            Mahjong::P(_) => Mahjong::P(new_value),
            Mahjong::S(_) => Mahjong::S(new_value),
            Mahjong::Z(_) => Mahjong::Z(new_value),
        };
    }
}

impl Display for Mahjong {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (suit, value) = match self {
            Mahjong::M(value) => ("m", value),
            Mahjong::P(value) => ("p", value),
            Mahjong::S(value) => ("s", value),
            Mahjong::Z(value) => ("z", value),
        };

        write!(f, "{}{}", value, suit)
    }
}
