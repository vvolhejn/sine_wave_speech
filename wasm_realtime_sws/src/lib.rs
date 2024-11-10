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
        music::quantize_frequencies(&frequencies, quantization_type, self.sample_rate)
    }

    pub fn quantize_frequencies_continuous(
        &mut self,
        frequencies: Vec<f32>,
        quantization_strength: f32,
    ) -> Vec<f32> {
        let quantized_versions = [
            None,
            Some(music::FrequencyQuantizationType::Chromatic),
            Some(music::FrequencyQuantizationType::Diatonic),
            Some(music::FrequencyQuantizationType::Pentatonic),
        ]
        .map(|quantization_type| {
            music::quantize_frequencies(&frequencies, quantization_type, self.sample_rate)
        });
        let max_strength = quantized_versions.len() as f32 - 1.0;

        if quantization_strength < 0.0 || quantization_strength > max_strength {
            panic!(
                "quantization_strength must be between 0 and {}, got {}",
                max_strength, quantization_strength
            );
        }

        let interpolation_weights = [
            // Unquantized version fades out quickly
            remap(quantization_strength, 0.0, 3.0, 1.0, 0.0),
            // Chromatic peaks at quantization_strength==1 and then fades
            if quantization_strength <= 1.0 {
                remap(quantization_strength, 0.0, 1.0, 0.0, 3.0)
            } else {
                remap(quantization_strength, 1.0, 2.0, 3.0, 0.0)
            },
            // Diatonic peaks at quantization_strength==2 and then fades
            if quantization_strength <= 2.0 {
                remap(quantization_strength, 1.0, 2.0, 0.0, 3.0)
            } else {
                remap(quantization_strength, 2.0, 3.0, 3.0, 0.0)
            },
            // Pentatonic comes in at quantization_strength==1 and then gets stronger
            remap(quantization_strength, 1.0, 3.0, 0.0, 3.0),
        ];

        let total_weight: f32 = interpolation_weights.iter().sum();

        let mut result = vec![0.0; frequencies.len()];
        for (i, quantized_version) in quantized_versions.iter().enumerate() {
            for (j, (result, quantized)) in
                result.iter_mut().zip(quantized_version.iter()).enumerate()
            {
                *result += quantized * (interpolation_weights[i] / total_weight);
            }
        }

        result
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

fn remap(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    // For our purposes, it's useful to silently return 0.0 if the value is outside the range
    if value < from_min || value > from_max {
        return 0.0;
    }
    (value - from_min) / (from_max - from_min) * (to_max - to_min) + to_min
}
