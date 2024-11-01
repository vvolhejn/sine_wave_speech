use ndarray::{s, Array, Array2};
use synthesis::synthesize;
use wasm_bindgen::prelude::*;
use web_sys::js_sys;

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

        let sws = synthesize(
            frequencies.view(),
            magnitudes.view(),
            self.hop_size,
            f32::sin,
        );

        // console_log!("lpc_coefficients: {:?}", lpc_coefficients);
        // console_log!("gain: {:?}", gain);
        // console_log!("residual: {:?}", residual);
        // console_log!("frequencies: {:?}", frequencies);
        // console_log!("magnitudes: {:?}", magnitudes);

        sws.to_vec()
    }
}
