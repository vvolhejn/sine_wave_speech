use ndarray::{Array, Array2};
use synthesis::synthesize;
use wasm_bindgen::prelude::*;

mod linear_algebra;
mod lpc;
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

    pub fn convert(&mut self, audio_samples: Vec<f32>) -> Vec<f32> {
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
        let (frequencies, mut magnitudes) =
            lpc::lpc_coefficients_to_frequencies(lpc_coefficients.view(), gain.view());

        // Limit the really extreme values. I'm not sure at what value the should be limited to
        // but this at least seemed to get rid of the really extreme values.
        // Note that in synthesize() we normalize the output to [-1, 1] so there is no clipping,
        // it's just that some values are really extreme.
        magnitudes.mapv_inplace(|x| x.min(2.0));

        let sws = synthesize(frequencies.view(), magnitudes.view(), None, f32::cos);

        // console_log!("lpc_coefficients: {:?}", lpc_coefficients);
        // console_log!("gain: {:?}", gain);
        // console_log!("residual: {:?}", residual);
        // console_log!("frequencies: {:?}", frequencies);
        // console_log!("magnitudes: {:?}", magnitudes);

        sws.to_vec()
    }
}
