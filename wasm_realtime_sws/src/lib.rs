use ndarray::Array;
use pitch_detection::{McLeodDetector, PitchDetector};
use wasm_bindgen::prelude::*;

mod lpc;
mod signal_processing;
mod utils;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct WasmPitchDetector {
    sample_rate: usize,
    fft_size: usize,
    detector: McLeodDetector<f32>,
}

#[wasm_bindgen]
impl WasmPitchDetector {
    pub fn new(sample_rate: usize, fft_size: usize) -> WasmPitchDetector {
        utils::set_panic_hook();

        let fft_pad = fft_size / 2;

        WasmPitchDetector {
            sample_rate,
            fft_size,
            detector: McLeodDetector::<f32>::new(fft_size, fft_pad),
        }
    }

    pub fn detect_pitch(&mut self, audio_samples: Vec<f32>) -> f32 {
        if audio_samples.len() < self.fft_size {
            panic!(
                "Insufficient samples passed to detect_pitch(). \
                Expected an array containing {} elements but got {}",
                self.fft_size,
                audio_samples.len(),
            );
        }

        // Include only notes that exceed a power threshold which relates to the
        // amplitude of frequencies in the signal. Use the suggested default
        // value of 5.0 from the library.
        const POWER_THRESHOLD: f32 = 5.0;

        console_log!("POWER_THRESHOLD: {}", POWER_THRESHOLD);

        const HOP_SIZE: usize = 128;
        let x = Array::from_vec(audio_samples.clone());
        let (lpc_coefficients, gain, residual) = lpc::fit_lpc(&x, self.fft_size, HOP_SIZE, None);
        console_log!("lpc_coefficients: {:?}", lpc_coefficients);
        console_log!("gain: {:?}", gain);
        console_log!("residual: {:?}", residual);

        // The clarity measure describes how coherent the sound of a note is. For
        // example, the background sound in a crowded room would typically be would
        // have low clarity and a ringing tuning fork would have high clarity.
        // This threshold is used to accept detect notes that are clear enough
        // (valid values are in the range 0-1).
        const CLARITY_THRESHOLD: f32 = 0.6;

        let optional_pitch = self.detector.get_pitch(
            &audio_samples,
            self.sample_rate,
            POWER_THRESHOLD,
            CLARITY_THRESHOLD,
        );

        match optional_pitch {
            Some(pitch) => pitch.frequency,
            None => 0.0,
        }
    }
}
