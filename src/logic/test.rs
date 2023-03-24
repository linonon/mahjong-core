// 在文件末尾添加测试模块
#[cfg(test)]
mod tests {
    use crate::logic::{
        mahjong::{Mahjong, Suit},
        player::{Meld, Player, SeatPosition},
    };

    // 添加一个测试函数
    #[test]
    fn test_pon() {
        // 准备测试数据
        let mut player = Player::new(1, SeatPosition::East);
        player.hand = vec![
            Mahjong::new(Suit::M, 1),
            Mahjong::new(Suit::M, 1),
            Mahjong::new(Suit::M, 2),
            Mahjong::new(Suit::M, 3),
            Mahjong::new(Suit::P, 3),
            Mahjong::new(Suit::P, 3),
            Mahjong::new(Suit::P, 4),
            Mahjong::new(Suit::S, 6),
            Mahjong::new(Suit::S, 7),
            Mahjong::new(Suit::S, 8),
            Mahjong::new(Suit::Z, 2),
            Mahjong::new(Suit::Z, 4),
            Mahjong::new(Suit::Z, 4),
        ];

        let discard_card = Mahjong::new(Suit::P, 3);

        // 调用pon()函数
        let result = player.pon(discard_card, &SeatPosition::South);

        // 检查结果
        assert_eq!(result, true);
        assert_eq!(player.hand.len(), 11);
        assert_eq!(player.melds.len(), 1);
        println!("{}", player.info());
        assert_eq!(
            player.melds[0],
            Meld::Pon(
                vec![discard_card, discard_card, discard_card],
                SeatPosition::South
            )
        );
    }
}
