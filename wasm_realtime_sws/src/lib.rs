use ndarray::{Array, Array2};
use wasm_bindgen::prelude::*;

mod linear_algebra;
mod lpc;
mod signal_processing;
mod utils;

/// Note that the converter doesn't care about the sample rate,
#[wasm_bindgen]
pub struct SineWaveSpeechConverter {
    pub n_waves: usize,  // 4 is the default in Python
    pub hop_size: usize, // 256 is the default in Python
}

#[wasm_bindgen]
pub struct SineWaveStep {
    /// Frequency expressed as radians/sample
    pub normalized_frequency: f32,
    pub magnitude: f32,
}

#[wasm_bindgen]
impl SineWaveSpeechConverter {
    pub fn new(n_waves: usize, hop_size: usize) -> SineWaveSpeechConverter {
        utils::set_panic_hook();

        SineWaveSpeechConverter { n_waves, hop_size }
    }

    pub fn convert(&mut self, audio_samples: Vec<f32>) -> Vec<SineWaveStep> {
        if audio_samples.len() < self.hop_size {
            panic!(
                "Insufficient samples passed to detect_pitch(). \
                Expected an array containing {} elements but got {}",
                self.hop_size,
                audio_samples.len(),
            );
        }

        let (lpc_coefficients, gain, _residual) = lpc::fit_lpc(
            &Array::from_vec(audio_samples),
            self.n_waves * 2,
            self.hop_size,
            None,
        );
        let (frequencies, magnitudes) =
            lpc::lpc_coefficients_to_frequencies(lpc_coefficients.view(), gain.view());

        // console_log!("lpc_coefficients: {:?}", lpc_coefficients);
        // console_log!("gain: {:?}", gain);
        // console_log!("residual: {:?}", residual);
        // console_log!("frequencies: {:?}", frequencies);
        // console_log!("magnitudes: {:?}", magnitudes);

        parse_results(frequencies, magnitudes, self.n_waves)
    }
}

/// Return a flat vector of SineWaveStep structs from the 2D arrays of frequencies and magnitudes.
/// The frequency and magnitude of the j-th sine wave at the i-th time step
/// will be at index i * n_waves + j in the flat vector.
/// I didn't figure out how to return a 2D vector to JS, so I'm returning a flat one.
fn parse_results(
    frequencies: Array2<f32>,
    magnitudes: Array2<f32>,
    n_waves: usize,
) -> Vec<SineWaveStep> {
    let mut results = Vec::new();

    for i in 0..frequencies.shape()[0] {
        for j in 0..n_waves {
            let normalized_frequency = frequencies[[i, j]];
            let magnitude = magnitudes[[i, j]];

            results.push(SineWaveStep {
                normalized_frequency,
                magnitude,
            });
        }
    }

    results
}
