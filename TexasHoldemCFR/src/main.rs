use NueralBinarizedRS::*;
use rand::{RngExt, random_range, rng};

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

pub const fn card_to_magnitude(card_num: CardNum, card_symbol: CardSymbol) -> u16 {
    let rank_bits = ((1u16 << ((card_num as u8) + 1)) - 1) << 2;
    rank_bits | (card_symbol as u16)
}

const DECK: [u16; 52] = [
    card_to_magnitude(CardNum::Two, CardSymbol::Clubs),
    card_to_magnitude(CardNum::Three, CardSymbol::Clubs),
    card_to_magnitude(CardNum::Four, CardSymbol::Clubs),
    card_to_magnitude(CardNum::Five, CardSymbol::Clubs),
    card_to_magnitude(CardNum::Six, CardSymbol::Clubs),
    card_to_magnitude(CardNum::Seven, CardSymbol::Clubs),
    card_to_magnitude(CardNum::Eight, CardSymbol::Clubs),
    card_to_magnitude(CardNum::Nine, CardSymbol::Clubs),
    card_to_magnitude(CardNum::Ten, CardSymbol::Clubs),
    card_to_magnitude(CardNum::Jack, CardSymbol::Clubs),
    card_to_magnitude(CardNum::Queen, CardSymbol::Clubs),
    card_to_magnitude(CardNum::King, CardSymbol::Clubs),
    card_to_magnitude(CardNum::Ace, CardSymbol::Clubs),
    card_to_magnitude(CardNum::Two, CardSymbol::Diamonds),
    card_to_magnitude(CardNum::Three, CardSymbol::Diamonds),
    card_to_magnitude(CardNum::Four, CardSymbol::Diamonds),
    card_to_magnitude(CardNum::Five, CardSymbol::Diamonds),
    card_to_magnitude(CardNum::Six, CardSymbol::Diamonds),
    card_to_magnitude(CardNum::Seven, CardSymbol::Diamonds),
    card_to_magnitude(CardNum::Eight, CardSymbol::Diamonds),
    card_to_magnitude(CardNum::Nine, CardSymbol::Diamonds),
    card_to_magnitude(CardNum::Ten, CardSymbol::Diamonds),
    card_to_magnitude(CardNum::Jack, CardSymbol::Diamonds),
    card_to_magnitude(CardNum::Queen, CardSymbol::Diamonds),
    card_to_magnitude(CardNum::King, CardSymbol::Diamonds),
    card_to_magnitude(CardNum::Ace, CardSymbol::Diamonds),
    card_to_magnitude(CardNum::Two, CardSymbol::Hearts),
    card_to_magnitude(CardNum::Three, CardSymbol::Hearts),
    card_to_magnitude(CardNum::Four, CardSymbol::Hearts),
    card_to_magnitude(CardNum::Five, CardSymbol::Hearts),
    card_to_magnitude(CardNum::Six, CardSymbol::Hearts),
    card_to_magnitude(CardNum::Seven, CardSymbol::Hearts),
    card_to_magnitude(CardNum::Eight, CardSymbol::Hearts),
    card_to_magnitude(CardNum::Nine, CardSymbol::Hearts),
    card_to_magnitude(CardNum::Ten, CardSymbol::Hearts),
    card_to_magnitude(CardNum::Jack, CardSymbol::Hearts),
    card_to_magnitude(CardNum::Queen, CardSymbol::Hearts),
    card_to_magnitude(CardNum::King, CardSymbol::Hearts),
    card_to_magnitude(CardNum::Ace, CardSymbol::Hearts),
    card_to_magnitude(CardNum::Two, CardSymbol::Spades),
    card_to_magnitude(CardNum::Three, CardSymbol::Spades),
    card_to_magnitude(CardNum::Four, CardSymbol::Spades),
    card_to_magnitude(CardNum::Five, CardSymbol::Spades),
    card_to_magnitude(CardNum::Six, CardSymbol::Spades),
    card_to_magnitude(CardNum::Seven, CardSymbol::Spades),
    card_to_magnitude(CardNum::Eight, CardSymbol::Spades),
    card_to_magnitude(CardNum::Nine, CardSymbol::Spades),
    card_to_magnitude(CardNum::Ten, CardSymbol::Spades),
    card_to_magnitude(CardNum::Jack, CardSymbol::Spades),
    card_to_magnitude(CardNum::Queen, CardSymbol::Spades),
    card_to_magnitude(CardNum::King, CardSymbol::Spades),
    card_to_magnitude(CardNum::Ace, CardSymbol::Spades),
];

// https://www.cardplayer.com/rules-of-poker/how-to-play-poker/games/texas-holdem
fn main() {
    let mut rng = rng();

    let training_epochs: usize = 2000;
    for epoch in 0..training_epochs {
        println!("{:?}", epoch);
        let seed: usize = ((rng.random::<u8>() & 0b11111).saturating_sub(12)) as usize;

        // other player hole cards
        let mut enemy_hole0: u16 = DECK[seed ^ 7];
        let mut enemy_hole1: u16 = DECK[seed ^ 32];
        let mut enmey_chips: u8 = 4;

        // game view
        let mut flop0: u16 = DECK[seed ^ 22];
        let mut flop1: u16 = DECK[seed ^ 27];
        let mut flop2: u16 = DECK[seed ^ 12];

        let mut turn: u16 = DECK[seed ^ 4];
        let mut river: u16 = DECK[seed ^ 30];
        let mut pot: u8 = 0;

        let mut hole0: u16 = DECK[seed ^ 17];
        let mut hole1: u16 = DECK[seed ^ 2];
        let mut player_chips: u8 = 4;

        let mut layer0 = BinarizedLayer::new(3);
        let mut layer1 = BinarizedLayer::new(6);
        let mut layer2 = BinarizedLayer::new(3);
        let mut layer3 = BinarizedLayer::new(1);
    }
}
