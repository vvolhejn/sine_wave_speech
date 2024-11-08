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
}

#[wasm_bindgen]
impl SineWaveSpeechConverter {
    pub fn new(n_waves: usize, hop_size: usize) -> SineWaveSpeechConverter {
        utils::set_panic_hook();

        SineWaveSpeechConverter { n_waves, hop_size }
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

        // Limit the really extreme values. I'm not sure at what value the should be limited to
        // but this at least seemed to get rid of the really extreme values.
        // Note that in synthesize() we make sure values end up in [01, 1] so there is no clipping,
        // it's just that some values are really extreme.
        magnitudes.mapv_inplace(|x| x.min(2.0));

        let frequencies = frequencies.flatten();
        let magnitudes = magnitudes.flatten();

        let mut result = Vec::with_capacity(frequencies.len() + magnitudes.len());
        result.extend_from_slice(&frequencies.to_vec());
        result.extend_from_slice(&magnitudes.to_vec());

        result
    }

    pub fn synthesize(
        &mut self,
        frequencies: Vec<f32>,
        magnitudes: Vec<f32>,
        first_phases: Vec<f32>,
        // TODO: support multiple scales
        quantize: bool,
    ) -> Vec<f32> {
        assert_eq!(frequencies.len(), magnitudes.len());
        let n_steps: usize = frequencies.len() / self.n_waves;

        let frequencies = Array2::from_shape_vec((n_steps, self.n_waves), frequencies).unwrap();
        let magnitudes = Array2::from_shape_vec((n_steps, self.n_waves), magnitudes).unwrap();

        let scale = music::C_MAJOR_PENTATONIC.to_vec();
        let allowed_notes = if quantize { Some(&scale) } else { None };

        let (sws, last_phases) = synthesize(
            frequencies.view(),
            magnitudes.view(),
            self.hop_size,
            f32::sin,
            Some(Array::from_vec(first_phases)),
            allowed_notes,
        );

        let mut result = sws.to_vec();
        result.append(&mut last_phases.to_vec());
        result
    }
}
