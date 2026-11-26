use rand::{RngExt, random, rng};
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
                let mut nums_acc = [1_u8; 17];
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

//#[derive(Debug)]
pub struct KuhnTronicNet {
    // in: [5 chunks 320 bits] -> out: [8 chunks 512 neurons]
    pub l0: BinarizedLayer<5, 512, 8>,
    // in: [8 chunks 512 bits] -> out: [8 chunks 512 neurons]
    pub l1: BinarizedLayer<8, 512, 8>,
    // in: [8 chunks 512 bits] -> out: [4 chunks 256 neurons]
    pub l2: BinarizedLayer<8, 256, 4>,
    // in: [4 chunks 256 bits] -> out: [1 chunk  64 neurons]
    pub l3: BinarizedLayer<4, 64, 1>,
}

impl KuhnTronicNet {
    pub fn new(mut seed: u64) -> Self {
        Self {
            l0: BinarizedLayer::new(&mut seed),
            l1: BinarizedLayer::new(&mut seed),
            l2: BinarizedLayer::new(&mut seed),
            l3: BinarizedLayer::new(&mut seed),
        }
    }
}

// https://www.cardplayer.com/rules-of-poker/how-to-play-poker/games/texas-holdem
fn main() {
    let mut prng_state: u64 = random();

    let mut net = KuhnTronicNet::new(splitmix64(&mut prng_state));

    let mut h0 = [0u64; 8];
    let mut h1 = [0u64; 8];
    let mut h2 = [0u64; 4];
    let mut out = [0u64; 1];

    let training_epochs: usize = 2000;
    for epoch in 0..training_epochs {
        let pre_seed: u64 = splitmix64(&mut prng_state);
        let seed: usize = ((pre_seed & 0b1111_1).saturating_sub(12)) as usize;

        // Other player hole cards
        let enemy_hole0: u16 = DECK[seed ^ 7];
        let enemy_hole1: u16 = DECK[seed ^ 32];
        let mut enemy_chips: u8 = 0b1111_1111;
        let mut enemy_pot: u16 = 0;

        // Game view
        let flop0: u16 = DECK[seed ^ 22];
        let flop1: u16 = DECK[seed ^ 27];
        let flop2: u16 = DECK[seed ^ 12];

        let turn: u16 = DECK[seed ^ 4];
        let river: u16 = DECK[seed ^ 30];

        let hole0: u16 = DECK[seed ^ 17];
        let hole1: u16 = DECK[seed ^ 2];
        let mut player_chips: u8 = 0b1111_1111;
        let mut player_pot: u16 = 0;

        // Deal Stage & Preflop
        let enemy_action = (pre_seed & 0b0000_0111) as u8;
        let enemy_bet_amt = (pre_seed & 0b1111_1111) as u8;
        if enemy_action == 0b0000_0111 {
            let shift = enemy_bet_amt.count_ones().min(7);
            enemy_chips >>= shift;
            enemy_pot = (enemy_pot | 1) << shift;
        }

        let player_rank: u16 = hand_rank_eval(flop0, flop1, flop2, hole0, hole1);

        let inputs: [u64; 5] = [
            player_pot as u64 | ((player_chips as u64) << 16),
            enemy_pot as u64 | ((enemy_chips as u64) << 16),
            /*
            flop0 as u64
                | ((flop1 as u64) << 16)
                | ((flop2 as u64) << 32)
                | ((RoundStage::Deal as u64) << 48),
            */
            0,
            hole0 as u64 | ((hole1 as u64) << 16), //| ((player_rank as u64) << 32),
            //turn as u64 | ((river as u64) << 16),  // attention mask
            0,
        ];

        net.l0.forward(&inputs, &mut h0);
        net.l1.forward(&h0, &mut h1);

        // residual skip connection
        for i in 0..8 {
            h1[i] ^= h0[i];
        }

        net.l2.forward(&h1, &mut h2);
        net.l3.forward(&h2, &mut out);

        let output: u64 = out[0];

        // output: [0-2 Action][3-11 Bet Amt][31-63 Attention Mask]
        let action = output & 0b111;
        let bet_bits = (output >> 3) & 0x1FF;
        let attention_mask = output >> 31;

        if action == 0b111 {
            let bet_shift = bet_bits.count_ones().min(7);
            player_chips >>= bet_shift;
            player_pot = (player_pot | 1) << bet_shift;
        }

        if action != 0b000 {
            // Flop

            let player_rank: u16 = hand_rank_eval(flop0, flop1, flop2, hole0, hole1);

            let inputs: [u64; 5] = [
                player_pot as u64 | ((player_chips as u64) << 16),
                enemy_pot as u64 | ((enemy_chips as u64) << 16),
                flop0 as u64
                    | ((flop1 as u64) << 16)
                    | ((flop2 as u64) << 32)
                    | ((RoundStage::Deal as u64) << 48),
                hole0 as u64 | ((hole1 as u64) << 16) | ((player_rank as u64) << 32),
                0 | ((attention_mask as u64) << 32),
            ];

            net.l0.forward(&inputs, &mut h0);
            net.l1.forward(&h0, &mut h1);

            // residual skip connection
            for i in 0..8 {
                h1[i] ^= h0[i];
            }

            net.l2.forward(&h1, &mut h2);
            net.l3.forward(&h2, &mut out);

            let output: u64 = out[0];

            // output: [0-2 Action][3-11 Bet Amt][31-63 Attention Mask]
            let action = output & 0b111;
            let bet_bits = (output >> 3) & 0x1FF;
            let attention_mask = output >> 31;

            if action == 0b111 {
                let bet_shift = bet_bits.count_ones().min(7);
                player_chips >>= bet_shift;
                player_pot = (player_pot | 1) << bet_shift;
            }

            if action != 0b000 {
                // Turn

                let player_rank: u16 = hand_rank_eval(flop0, flop1, flop2, hole0, hole1);

                let inputs: [u64; 5] = [
                    player_pot as u64 | ((player_chips as u64) << 16),
                    enemy_pot as u64 | ((enemy_chips as u64) << 16),
                    flop0 as u64
                        | ((flop1 as u64) << 16)
                        | ((flop2 as u64) << 32)
                        | ((RoundStage::Deal as u64) << 48),
                    hole0 as u64 | ((hole1 as u64) << 16) | ((player_rank as u64) << 32),
                    turn as u64 | ((attention_mask as u64) << 32),
                ];

                net.l0.forward(&inputs, &mut h0);
                net.l1.forward(&h0, &mut h1);

                // residual skip connection
                for i in 0..8 {
                    h1[i] ^= h0[i];
                }

                net.l2.forward(&h1, &mut h2);
                net.l3.forward(&h2, &mut out);

                let output: u64 = out[0];

                // output: [0-2 Action][3-11 Bet Amt][31-63 Attention Mask]
                let action = output & 0b111;
                let bet_bits = (output >> 3) & 0x1FF;
                let attention_mask = output >> 31;

                if action == 0b111 {
                    let bet_shift = bet_bits.count_ones().min(7);
                    player_chips >>= bet_shift;
                    player_pot = (player_pot | 1) << bet_shift;
                }

                if action != 0b000 {
                    // River

                    let player_rank: u16 = hand_rank_eval(flop0, flop1, flop2, hole0, hole1);

                    let inputs: [u64; 5] = [
                        player_pot as u64 | ((player_chips as u64) << 16),
                        enemy_pot as u64 | ((enemy_chips as u64) << 16),
                        flop0 as u64
                            | ((flop1 as u64) << 16)
                            | ((flop2 as u64) << 32)
                            | ((RoundStage::Deal as u64) << 48),
                        hole0 as u64 | ((hole1 as u64) << 16) | ((player_rank as u64) << 32),
                        turn as u64 | ((river as u64) << 16) | ((attention_mask as u64) << 32),
                    ];

                    net.l0.forward(&inputs, &mut h0);
                    net.l1.forward(&h0, &mut h1);

                    // residual skip connection
                    for i in 0..8 {
                        h1[i] ^= h0[i];
                    }

                    net.l2.forward(&h1, &mut h2);
                    net.l3.forward(&h2, &mut out);

                    let output: u64 = out[0];

                    // output: [0-2 Action][3-11 Bet Amt][31-63 Attention Mask]
                    let action = output & 0b111;
                    let bet_bits = (output >> 3) & 0x1FF;

                    if action == 0b111 {
                        let bet_shift = bet_bits.count_ones().min(7);
                        player_chips >>= bet_shift;
                        player_pot = (player_pot | 1) << bet_shift;
                    }

                    if action != 0b000 {
                        // Showdown
                        {
                            let cards: [u16; 7] = [flop0, flop1, flop2, hole0, hole1, turn, river];
                            let mut best_rank = 0;
                            let mut best_combo = 0;

                            for combo in 0..3125 {
                                let i0 = combo % 7;
                                let i1 = (combo / 7) % 7;
                                let i2 = (combo / 49) % 7;
                                let i3 = (combo / 343) % 7;
                                let i4 = (combo / 2401) % 7;

                                let unique_mask = (1u8 << i0)
                                    | (1u8 << i1)
                                    | (1u8 << i2)
                                    | (1u8 << i3)
                                    | (1u8 << i4);
                                if unique_mask.count_ones() != 5 {
                                    continue;
                                }

                                let rank = hand_rank_eval(
                                    cards[i0], cards[i1], cards[i2], cards[i3], cards[i4],
                                );
                                if rank > best_rank {
                                    best_rank = rank;
                                    best_combo = combo;
                                }
                            }
                            let i0 = (best_combo % 7) as u16;
                            let i1 = ((best_combo / 7) % 7) as u16;
                            let i2 = ((best_combo / 49) % 7) as u16;
                            let i3 = ((best_combo / 343) % 7) as u16;
                            let i4 = ((best_combo / 2401) % 7) as u16;
                            hand_rank_eval(i0, i1, i2, i3, i4);
                        }

                        // TODO STE backprop and model saving
                        if (epoch % 500) == 0 {
                            println!("Epoch {}", epoch);
                            /*
                            // debugging bits on one neuron
                            println!("L0[0]: {:064b}", h0[0]);
                            println!("L1[0]: {:064b}", h1[0]);
                            println!("L2[0]: {:064b}", h2[0]);
                            println!("OUT  : {:064b}", output);
                            */
                        }

                        continue;
                    }
                }
            }
        }

        // TODO STE backprop and model saving
        if (epoch % 500) == 0 {
            println!("Epoch {}", epoch);
            /*
            // debugging bits on one neuron
            println!("L0[0]: {:064b}", h0[0]);
            println!("L1[0]: {:064b}", h1[0]);
            println!("L2[0]: {:064b}", h2[0]);
            println!("OUT  : {:064b}", output);
            */
        }
    }
}
