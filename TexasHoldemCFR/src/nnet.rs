use rand::random;

pub const FP_SHIFT: u32 = 16;
// 1.0 in fixed-point representation binary 1 followed by 16 zeros
pub const FP_ONE: i32 = 1 << FP_SHIFT;

#[repr(C, align(64))]
pub struct BinarizedLayer {
    input_chunks: usize,

    forward_weights: Vec<u64>,

    shadow_weights: Vec<i32>,

    thresholds: Vec<i32>,
}

impl BinarizedLayer {
    pub fn new(input_chunks: usize) -> Self {
        let num_neurons: usize = 64;
        let total_chunks = input_chunks * num_neurons;

        let mut forward_weights = vec![0; total_chunks];
        let mut shadow_weights = vec![0; total_chunks * 64];

        let mut rng_seed: u64 = 0x87654321_12345678 ^ random::<u64>();

        for chunk_idx in 0..total_chunks {
            rng_seed ^= rng_seed << 13;
            rng_seed ^= rng_seed >> 7;
            rng_seed ^= rng_seed << 17;

            let random_mask = rng_seed;
            forward_weights[chunk_idx] = random_mask;

            for bit_idx in 0..64 {
                let bit = (random_mask >> bit_idx) & 1;
                let shadow_idx = chunk_idx * 64 + bit_idx;

                shadow_weights[shadow_idx] = if bit == 1 { FP_ONE } else { -FP_ONE };
            }
        }

        Self {
            input_chunks,
            forward_weights,
            shadow_weights,
            thresholds: vec![((input_chunks * 64) / 2) as i32 * FP_ONE; num_neurons],
        }
    }

    pub fn forward(&self, inputs: &[u64], outputs: &mut [u64]) {
        let num_neurons: usize = 64;
        outputs.fill(0);

        for neuron_idx in 0..num_neurons {
            let mut total_set_bits = 0;
            let weight_offset = neuron_idx * self.input_chunks;

            for chunk_idx in 0..self.input_chunks {
                let w = self.forward_weights[weight_offset + chunk_idx];
                let x = inputs[chunk_idx];
                total_set_bits += (!(x ^ w)).count_ones();
            }

            let fp_set_bits = (total_set_bits as i32) << FP_SHIFT;
            let fired = (fp_set_bits > self.thresholds[neuron_idx]) as u64;

            let out_chunk = neuron_idx >> 6;
            let out_bit = neuron_idx & 63;
            outputs[out_chunk] |= fired << out_bit;
        }
    }

    pub fn backward_ste(
        &mut self,
        inputs: &[u64],
        output_errors: &[i32],
        mut prev_layer_errors: Option<&mut [i32]>,
        learning_rate_fp: i32,
    ) {
        let num_neurons: usize = 64;
        if let Some(ref mut errs) = prev_layer_errors {
            errs.fill(0);
        }

        for neuron_idx in 0..num_neurons {
            let error_fp = output_errors[neuron_idx];
            if error_fp == 0 {
                continue;
            }

            let weight_offset = neuron_idx * self.input_chunks;

            for chunk_idx in 0..self.input_chunks {
                let in_chunk = inputs[chunk_idx];
                let forward_mask = self.forward_weights[weight_offset + chunk_idx];
                let shadow_offset = (weight_offset + chunk_idx) * 64;

                let mut new_bitmask: u64 = 0;

                for bit_idx in 0..64 {
                    if let Some(ref mut prev_errs) = prev_layer_errors {
                        let weight_bit = (forward_mask >> bit_idx) & 1;
                        let weight_dir = (weight_bit as i32 * 2) - 1;

                        let propagated = error_fp.saturating_mul(weight_dir);
                        let target_idx = chunk_idx * 64 + bit_idx;
                        prev_errs[target_idx] = prev_errs[target_idx].saturating_add(propagated);
                    }

                    let in_bit = (in_chunk >> bit_idx) & 1;
                    let in_dir = (in_bit as i32 * 2) - 1;

                    let delta = ((error_fp as i64 * learning_rate_fp as i64) >> FP_SHIFT) as i32;
                    let directional_delta = delta.saturating_mul(in_dir);

                    let s_idx = shadow_offset + bit_idx;
                    let clamp_max = FP_ONE + (FP_ONE >> 1);
                    let clamp_min = -clamp_max;

                    self.shadow_weights[s_idx] = self.shadow_weights[s_idx]
                        .saturating_add(directional_delta)
                        .clamp(clamp_min, clamp_max);

                    let bit_val = (self.shadow_weights[s_idx] >> 31) + 1;

                    new_bitmask |= (bit_val as u64) << bit_idx;
                }

                self.forward_weights[weight_offset + chunk_idx] = new_bitmask;
            }
        }
    }
}
