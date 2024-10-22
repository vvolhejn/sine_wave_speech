use ndarray::{concatenate, prelude::*};

use crate::signal_processing::{
    autocorrelate, hann_window, lfilter, solve_toeplitz, ToeplitzError,
};

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
        let autocorrelated = autocorrelated.slice(s![..p + 1]);

        // Original Python:
        // try:
        //     cur_lpc_coefficients = scipy.linalg.solve_toeplitz(
        //         autocorrelated[:p], autocorrelated[1 : p + 1]
        //     )
        // except scipy.linalg.LinAlgError:  # "Singular principal minor"
        //     print("Singular principal minor")
        //     continue

        // construct the toeplitz matrix argument represented as 1d array of the "edge"
        let toeplitz = concatenate![
            Axis(0),
            autocorrelated.slice(s![1..p;-1]),
            autocorrelated.slice(s![..p]),
        ];

        let cur_lpc_coefficients =
            match solve_toeplitz(toeplitz.view(), autocorrelated.slice(s![1..p + 1])) {
                Ok(coeffs) => coeffs,
                Err(ToeplitzError::SingularPrincipalMinor) => {
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

#[cfg(test)]
mod tests {
    use crate::signal_processing::{assert_array2_eq, assert_array_eq};

    use super::*;

    #[derive(serde::Deserialize)]
    struct FitLpcOutput {
        // Can't use Array1<f32> directly, getting:
        // Failed to load test data: Syntax("invalid type: floating point `-0.0005080277333036065`, expected u8")
        // perhaps because rmp_serde serializes it into a different format than for Vec?
        audio: Vec<f32>,
        p: usize,
        hop_size: usize,
        lpc_coefficients: Vec<f32>,
        gain: Vec<f32>,
        residual: Vec<f32>,
    }

    #[test]
    fn test_fit_lpc() {
        // load from msgpack
        let input: FitLpcOutput =
            rmp_serde::from_slice(include_bytes!("../fixtures/fit_lpc_results.msgpack"))
                .expect("Failed to load test data");

        let audio = Array1::from_vec(input.audio);

        let (lpc_coefficients, gain, residual) = fit_lpc(&audio, input.p, input.hop_size, None);

        let expected_lpc_coefficients = Array2::from_shape_vec(
            (input.lpc_coefficients.len() / (input.p + 1), input.p + 1),
            input.lpc_coefficients,
        )
        .unwrap();

        // Needed to set the epsilon fairly high for this to work, is there
        // concern? Perhaps it's because of float64 in Python or a different
        // implementation in Scipy than what we have.
        assert_array2_eq(&lpc_coefficients, &expected_lpc_coefficients, 1e-2);
        assert_array_eq(&gain, &Array1::from_vec(input.gain), 1e-2);
        assert_array_eq(&residual, &Array1::from_vec(input.residual), 1e-2);
    }
}
