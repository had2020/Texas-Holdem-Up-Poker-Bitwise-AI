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
enum Action {
    Fold = 0b00,
    Check = 0b01,
    Call = 0b10,
    Bet = 0b11,
}

//https://www.youtube.com/watch?v=ep1riICX-KU
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum RoundStage {
    // Shuffle
    Blinds = 0b000000,
    Deal = 0b000011,
    PreFlop = 0b000111,
    Flop = 0b001111,
    Turn = 0b011111,
    River = 0b111111,
    //Showdown/Winner selected
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

    let mut layer0 = BinarizedLayer::new(3);
    let mut layer1 = BinarizedLayer::new(6);
    let mut layer2 = BinarizedLayer::new(3);
    let mut layer3 = BinarizedLayer::new(1);

    let training_epochs: usize = 2000;
    for epoch in 0..training_epochs {
        println!("{:?}", epoch);
        let pre_seed: u16 = rng.random::<u16>();
        let seed: usize = ((pre_seed & 0b11111).saturating_sub(12)) as usize;

        // other player hole cards
        let mut enemy_hole0: u16 = DECK[seed ^ 7];
        let mut enemy_hole1: u16 = DECK[seed ^ 32];
        let mut enemy_chips: u16 = 0b11111111;
        let mut enemy_pot: u16 = 0;

        // game view
        let mut flop0: u16 = DECK[seed ^ 22];
        let mut flop1: u16 = DECK[seed ^ 27];
        let mut flop2: u16 = DECK[seed ^ 12];

        //let mut turn: u16 = DECK[seed ^ 4];
        //let mut river: u16 = DECK[seed ^ 30];
        let mut turn: u16 = 0;
        let mut river: u16 = 0;

        let mut hole0: u16 = DECK[seed ^ 17];
        let mut hole1: u16 = DECK[seed ^ 2];
        let mut player_chips: u16 = 0b11111111;
        let mut player_pot: u16 = 0;

        // TODO get player and enemy card hand ranks

        // forced blind bets
    }
}

/*
fn main() {
    let mut layer0 = BinarizedLayer::new(2);
    let mut layer1 = BinarizedLayer::new(1);

    let input0: u64 = 7;
    let input1: u64 = 4;
    let target0: u64 = 3;
    let input_data = vec![input0, input1];

    let mut hidden_activations = vec![0; 1];
    let mut final_activations = vec![0; 1];

    let mut hidden_errors = vec![0; 64];
    let mut final_errors = vec![0; 64];

    let learning_rate_fp = (0.1 * (FP_ONE as f32)) as i32;

    println!("Target: {:064b}", target0);

    for epoch in 0..=2000 {
        layer0.forward(&input_data, &mut hidden_activations);
        layer1.forward(&hidden_activations, &mut final_activations);

        let current_output = final_activations[0];
        let error_bits = (current_output ^ target0).count_ones();

        if epoch % 200 == 0 || error_bits == 0 {
            println!(
                "Epoch {:4} | Bit Errors: {:2} | Out: {:064b}",
                epoch, error_bits, current_output
            );
            if error_bits == 0 {
                break;
            }
        }

        for i in 0..64 {
            let target_bit = (target0 >> i) & 1;
            let actual_bit = (current_output >> i) & 1;

            final_errors[i] = (target_bit as i32 - actual_bit as i32) * FP_ONE;
        }

        layer1.backward_ste(
            &hidden_activations,
            &final_errors,
            Some(&mut hidden_errors),
            learning_rate_fp,
        );

        layer0.backward_ste(&input_data, &hidden_errors, None, learning_rate_fp);
    }
}
 */
