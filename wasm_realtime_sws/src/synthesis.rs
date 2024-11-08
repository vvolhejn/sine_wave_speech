use ndarray::{s, Array1, ArrayView2, Axis};

use crate::music;

/// Synthesizes a signal from sine wave frequencies and magnitudes.
///
/// We don't need to know the sample rate because the frequencies are in radians/sample.
pub fn synthesize(
    normalized_frequencies: ArrayView2<f32>,
    magnitudes: ArrayView2<f32>,
    hop_size: usize,
    wave_fn: impl Fn(f32) -> f32,
    first_phases: Option<Array1<f32>>,
    allowed_notes: Option<&Vec<music::NoteName>>,
) -> (Array1<f32>, Array1<f32>) {
    assert_eq!(normalized_frequencies.shape(), magnitudes.shape());
    if normalized_frequencies.len_of(Axis(0)) < 2 {
        panic!(
            "At least two frames are required, but normalized_frequencies has length {}",
            normalized_frequencies.len_of(Axis(0))
        );
    }

    let first_phases =
        first_phases.unwrap_or_else(|| Array1::zeros(normalized_frequencies.dim().1));

    let (n_frames, n_waves) = normalized_frequencies.dim();
    let output_samples = (n_frames - 1) * hop_size;
    let mut output = Array1::zeros(output_samples);
    let mut last_phases = Array1::zeros(n_waves);

    const MIN_OCTAVE: i32 = 0;
    const MAX_OCTAVE: i32 = 8;
    const FREQUENCY_MULTIPLIER: f32 = (2. * std::f32::consts::PI) / 8000.0;
    let allowed_frequencies = allowed_notes
        .map(|notes| music::generate_scale(notes, MIN_OCTAVE, MAX_OCTAVE, FREQUENCY_MULTIPLIER));

    for i in 0..n_waves {
        let freq_slice = normalized_frequencies.slice(s![.., i]);
        let mag_slice = magnitudes.slice(s![.., i]);

        let cur = synthesize_one(
            &freq_slice.to_owned(),
            &mag_slice.to_owned(),
            hop_size,
            &wave_fn,
            first_phases[i],
            allowed_frequencies.as_ref(),
        );
        output += &cur.0;
        last_phases[i] = cur.1;
    }

    // Apply compression to avoid clipping
    output.mapv_inplace(|x| x.atan());

    (output, last_phases)
}

/// Synthesize one wave from normalized frequencies and magnitudes.
pub fn synthesize_one(
    normalized_frequencies: &Array1<f32>,
    magnitudes: &Array1<f32>,
    hop_size: usize,
    wave_fn: impl Fn(f32) -> f32,
    first_phase: f32,
    allowed_frequencies: Option<&Vec<f32>>,
) -> (Array1<f32>, f32) {
    assert_eq!(normalized_frequencies.shape(), magnitudes.shape());
    assert_eq!(normalized_frequencies.ndim(), 1);

    let frequencies_upsampled = upsample(
        normalized_frequencies,
        hop_size,
        false,
        UpsamplingMethod::Nearest,
    );
    let magnitudes_upsampled = upsample(magnitudes, hop_size, false, UpsamplingMethod::Linear);

    let frequencies_upsampled = match allowed_frequencies {
        Some(allowed_frequencies) => frequencies_upsampled
            .mapv(|x| music::snap_frequency(x, allowed_frequencies))
            .to_owned(),
        None => frequencies_upsampled.to_owned(),
    };

    // Calculate cumulative sum for phase
    let mut phase = Array1::zeros(frequencies_upsampled.len());
    let mut sum = first_phase;
    for (i, &freq) in frequencies_upsampled.iter().enumerate() {
        sum += freq;
        phase[i] = sum;
    }

    (
        // Apply wave function and scale by magnitudes
        phase.mapv(|x| wave_fn(x)) * &magnitudes_upsampled,
        phase[phase.len() - 1] % (2.0 * std::f32::consts::PI),
    )
}

pub enum UpsamplingMethod {
    Linear,
    Nearest,
}

/// Upsamples a signal, stretching it by an integer factor.
/// Linearly interpolates between the original samples.
/// Returns an array of size (x.len() - 1) * factor,
/// or one more if include_last is true.
/// For example, if you have [a, b, c] and factor = 3, you end
/// up with axxbxxc, where x represent interpolated values.
/// If include_last is false, you get axxbxx.
pub fn upsample(
    x: &Array1<f32>,
    factor: usize,
    include_last: bool,
    method: UpsamplingMethod,
) -> Array1<f32> {
    // Replace NaN values with 0.0
    let x = x.mapv(|v| if v.is_nan() { 0.0 } else { v });

    let output_size = (x.len() - 1) * factor + (if include_last { 1 } else { 0 });
    let mut output = Array1::zeros(output_size);

    for i in 0..output_size {
        let global_fraction = i as f32 / factor as f32;
        let i_low = global_fraction.floor() as usize;
        let i_high = (i_low + 1).min(x.len() - 1);
        let local_fraction = global_fraction - i_low as f32;

        output[i] = match method {
            UpsamplingMethod::Linear => {
                x[i_low].mul_add(1.0 - local_fraction, x[i_high] * local_fraction)
            }
            UpsamplingMethod::Nearest => x[i_low],
        };
    }

    output
}

#[cfg(test)]
mod tests {
    use crate::signal_processing::tests::assert_array1_eq;

    use super::*;

    #[test]
    fn test_upsample() {
        let input = Array1::from_vec(vec![0.0, 1.0, 2.0]);
        let upsampled = upsample(&input, 2, true, UpsamplingMethod::Linear);
        let expected = Array1::from_vec(vec![0.0, 0.5, 1.0, 1.5, 2.0]);
        assert_array1_eq(&upsampled, &expected, 1e-6);

        let upsampled = upsample(&input, 2, false, UpsamplingMethod::Linear);
        let expected = Array1::from_vec(vec![0.0, 0.5, 1.0, 1.5]);
        assert_array1_eq(&upsampled, &expected, 1e-6);

        let upsampled = upsample(&input, 2, true, UpsamplingMethod::Nearest);
        let expected = Array1::from_vec(vec![0.0, 0.0, 1.0, 1.0, 2.0]);
        assert_array1_eq(&upsampled, &expected, 1e-6);
    }
}
