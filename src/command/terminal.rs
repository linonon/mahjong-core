use core::panic;
use std::io::{self, Write};

use crate::g::constants;
use crate::logic::deck::Deck;
use crate::logic::mahjong::Mahjong;
use crate::logic::player::Player;
use crate::logic::player::SeatPosition::*;

/// # Demo 版打牌
pub fn run() {
    let mut deck = Deck::new_game();

    // 分配位置: TODO: 随机分配
    let mut player1 = Player::new(1, East);
    let mut player2 = Player::new(2, South);
    let mut player3 = Player::new(3, West);
    let mut player4 = Player::new(4, North);

    // 按照顺序抽牌
    for _ in 0..constants::HAND_CARD_COUNT {
        player1.add_hand(&mut deck);
        player2.add_hand(&mut deck);
        player3.add_hand(&mut deck);
        player4.add_hand(&mut deck);
    }

    // 按照顺序循环打牌
    loop {
        show_players_cards(&player1, &player2, &player3, &player4);
        round(&mut player1, &mut deck);
        clear_screen();
        round_auto(&mut player2, &mut deck);
        round_auto(&mut player3, &mut deck);
        round_auto(&mut player4, &mut deck);
    }
}

fn show_players_cards(player1: &Player, player2: &Player, player3: &Player, player4: &Player) {
    println!("Player 1's hand: {}", player1.info());
    println!("Player 2's hand: {}", player2.info());
    println!("Player 3's hand: {}", player3.info());
    println!("Player 4's hand: {}", player4.info());
}

fn clear_screen() {
    print!("{}[2J", 27 as char,);
    io::stdout().flush().unwrap();
}

fn round(player: &mut Player, deck: &mut Deck) {
    player.draw_card(deck);
    println!(
        "Player {}'s drawn card: {}",
        player.id,
        player.drawn_card.unwrap()
    );
    println!(
        "Which card player {} wants to discard? (0 for drawn card, 1-13 for hand)",
        player.id
    );
    player_discard(player);

    // TODO: 检查玩家打出的牌后,
    // deck.check_user_card_info();
}

fn round_auto(player: &mut Player, deck: &mut Deck) {
    player.draw_card(deck);
    println!(
        "Player {}'s drawn card: {}",
        player.id,
        player.drawn_card.unwrap()
    );
    player_discard_random(player);
}

use anyhow::Result;

fn player_discard(player: &mut Player) -> Result<Mahjong> {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            // TODO: 这里只是按照 index 打出牌, 后续改成根据牌的名字打出牌
            let input: usize = input.trim().parse()?;
            let discard = player.discard(input)?;

            println!("Player {} discarded {}", player.id, discard);
            Ok(discard)
        }
        Err(error) => Err(error.into()),
    }
}

fn player_discard_random(player: &mut Player) {
    let input: usize = rand::random::<usize>() % 14;
    let discard = player.discard(input).unwrap();
    println!("Player {} discarded {}", player.id, discard);
}
