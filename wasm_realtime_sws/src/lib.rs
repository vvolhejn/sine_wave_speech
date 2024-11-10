use ndarray::{Array, Array2};
use synthesis::synthesize;
use wasm_bindgen::prelude::*;

mod linear_algebra;
mod lpc;
mod music;
mod signal_processing;
mod synthesis;
mod utils;

/// Note that the converter doesn't care about the sample rate,
#[wasm_bindgen]
pub struct SineWaveSpeechConverter {
    pub n_waves: usize,  // 4 is the default in Python
    pub hop_size: usize, // 256 is the default in Python
    pub sample_rate: usize,
}

#[wasm_bindgen]
impl SineWaveSpeechConverter {
    pub fn new(n_waves: usize, hop_size: usize, sample_rate: usize) -> SineWaveSpeechConverter {
        utils::set_panic_hook();

        SineWaveSpeechConverter {
            n_waves,
            hop_size,
            sample_rate,
        }
    }

    pub fn get_frequencies_and_magnitudes(&mut self, audio_samples: Vec<f32>) -> Vec<f32> {
        let (lpc_coefficients, gain, _residual) = lpc::fit_lpc(
            &Array::from_vec(audio_samples),
            self.n_waves * 2,
            self.hop_size,
            None,
        );
        let (frequencies, mut magnitudes) =
            lpc::lpc_coefficients_to_frequencies(lpc_coefficients.view(), gain.view());

        // Normalize magnitudes by the number of waves because otherwise the total magnitude
        // increases with the number of waves. Not sure if this is the proper way to do it
        // but it works.
        // Note we also apply compression to the final waveform in synthesize().
        magnitudes.mapv_inplace(|x| x / self.n_waves as f32);

        let frequencies = frequencies.flatten();
        let magnitudes = magnitudes.flatten();

        let mut result = Vec::with_capacity(frequencies.len() + magnitudes.len());
        result.extend_from_slice(&frequencies.to_vec());
        result.extend_from_slice(&magnitudes.to_vec());

        result
    }

    pub fn quantize_frequencies(
        &mut self,
        frequencies: Vec<f32>,
        quantization_type: Option<music::FrequencyQuantizationType>,
    ) -> Vec<f32> {
        match quantization_type {
            None => return frequencies,
            Some(quantization_type) => {
                let allowed_notes = quantization_type.to_scale();

                const MIN_OCTAVE: i32 = 0;
                const MAX_OCTAVE: i32 = 8;
                let frequency_multiplier: f32 =
                    (2. * std::f32::consts::PI) / self.sample_rate as f32;
                let allowed_frequencies = music::generate_scale(
                    &allowed_notes.to_vec(),
                    MIN_OCTAVE,
                    MAX_OCTAVE,
                    Some(frequency_multiplier),
                );

                let quantized_frequencies = frequencies
                    .iter()
                    .map(|x| music::quantize_frequency(*x, &allowed_frequencies))
                    .collect();
                quantized_frequencies
            }
        }
    }

    pub fn synthesize(
        &mut self,
        frequencies: Vec<f32>,
        magnitudes: Vec<f32>,
        first_phases: Vec<f32>,
    ) -> Vec<f32> {
        assert_eq!(frequencies.len(), magnitudes.len());
        let n_steps: usize = frequencies.len() / self.n_waves;

        let frequencies = Array2::from_shape_vec((n_steps, self.n_waves), frequencies).unwrap();
        let magnitudes = Array2::from_shape_vec((n_steps, self.n_waves), magnitudes).unwrap();

        let (sws, last_phases) = synthesize(
            frequencies.view(),
            magnitudes.view(),
            self.hop_size,
            f32::sin,
            Some(Array::from_vec(first_phases)),
        );

        let mut result = sws.to_vec();
        result.append(&mut last_phases.to_vec());
        result
    }
}
