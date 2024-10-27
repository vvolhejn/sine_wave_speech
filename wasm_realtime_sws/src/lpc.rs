use nalgebra::{Complex, ComplexField};
use ndarray::{concatenate, prelude::*};

use crate::{
    linear_algebra::find_roots,
    signal_processing::{autocorrelate, hann_window, lfilter, solve_toeplitz, ToeplitzError},
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

pub fn lpc_coefficients_to_frequencies(
    lpc_coefficients: ArrayView2<f32>,
    gain: ArrayView1<f32>,
) -> (Array2<f32>, Array2<f32>) {
    let n_hops = lpc_coefficients.len_of(Axis(0));
    let p = lpc_coefficients.len_of(Axis(1)) - 1;

    let mut frequencies: Array2<f32> = Array::zeros((n_hops, p / 2));
    let mut magnitudes: Array2<f32> = Array::zeros((n_hops, p / 2));

    for hop in 0..n_hops {
        // Note that we reverse the slice because our find_roots() function expects
        // the coefficients in the opposite order than np.roots
        let cur_lpc_coefficients = lpc_coefficients.slice(s![hop, 1..p + 1;-1]);
        let roots = find_roots(cur_lpc_coefficients);

        fn get_frequency(x: Complex<f32>) -> f32 {
            // The frequencies are the angles of the complex roots
            let frequency = x.to_polar().1;
            // frequencies of 0 and pi correspond to real-only roots and seem to be useless.
            // (I don't understand the math behind this, but it seems to work).
            // The pi frequency in radians/sample corresponds to sr/2 Hz.
            // Setting it to -pi instead of pi makes us filter the root out later.
            if frequency >= std::f32::consts::PI - 1e-3 {
                -frequency
            } else {
                frequency
            }
        }

        let mut frequencies_and_magnitudes: Vec<(f32, f32)> = roots
            .to_vec()
            .into_iter()
            .map(|x| (get_frequency(x), gain[hop] / (1. - x.abs())))
            // Each frequency is repeated twice, once with a positive angle and once
            // with a negative angle. We only want to keep the positive angle.
            // Also filter out frequencies of 0 and pi (see get_frequency()).
            .filter(|(f, _m)| *f >= 1e-3)
            .collect();

        if frequencies_and_magnitudes.len() > p / 2 {
            panic!(
                "expected at most {:?} positive frequencies, got {:?}: {:?}",
                p / 2,
                frequencies_and_magnitudes.len(),
                frequencies_and_magnitudes
            );
        }

        // Sort the frequencies so that the sine waves don't cross when we upsample.
        frequencies_and_magnitudes.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        for (i, (f, m)) in frequencies_and_magnitudes.iter().enumerate() {
            frequencies[[hop, i]] = *f;
            magnitudes[[hop, i]] = *m;
        }
    }

    (frequencies, magnitudes)
}

#[cfg(test)]
mod tests {
    use crate::signal_processing::tests::{assert_array1_eq, assert_array2_eq};

    use super::*;

    #[derive(serde::Deserialize)]
    struct PythonSwsOutput {
        // Can't use Array1<f32> directly, getting:
        // Failed to load test data: Syntax("invalid type: floating point `-0.0005080277333036065`, expected u8")
        // perhaps because rmp_serde serializes it into a different format than for Vec?
        audio: Vec<f32>,
        n_waves: usize,
        hop_size: usize,
        lpc_coefficients: Vec<f32>, // flattened from 2D
        gain: Vec<f32>,
        residual: Vec<f32>,
        frequencies: Vec<f32>, // flattened from 2D
        magnitudes: Vec<f32>,  // flattened from 2D
    }

    #[test]
    fn test_fit_lpc() {
        // load from msgpack
        let input: PythonSwsOutput =
            rmp_serde::from_slice(include_bytes!("../fixtures/python_sws_results.msgpack"))
                .expect("Failed to load test data");

        let audio = Array1::from_vec(input.audio);
        let n_steps = input.lpc_coefficients.len() / (input.n_waves * 2 + 1);
        let expected_lpc_coefficients =
            Array2::from_shape_vec((n_steps, input.n_waves * 2 + 1), input.lpc_coefficients)
                .unwrap();
        let expected_frequencies =
            Array2::from_shape_vec((n_steps, input.n_waves), input.frequencies).unwrap();
        let expected_magnitudes =
            Array2::from_shape_vec((n_steps, input.n_waves), input.magnitudes).unwrap();

        let (lpc_coefficients, gain, residual) =
            fit_lpc(&audio, input.n_waves * 2, input.hop_size, None);

        // Needed to set the epsilon fairly high for this to work, is there
        // concern? Perhaps it's because of float64 in Python or a different
        // implementation in Scipy than what we have.
        assert_array2_eq(&lpc_coefficients, &expected_lpc_coefficients, 1e-2);
        assert_array1_eq(&gain, &Array1::from_vec(input.gain), 1e-2);
        assert_array1_eq(&residual, &Array1::from_vec(input.residual), 1e-2);

        let (frequencies, magnitudes) =
            lpc_coefficients_to_frequencies(lpc_coefficients.view(), gain.view());

        assert_array2_eq(&frequencies, &expected_frequencies, 1e-2);
        assert_array2_eq(&magnitudes, &expected_magnitudes, 1e-2);
    }
}
