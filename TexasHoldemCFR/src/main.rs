use rand::{RngExt, rng};
use std::{hint, vec};
mod nnet;
use nnet::*;

#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardNum {
    Hidden = 0b0_0000_0000_0000,
    Two = 0b1_0000_0000_0000,
    Three = 0b1_1000_0000_0000,
    Four = 0b1_1100_0000_0000,
    Five = 0b1_1110_0000_0000,
    Six = 0b1_1111_0000_0000,
    Seven = 0b1_1111_1000_0000,
    Eight = 0b1_1111_1100_0000,
    Nine = 0b1_1111_1110_0000,
    Ten = 0b1_1111_1111_0000,
    Jack = 0b1_1111_1111_1000,
    Queen = 0b1_1111_1111_1100,
    King = 0b1_1111_1111_1110,
    Ace = 0b1_1111_1111_1111,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum CardSymbol {
    Clubs = 0b00,
    Diamonds = 0b01,
    Hearts = 0b10,
    Spades = 0b11,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum Action {
    Fold = 0b000,
    Check = 0b100,
    Call = 0b110,
    Bet = 0b111,
}

//https://www.youtube.com/watch?v=ep1riICX-KU
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
enum RoundStage {
    // Shuffle
    Blinds = 0b0000,
    Deal = 0b0001,
    Flop = 0b0011,
    Turn = 0b0111,
    River = 0b1111,
    //Showdown/Winner selected
}

pub const fn card_to_magnitude(card_num: CardNum, card_symbol: CardSymbol) -> u16 {
    //let rank_bits = ((1u16 << ((card_num as u8) + 1)) - 1) << 2;
    let rank_bits = card_num as u16;
    rank_bits | ((card_symbol as u16).reverse_bits())
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

#[inline]
fn hand_rank_eval(flop0: u16, flop1: u16, flop2: u16, hole0: u16, hole1: u16) -> u16 {
    // https://officialpokerrules.com/hand-rankings/

    const NUM_MASK: u16 = 0b1_1111_1111_1111;
    const SYMBOL_MASK: u16 = 0b0_0000_0000_0000_011;

    // Royal Flush
    {
        let f0_ace = (((flop0 & NUM_MASK) ^ CardNum::Ace as u16) == 0) as u16;
        let f0_king = (((flop0 & NUM_MASK) ^ CardNum::King as u16) == 0) as u16;
        let f0_queen = (((flop0 & NUM_MASK) ^ CardNum::Queen as u16) == 0) as u16;
        let f0_jack = (((flop0 & NUM_MASK) ^ CardNum::Jack as u16) == 0) as u16;
        let f0_ten = (((flop0 & NUM_MASK) ^ CardNum::Ten as u16) == 0) as u16;
        let f0_sym = flop0 & SYMBOL_MASK;

        let f1_ace = (((flop1 & NUM_MASK) ^ CardNum::Ace as u16) == 0) as u16;
        let f1_king = (((flop1 & NUM_MASK) ^ CardNum::King as u16) == 0) as u16;
        let f1_queen = (((flop1 & NUM_MASK) ^ CardNum::Queen as u16) == 0) as u16;
        let f1_jack = (((flop1 & NUM_MASK) ^ CardNum::Jack as u16) == 0) as u16;
        let f1_ten = (((flop1 & NUM_MASK) ^ CardNum::Ten as u16) == 0) as u16;
        let f1_sym = flop1 & SYMBOL_MASK;

        let f2_ace = (((flop2 & NUM_MASK) ^ CardNum::Ace as u16) == 0) as u16;
        let f2_king = (((flop2 & NUM_MASK) ^ CardNum::King as u16) == 0) as u16;
        let f2_queen = (((flop2 & NUM_MASK) ^ CardNum::Queen as u16) == 0) as u16;
        let f2_jack = (((flop2 & NUM_MASK) ^ CardNum::Jack as u16) == 0) as u16;
        let f2_ten = (((flop2 & NUM_MASK) ^ CardNum::Ten as u16) == 0) as u16;
        let f2_sym = flop2 & SYMBOL_MASK;

        let h0_ace = (((hole0 & NUM_MASK) ^ CardNum::Ace as u16) == 0) as u16;
        let h0_king = (((hole0 & NUM_MASK) ^ CardNum::King as u16) == 0) as u16;
        let h0_queen = (((hole0 & NUM_MASK) ^ CardNum::Queen as u16) == 0) as u16;
        let h0_jack = (((hole0 & NUM_MASK) ^ CardNum::Jack as u16) == 0) as u16;
        let h0_ten = (((hole0 & NUM_MASK) ^ CardNum::Ten as u16) == 0) as u16;
        let h0_sym = hole0 & SYMBOL_MASK;

        let h1_ace = (((hole1 & NUM_MASK) ^ CardNum::Ace as u16) == 0) as u16;
        let h1_king = (((hole1 & NUM_MASK) ^ CardNum::King as u16) == 0) as u16;
        let h1_queen = (((hole1 & NUM_MASK) ^ CardNum::Queen as u16) == 0) as u16;
        let h1_jack = (((hole1 & NUM_MASK) ^ CardNum::Jack as u16) == 0) as u16;
        let h1_ten = (((hole1 & NUM_MASK) ^ CardNum::Ten as u16) == 0) as u16;
        let h1_sym = hole1 & SYMBOL_MASK;

        let has_ace = f0_ace | f1_ace | f2_ace | h0_ace | h1_ace;
        let has_king = f0_king | f1_king | f2_king | h0_king | h1_king;
        let has_queen = f0_queen | f1_queen | f2_queen | h0_queen | h1_queen;
        let has_jack = f0_jack | f1_jack | f2_jack | h0_jack | h1_jack;
        let has_ten = f0_ten | f1_ten | f2_ten | h0_ten | h1_ten;
        let same_sym =
            (f0_sym ^ f1_sym) | (f0_sym ^ f2_sym) | (f0_sym ^ h0_sym) | (f0_sym ^ h1_sym);

        let royal_flush = has_ace | has_king | has_queen | has_jack | has_ten | same_sym;
        if royal_flush == 0 {
            return 0b111_1111_11;
        } else {
            //Stright Flush

            let f0_z = (flop0 & NUM_MASK).count_ones();
            let f1_z = (flop1 & NUM_MASK).count_ones();
            let f2_z = (flop2 & NUM_MASK).count_ones();
            let h0_z = (hole0 & NUM_MASK).count_ones();
            let h1_z = (hole1 & NUM_MASK).count_ones();

            let pass: u64 =
                (1u64 << f0_z) | (1u64 << f1_z) | (1u64 << f2_z) | (1u64 << h0_z) | (1u64 << h1_z);

            if ((pass >> pass.trailing_zeros()) == 0b1_1111) && (same_sym == 0) {
                return 0b111_1111_10;
            } else {
                // Four Of a Kind
                let mut nums_acc = [1_u8; 16];
                nums_acc[flop0.count_zeros() as usize] <<= 1;
                nums_acc[flop1.count_zeros() as usize] <<= 1;
                nums_acc[flop2.count_zeros() as usize] <<= 1;
                nums_acc[hole0.count_zeros() as usize] <<= 1;
                nums_acc[hole1.count_zeros() as usize] <<= 1;

                let mut four_kind: bool = false;
                for i in 0..13 {
                    if nums_acc[i] > 3 {
                        four_kind = true;
                    }
                }
                if four_kind {
                    return 0b111_1111_00;
                } else {
                    // Full House
                    let mut three_kind: bool = false;
                    let mut two_kind: bool = false;
                    for i in 0..13 {
                        if nums_acc[i] == 3 {
                            three_kind = true;
                        } else if nums_acc[i] == 2 {
                            two_kind = true;
                        }
                    }
                    if three_kind && two_kind {
                        return 0b111_1110_00;
                    } else {
                        // Flush
                        if same_sym == 0 {
                            return 0b111_1100_00;
                        } else {
                            // Straight
                            let f0_z = (flop0 & NUM_MASK).count_ones();
                            let f1_z = (flop1 & NUM_MASK).count_ones();
                            let f2_z = (flop2 & NUM_MASK).count_ones();
                            let h0_z = (hole0 & NUM_MASK).count_ones();
                            let h1_z = (hole1 & NUM_MASK).count_ones();

                            let pass: u64 = (1u64 << f0_z)
                                | (1u64 << f1_z)
                                | (1u64 << f2_z)
                                | (1u64 << h0_z)
                                | (1u64 << h1_z);

                            if (pass >> pass.trailing_zeros()) == 0b1_1111 {
                                return 0b111_1000_00;
                            } else {
                                // Three Of a Kind
                                let mut three_kind: bool = false;
                                for i in 0..13 {
                                    if nums_acc[i] > 2 {
                                        three_kind = true;
                                    }
                                }
                                if three_kind {
                                    return 0b111_0000_00;
                                } else {
                                    // Two Pair
                                    let mut two_kind = 0;
                                    for i in 0..13 {
                                        if nums_acc[i] > 1 {
                                            two_kind += 1;
                                        }
                                    }
                                    if two_kind == 2 {
                                        return 0b110_0000_00;
                                    } else {
                                        // One Pair
                                        let mut two_kind: bool = false;
                                        for i in 0..13 {
                                            if nums_acc[i] > 1 {
                                                two_kind = true;
                                            }
                                        }
                                        if two_kind {
                                            return 0b100_0000_00;
                                        } else {
                                            // High Card
                                            return 0;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// https://www.cardplayer.com/rules-of-poker/how-to-play-poker/games/texas-holdem
fn main() {
    let mut rng = rng();

    // 5 in -> 50 out 3200 neurons
    let mut layer0 = BinarizedLayer::new(5, 50);
    let mut hidden0 = vec![0u64; 50];

    // 50 in -> 25 out 1,600 neurons
    let mut layer1 = BinarizedLayer::new(50, 25);
    let mut hidden1 = vec![0u64; 25];

    // 25 in -> 15 out 960 neurons
    let mut layer2 = BinarizedLayer::new(25, 15);
    let mut hidden2 = vec![0u64; 15];

    // 15 in -> 4 out 256 neurons
    let mut layer3 = BinarizedLayer::new(15, 4);
    let mut hidden3 = vec![0u64; 4];

    // 4 in -> 2 out 128 neurons
    let mut layer4 = BinarizedLayer::new(4, 2);
    let mut hidden4 = vec![0u64; 2];

    // 2 in -> 1 out 64 neurons
    let mut layer5 = BinarizedLayer::new(2, 1);
    let mut hidden5 = vec![0u64; 1];
    // outputs are still expected to be themormeter encoded
    // action and bet amount bits: [0-2 Action ][3-11 bet amt][31-63 attention mask]

    let training_epochs: usize = 2000;
    for epoch in 0..training_epochs {
        println!("{:?}", epoch);
        let pre_seed: u64 = rng.random::<u64>();
        let seed: usize = ((pre_seed & 0b1111_1).saturating_sub(12)) as usize;

        // other player hole cards
        let enemy_hole0: u16 = DECK[seed ^ 7];
        let enemy_hole1: u16 = DECK[seed ^ 32];
        let mut enemy_chips: u8 = 0b1111_1111;
        let mut enemy_pot: u16 = 0;

        // game view
        let flop0: u16 = DECK[seed ^ 22];
        let flop1: u16 = DECK[seed ^ 27];
        let flop2: u16 = DECK[seed ^ 12];

        let turn: u16 = DECK[seed ^ 4];
        let river: u16 = DECK[seed ^ 30];

        let hole0: u16 = DECK[seed ^ 17];
        let hole1: u16 = DECK[seed ^ 2];
        let mut player_chips: u8 = 0b1111_1111;
        let mut player_pot: u16 = 0;

        // forced blinds stage is not required
        // Deal Stage & Preflop
        // TODO: enmey action and bet handling
        let enemy_action = (pre_seed & 0b0000_0111) as u8;
        let enemy_bet_amt = (pre_seed & 0b0000_0000_1111_1111) as u8;
        if enemy_action == 0b0000_0111 {
            enemy_chips >>= enemy_bet_amt.count_ones();
            enemy_pot <<= enemy_bet_amt.count_ones();
        }

        let player_rank: u16 = hand_rank_eval(flop0, flop1, flop2, hole0, hole1);
        let inputs: Vec<u64> = vec![
            player_pot as u64 | ((player_chips as u64) << 16),
            enemy_pot as u64 | ((enemy_chips as u64) << 16),
            /*
            flop0 as u64
                | ((flop1 as u64) << 16)
                | ((flop2 as u64) << 32)
                | ((RoundStage::Deal as u64) << 48),
            */
            0,
            hole0 as u64 | ((hole1 as u64) << 16), // hand_rank
            turn as u64 | ((river as u64) << 16),  // attention masks
        ];
        layer0.forward(&inputs, &mut hidden0, 50);
        println!("{:b}", hidden0[0]);
        layer1.forward(&hidden0, &mut hidden1, 25);
        println!("{:b}", hidden1[0]);
        layer2.forward(&hidden1, &mut hidden2, 15);
        println!("{:b}", hidden2[0]);
        layer3.forward(&hidden2, &mut hidden3, 4);
        println!("{:b}", hidden3[0]);
        layer4.forward(&hidden3, &mut hidden4, 2);
        println!("{:b}", hidden4[0]);
        layer5.forward(&hidden4, &mut hidden5, 1);
        let output: u64 = hidden5[0];
        if (output & 0b111) == 0b111 {
            player_chips >>= output.count_ones();
            player_pot <<= output.count_ones();
        }
        println!("{:b}", output);

        // later stage
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
