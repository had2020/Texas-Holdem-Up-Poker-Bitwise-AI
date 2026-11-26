pub const FP_SHIFT: u32 = 16;
pub const FP_ONE: i32 = 1 << FP_SHIFT;
pub const CLAMP_MAX: i32 = FP_ONE + (FP_ONE >> 1);
pub const CLAMP_MIN: i32 = -CLAMP_MAX;

#[inline(always)]
pub fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

//#[derive(Debug)]
#[repr(C, align(64))]
pub struct ForwardLayer<const IN_CHUNKS: usize, const OUT_NEURONS: usize, const OUT_CHUNKS: usize> {
    pub weights: [[u64; IN_CHUNKS]; OUT_NEURONS],
    pub thresholds: [u32; OUT_NEURONS],
}

//#[derive(Debug)]
#[repr(C, align(64))]
pub struct TrainState<const IN_CHUNKS: usize, const OUT_NEURONS: usize> {
    pub shadow_weights: [[[i32; 64]; IN_CHUNKS]; OUT_NEURONS],
    pub shadow_thresholds: [i32; OUT_NEURONS],
}

//#[derive(Debug)]
pub struct BinarizedLayer<const IN_CHUNKS: usize, const OUT_NEURONS: usize, const OUT_CHUNKS: usize>
{
    pub fwd: ForwardLayer<IN_CHUNKS, OUT_NEURONS, OUT_CHUNKS>,
    pub train: Box<TrainState<IN_CHUNKS, OUT_NEURONS>>,
}

impl<const IN_CHUNKS: usize, const OUT_NEURONS: usize, const OUT_CHUNKS: usize>
    BinarizedLayer<IN_CHUNKS, OUT_NEURONS, OUT_CHUNKS>
{
    pub fn new(seed: &mut u64) -> Self {
        const {
            assert!(
                OUT_NEURONS == OUT_CHUNKS * 64,
                "OUT_NEURONS must equal OUT_CHUNKS * 64"
            )
        };

        let mut weights = [[0u64; IN_CHUNKS]; OUT_NEURONS];
        let mut shadow_weights = vec![[[0i32; 64]; IN_CHUNKS]; OUT_NEURONS];

        for n in 0..OUT_NEURONS {
            for c in 0..IN_CHUNKS {
                let mask = splitmix64(seed);
                weights[n][c] = mask;
                for b in 0..64 {
                    let bit = ((mask >> b) & 1) as i32;
                    shadow_weights[n][c][b] = ((bit << 1) - 1) * FP_ONE;
                }
            }
        }

        let init_thresh_int = (IN_CHUNKS * 32) as u32;
        let init_thresh_fp = (init_thresh_int as i32) << FP_SHIFT;

        let boxed_shadows: Box<[[[i32; 64]; IN_CHUNKS]; OUT_NEURONS]> =
            shadow_weights.into_boxed_slice().try_into().unwrap();

        Self {
            fwd: ForwardLayer {
                weights,
                thresholds: [init_thresh_int; OUT_NEURONS],
            },
            train: Box::new(TrainState {
                shadow_weights: *boxed_shadows,
                shadow_thresholds: [init_thresh_fp; OUT_NEURONS],
            }),
        }
    }

    #[inline(always)]
    pub fn forward(&self, inputs: &[u64; IN_CHUNKS], outputs: &mut [u64; OUT_CHUNKS]) {
        for out_c in 0..OUT_CHUNKS {
            let mut packed_out = 0u64;
            let base_n = out_c * 64;

            for bit in 0..64 {
                let n = base_n + bit;
                let w = &self.fwd.weights[n];

                let mut acc0 = 0u32;
                let mut acc1 = 0u32;
                let mut acc2 = 0u32;
                let mut acc3 = 0u32;

                let mut c = 0;
                while c + 4 <= IN_CHUNKS {
                    acc0 += (!(inputs[c] ^ w[c])).count_ones();
                    acc1 += (!(inputs[c + 1] ^ w[c + 1])).count_ones();
                    acc2 += (!(inputs[c + 2] ^ w[c + 2])).count_ones();
                    acc3 += (!(inputs[c + 3] ^ w[c + 3])).count_ones();
                    c += 4;
                }
                while c < IN_CHUNKS {
                    acc0 += (!(inputs[c] ^ w[c])).count_ones();
                    c += 1;
                }

                let popcnt = (acc0 + acc1) + (acc2 + acc3);
                let fired = (popcnt >= self.fwd.thresholds[n]) as u64;
                packed_out |= fired << bit;
            }
            outputs[out_c] = packed_out;
        }
    }

    pub fn backward_ste(
        &mut self,
        inputs: &[u64; IN_CHUNKS],
        output_errors: &[i32; OUT_NEURONS],
        mut prev_layer_errors: Option<&mut [i32]>,
        learning_rate_fp: i32,
    ) {
        if let Some(ref mut errs) = prev_layer_errors {
            errs.fill(0);
        }

        let max_thresh_fp = ((IN_CHUNKS * 64) as i32) << FP_SHIFT;

        for n in 0..OUT_NEURONS {
            let error_fp = output_errors[n];
            if error_fp == 0 {
                continue;
            }

            let delta = ((error_fp as i64 * learning_rate_fp as i64) >> FP_SHIFT) as i32;

            let new_t = self.train.shadow_thresholds[n]
                .saturating_sub(delta)
                .clamp(0, max_thresh_fp);
            self.train.shadow_thresholds[n] = new_t;
            self.fwd.thresholds[n] = (new_t >> FP_SHIFT) as u32;

            for c in 0..IN_CHUNKS {
                let in_chunk = inputs[c];
                let fwd_mask = self.fwd.weights[n][c];
                let shadows = &mut self.train.shadow_weights[n][c];
                let mut new_mask = 0u64;

                for b in 0..64 {
                    if let Some(ref mut prev_errs) = prev_layer_errors {
                        let w_dir = (((fwd_mask >> b) & 1) as i32 * 2) - 1;
                        let target_idx = c * 64 + b;
                        prev_errs[target_idx] =
                            prev_errs[target_idx].saturating_add(error_fp.saturating_mul(w_dir));
                    }

                    let in_dir = (((in_chunk >> b) & 1) as i32 * 2) - 1;
                    let updated = shadows[b]
                        .saturating_add(delta.saturating_mul(in_dir))
                        .clamp(CLAMP_MIN, CLAMP_MAX);
                    shadows[b] = updated;

                    let bit_val = ((updated >> 31) + 1) as u64;
                    new_mask |= bit_val << b;
                }
                self.fwd.weights[n][c] = new_mask;
            }
        }
    }
}
