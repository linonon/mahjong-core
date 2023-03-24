// 在文件末尾添加测试模块
#[cfg(test)]
mod tests {
    use crate::logic::{
        mahjong::Mahjong,
        player::{Meld, Player, SeatPosition},
    };

    // 添加一个测试函数
    #[test]
    fn test_pon() {
        // 准备测试数据
        let mut player = Player::new(1, SeatPosition::East);
        player.hand = vec![
            Mahjong::M(1),
            Mahjong::M(1),
            Mahjong::M(2),
            Mahjong::M(3),
            Mahjong::P(3),
            Mahjong::P(3),
            Mahjong::P(4),
            Mahjong::S(6),
            Mahjong::S(7),
            Mahjong::S(8),
            Mahjong::Z(2),
            Mahjong::Z(4),
            Mahjong::Z(4),
        ];

        let discard_card = Mahjong::P(3);

        // 调用pon()函数
        println!("before: {}", player.info());
        let result = player.pon(discard_card, &SeatPosition::South);

        // 检查结果
        assert_eq!(result, true);
        assert_eq!(player.hand.len(), 11);
        assert_eq!(player.melds.len(), 1);
        println!("after: {}", player.info());
        assert_eq!(
            player.melds[0],
            Meld::Pon(
                vec![discard_card, discard_card, discard_card],
                SeatPosition::South
            )
        );
    }
}
