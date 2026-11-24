use NueralBinarizedRS::*;
use rand::{RngExt, rng};

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum CardNum {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum CardSymbol {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Action {
    Fold,
    Check,
    Call,
    Bet,
}

pub fn card_to_magnitude(card_num: CardNum, card_symbol: CardSymbol) -> u16 {
    /*
    let mut r0: u16 = 0;
    for _ in 0..((card_num as u8) + 1) {
        r0 = (r0 << 1) | 1;
    }
    r0 &= 0b1111_1111_1111_1100;
    match card_symbol {
        CardSymbol::Clubs => {}
        CardSymbol::Diamonds => r0 |= 0b0000_0000_0000_0001,
        CardSymbol::Hearts => r0 |= 0b0000_0000_0000_0010,
        CardSymbol::Spades => r0 |= 0b0000_0000_0000_0011,
    }
    return r0;*/
    let rank_bits = ((1u16 << ((card_num as u8) + 1)) - 1) << 2;
    rank_bits | (card_symbol as u16)
}

// https://www.cardplayer.com/rules-of-poker/how-to-play-poker/games/texas-holdem
fn main() {
    let mut rng = rng();

    let training_epochs: usize = 2000;
    for epoch in 0..training_epochs {
        println!("{:?}", epoch);
        // other player hole cards
        let mut enemy_hole0: u16 = rng.random();
        let mut enemy_hole1: u16 = rng.random();
        let mut enmey_chips: u8 = 16;

        // game view
        let mut flop0: u16 = rng.random();
        let mut flop1: u16 = rng.random();
        let mut flop2: u16 = rng.random();

        let mut turn: u16 = rng.random();
        let mut river: u16 = rng.random();
        let mut pot: u8 = 0;

        let mut hole0: u16 = rng.random();
        let mut hole1: u16 = rng.random();
        let mut player_chips: u8 = 16;

        let mut layer0 = BinarizedLayer::new(3);
        let mut layer1 = BinarizedLayer::new(6);
        let mut layer2 = BinarizedLayer::new(3);
        let mut layer3 = BinarizedLayer::new(1);
    }
}
