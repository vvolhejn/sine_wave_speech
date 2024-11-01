use ndarray::{s, Array1, Array2, ArrayView2, Axis};
use std::f32::consts::PI;

use crate::lpc::{fit_lpc, lpc_coefficients_to_frequencies};

const DEFAULT_HOP_SIZE: usize = 256;

/// Synthesizes a signal from sine wave frequencies and magnitudes.
///
/// We don't need to know the sample rate because the frequencies are in radians/sample.
pub fn synthesize(
    normalized_frequencies: ArrayView2<f32>,
    magnitudes: ArrayView2<f32>,
    hop_size: Option<usize>,
    wave_fn: impl Fn(f32) -> f32,
) -> Array1<f32> {
    assert_eq!(normalized_frequencies.shape(), magnitudes.shape());
    assert_eq!(normalized_frequencies.ndim(), 2);

    let hop_size = hop_size.unwrap_or(DEFAULT_HOP_SIZE);
    let (n_frames, n_waves) = normalized_frequencies.dim();
    let output_samples = 1 + (n_frames - 1) * hop_size;
    let mut output = Array1::zeros(output_samples);

    for i in 0..n_waves {
        let freq_slice = normalized_frequencies.slice(s![.., i]);
        let mag_slice = magnitudes.slice(s![.., i]);

        let cur = synthesize_one(
            &freq_slice.to_owned(),
            &mag_slice.to_owned(),
            hop_size,
            &wave_fn,
        );
        output += &cur;
    }

    // Normalize output to [-1, 1]
    let max_abs = output.fold(0.0f32, |acc, &x| acc.max(x.abs()));
    if max_abs > 0.0 {
        output.mapv_inplace(|x| x / max_abs);
    }

    output
}

/// Synthesize one wave from normalized frequencies and magnitudes.
pub fn synthesize_one(
    normalized_frequencies: &Array1<f32>,
    magnitudes: &Array1<f32>,
    hop_size: usize,
    wave_fn: impl Fn(f32) -> f32,
) -> Array1<f32> {
    assert_eq!(normalized_frequencies.shape(), magnitudes.shape());
    assert_eq!(normalized_frequencies.ndim(), 1);

    let frequencies_upsampled = upsample(normalized_frequencies, hop_size);
    let magnitudes_upsampled = upsample(magnitudes, hop_size);

    // Calculate cumulative sum for phase
    let mut phase = Array1::zeros(frequencies_upsampled.len());
    let mut sum = 0.0f32;
    for (i, &freq) in frequencies_upsampled.iter().enumerate() {
        sum += freq;
        phase[i] = sum;
    }

    // Apply wave function and magnitudes
    phase.mapv(|x| wave_fn(x)) * &magnitudes_upsampled
}

/// Upsamples a signal, stretching it by an integer factor.
pub fn upsample(x: &Array1<f32>, factor: usize) -> Array1<f32> {
    // Replace NaN values with 0.0
    let x = x.mapv(|v| if v.is_nan() { 0.0 } else { v });

    let output_size = (x.len() - 1) * factor + 1;
    let mut output = Array1::zeros(output_size);

    // Pre-calculate factor as f32 to avoid repeated conversions
    let factor_f = factor as f32;

    for i in 0..output_size {
        let idx_f = i as f32 / factor_f;
        let idx_low = idx_f.floor() as usize;
        let idx_high = (idx_low + 1).min(x.len() - 1);
        let frac = idx_f - idx_low as f32;

        output[i] = x[idx_low].mul_add(1.0 - frac, x[idx_high] * frac);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_upsample() {
        let input = Array1::from_vec(vec![0.0, 1.0, 2.0]);
        let upsampled = upsample(&input, 2);
        let expected = Array1::from_vec(vec![0.0, 0.5, 1.0, 1.5, 2.0]);

        for (a, b) in upsampled.iter().zip(expected.iter()) {
            assert_relative_eq!(a, b, epsilon = 1e-5);
        }
    }
}
