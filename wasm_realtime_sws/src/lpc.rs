use ndarray::{concatenate, prelude::*, stack};
use rustfft::{num_complex::Complex, FftPlanner};
use std::f32::consts::PI;

use crate::signal_processing::{autocorrelate, hann_window, lfilter};

// Original Python: def fit_lpc(audio: np.ndarray, p=12, hop_size=DEFAULT_HOP_SIZE, window_size=None):
pub fn fit_lpc(
    audio: &Array1<f32>,
    p: usize,
    hop_size: usize,
    window_size: Option<usize>,
) -> (Array2<f32>, Array1<f32>, Array1<f32>) {
    let window_size = window_size.unwrap_or(2 * hop_size);
    let n_hops = audio.len() / hop_size;

    // Original Python:
    // audio = np.concatenate([
    //     np.zeros((window_size - hop_size) // 2),
    //     audio,
    //     np.zeros((window_size - hop_size) // 2),
    // ])
    let pad_size = (window_size - hop_size) / 2;
    let audio = concatenate![
        Axis(0),
        Array1::zeros(pad_size),
        audio.to_owned(),
        Array1::zeros(pad_size)
    ];

    let mut lpc_coefficients = Array2::zeros((n_hops, p + 1));
    let mut gain = Array1::zeros(n_hops);
    let mut residual = Array1::zeros((n_hops - 1) * hop_size + window_size);

    // Original Python: audio = scipy.signal.lfilter(np.array([1.0, -0.9]), 1, audio)
    let audio = lfilter(&array![1.0, -0.9], &audio);

    for hop in 0..n_hops {
        let cur_audio = audio.slice(s![hop * hop_size..hop * hop_size + window_size]);
        let windowed_audio = &cur_audio * &hann_window(window_size);

        // Original Python: autocorrelated = scipy.signal.correlate(windowed_audio, windowed_audio)
        let autocorrelated = autocorrelate(windowed_audio.view());
        let autocorrelated = autocorrelated.slice(s![window_size - 1..window_size + p]);

        // Original Python:
        // try:
        //     cur_lpc_coefficients = scipy.linalg.solve_toeplitz(
        //         autocorrelated[:p], autocorrelated[1 : p + 1]
        //     )
        // except scipy.linalg.LinAlgError:  # "Singular principal minor"
        //     print("Singular principal minor")
        //     continue
        let cur_lpc_coefficients = match solve_toeplitz(
            &autocorrelated.slice(s![..p]),
            &autocorrelated.slice(s![1..p + 1]),
        ) {
            Ok(coeffs) => coeffs,
            Err(_) => {
                println!("Singular principal minor");
                continue;
            }
        };

        let cur_lpc_coefficients = concatenate![Axis(0), arr1(&[1.0]), -cur_lpc_coefficients];
        let cur_residual = lfilter(&cur_lpc_coefficients, &windowed_audio);
        let cur_gain = (cur_residual.mapv(|x| x.powi(2)).mean().unwrap()).sqrt();

        lpc_coefficients
            .slice_mut(s![hop, ..])
            .assign(&cur_lpc_coefficients);
        gain[hop] = cur_gain;
        // residual
        //     .slice_mut(s![hop * hop_size..hop * hop_size + window_size])
        //     .add_assign(&(&cur_residual / cur_gain));
        let mut slice = residual.slice_mut(s![hop * hop_size..hop * hop_size + window_size]);
        slice.zip_mut_with(&cur_residual, |a, &b| *a += b / cur_gain);
    }

    let residual = residual
        .slice(s![(window_size - hop_size) / 2..])
        .to_owned();

    (lpc_coefficients, gain, residual)
}

fn solve_toeplitz(
    r: &ArrayView1<f32>,
    b: &ArrayView1<f32>,
) -> Result<Array1<f32>, Box<dyn std::error::Error>> {
    // Implement Toeplitz matrix solver
    unimplemented!()
}
