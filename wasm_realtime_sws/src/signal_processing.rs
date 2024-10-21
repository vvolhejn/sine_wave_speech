use approx::{assert_abs_diff_eq, AbsDiffEq};
use ndarray::{Array1, ArrayView1};

pub fn lfilter(coeffs: &Array1<f32>, signal: &Array1<f32>) -> Array1<f32> {
    let n = signal.len();
    let m = coeffs.len();
    let mut output = Array1::zeros(n);

    for i in 0..n {
        let mut sum = 0.0;
        for j in 0..m {
            if i >= j {
                sum += coeffs[j] * signal[i - j];
            }
        }
        output[i] = sum;
    }

    output
}

pub fn autocorrelate(signal: ArrayView1<f32>) -> Array1<f32> {
    let n = signal.len();
    let mut result = Array1::zeros(n);

    for lag in 0..n {
        let mut sum = 0.0;
        for i in 0..n - lag {
            sum += signal[i] * signal[i + lag];
        }
        result[lag] = sum;
    }

    result
}

pub fn hann_window(size: usize) -> Array1<f32> {
    Array1::from_iter((0..size).map(|n| {
        let x = 2.0 * std::f32::consts::PI * n as f32 / size as f32;
        0.5 * (1.0 - x.cos())
    }))
}

fn assert_array_eq(actual: &Array1<f32>, expected: &Array1<f32>, epsilon: f32) {
    assert_eq!(
        actual.len(),
        expected.len(),
        "Arrays have different lengths. \nActual: {:?}\nExpected: {:?}",
        actual,
        expected
    );

    for (i, (a, e)) in actual.iter().zip(expected.iter()).enumerate() {
        if !a.abs_diff_eq(e, epsilon) {
            panic!(
                "Assertion failed at index {}.\nActual array: {:?}\nExpected array: {:?}\nDifference at index {}: {} vs {}",
                i, actual, expected, i, a, e
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_autocorrelate() {
        let signal = array![1.0, 2.0, 3.0, 4.0, 5.0];
        let autocorrelation = autocorrelate(signal.view());

        // Expected results (calculated manually)
        let expected = array![55.0, 40.0, 26.0, 14.0, 5.0];

        assert_array_eq(&autocorrelation, &expected, 1e-6);
    }

    #[test]
    fn test_lfilter() {
        let coeffs = Array1::from_vec(vec![1.0, 2.0]);
        let signal = Array1::from_vec(vec![1.0, 0.0, 0.0, 2.0, 1.0, 0.0]);
        let expected = Array1::from_vec(vec![1.0, 2.0, 0.0, 2.0, 5.0, 2.0]);

        let result = lfilter(&coeffs, &signal);

        assert_array_eq(&result, &expected, 1e-6);
    }

    #[test]
    fn test_hann_window() {
        let window = hann_window(4);
        let expected = array![0.0, 0.5, 1.0, 0.5];

        assert_array_eq(&window, &expected, 1e-6);

        let window = hann_window(16);
        // Values from scipy.signal.get_window("hann", 16)
        let expected = array![
            0., 0.03806023, 0.14644661, 0.30865828, 0.5, 0.69134172, 0.85355339, 0.96193977, 1.,
            0.96193977, 0.85355339, 0.69134172, 0.5, 0.30865828, 0.14644661, 0.03806023
        ];

        assert_array_eq(&window, &expected, 1e-6);
    }
}
