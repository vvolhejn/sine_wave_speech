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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use ndarray::array;

    #[test]
    fn test_autocorrelate() {
        let signal = array![1.0, 2.0, 3.0, 4.0, 5.0];
        let autocorrelation = autocorrelate(signal.view());

        // Expected results (calculated manually)
        let expected = array![55.0, 40.0, 26.0, 14.0, 5.0];

        assert_eq!(autocorrelation.len(), signal.len());

        for (a, e) in autocorrelation.iter().zip(expected.iter()) {
            assert_abs_diff_eq!(a, e, epsilon = 1e-6);
        }
    }

    #[test]
    fn test_lfilter() {
        let coeffs = Array1::from_vec(vec![1.0, 2.0]);
        let signal = Array1::from_vec(vec![1.0, 0.0, 0.0, 2.0, 1.0, 0.0]);
        let expected = Array1::from_vec(vec![1.0, 2.0, 0.0, 2.0, 5.0, 2.0]);

        let result = lfilter(&coeffs, &signal);

        assert_eq!(result.len(), expected.len());
        for (a, b) in result.iter().zip(expected.iter()) {
            assert_abs_diff_eq!(a, b, epsilon = 1e-6);
        }
    }
}
