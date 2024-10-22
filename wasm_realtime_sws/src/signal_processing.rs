use approx::{assert_abs_diff_eq, AbsDiffEq};
use ndarray::{Array1, Array2, ArrayView1};
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum ToeplitzError {
    #[error("Singular principal minor")]
    SingularPrincipalMinor,
}

/// AI reimplementation of the Cython version from SciPy:
/// https://github.com/scipy/scipy/blob/92d2a8592782ee19a1161d0bf3fc2241ba78bb63/scipy/linalg/_solve_toeplitz.pyx#L14
pub fn solve_toeplitz(
    a: ArrayView1<f32>,
    b: ArrayView1<f32>,
) -> Result<Array1<f32>, ToeplitzError> {
    let n = b.len();
    assert_eq!(a.len(), 2 * n - 1, "Input 'a' must have length 2n-1");

    let mut x = Array1::zeros(n);
    let mut g = Array1::zeros(n);
    let mut h = Array1::zeros(n);

    if a[n - 1] == 0.0 {
        return Err(ToeplitzError::SingularPrincipalMinor);
    }

    x[0] = b[0] / a[n - 1];

    if n == 1 {
        return Ok(x);
    }

    g[0] = a[n - 2] / a[n - 1];
    h[0] = a[n] / a[n - 1];

    for m in 1..n {
        let mut x_num = -b[m];
        let mut x_den = -a[n - 1];
        for j in 0..m {
            let nmj = n + m - (j + 1);
            x_num += a[nmj] * x[j];
            x_den += a[nmj] * g[m - j - 1];
        }
        if x_den == 0.0 {
            return Err(ToeplitzError::SingularPrincipalMinor);
        }
        x[m] = x_num / x_den;

        for j in 0..m {
            x[j] -= x[m] * g[m - j - 1];
        }
        if m == n - 1 {
            return Ok(x);
        }

        let mut g_num = -a[n - m - 2];
        let mut h_num = -a[n + m];
        let mut g_den = -a[n - 1];
        for j in 0..m {
            g_num += a[n + j - m - 1] * g[j];
            h_num += a[n + m - j - 1] * h[j];
            g_den += a[n + j - m - 1] * h[m - j - 1];
        }

        if g_den == 0.0 {
            return Err(ToeplitzError::SingularPrincipalMinor);
        }

        g[m] = g_num / g_den;
        h[m] = h_num / x_den;
        let k = m - 1;
        let m2 = (m + 1) >> 1;
        let c1 = g[m];
        let c2 = h[m];
        for j in 0..m2 {
            let gj = g[j];
            let gk = g[k - j];
            let hj = h[j];
            let hk = h[k - j];
            g[j] = gj - (c1 * hk);
            g[k - j] = gk - (c1 * hj);
            h[j] = hj - (c2 * gk);
            h[k - j] = hk - (c2 * gj);
        }
    }

    Ok(x)
}

pub fn assert_array_eq(actual: &Array1<f32>, expected: &Array1<f32>, epsilon: f32) {
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

pub fn assert_array2_eq(actual: &Array2<f32>, expected: &Array2<f32>, epsilon: f32) {
    assert_eq!(
        actual.shape(),
        expected.shape(),
        "Arrays have different shapes.\nActual shape: {:?}\nExpected shape: {:?}",
        actual.shape(),
        expected.shape()
    );

    for (((i, j), a), e) in actual.indexed_iter().zip(expected.iter()) {
        if !a.abs_diff_eq(e, epsilon) {
            panic!(
                "Assertion failed at index [{}, {}].\nActual array:\n{:?}\nExpected array:\n{:?}\nDifference at [{}, {}]: {} vs {}",
                i, j, actual, expected, i, j, a, e
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

    #[test]
    fn test_toeplitz_singular_principal_minor() {
        let a = array![1.0, 2.0, 0.0, 2.0, 1.0]; // Note the 0.0 in the middle (a[n-1])
        let b = array![1.0, 2.0, 3.0];
        let result = solve_toeplitz(a.view(), b.view());
        assert!(matches!(result, Err(ToeplitzError::SingularPrincipalMinor)));
    }

    #[test]
    fn test_toeplitz_non_singular_case() {
        // Solution computed using:
        // scipy.linalg.solve_toeplitz([1., 2., 3., 4.], [2., 2., -1., 4.])
        // note the input format is a bit different in our rewrite, we explcitly require
        // the whole "edge" of the toeplitz matrix as the first argument
        let a = array![4., 3., 2., 1., 2., 3., 4.];
        let b = array![2., 2., -1., 4.];

        match solve_toeplitz(a.view(), b.view()) {
            Ok(x) => {
                let expected: Array1<f32> = array![0.6, -1.5, 4., -1.9];
                assert_array_eq(&x, &expected, 1e-6);
            }
            Err(e) => panic!("Expected Ok result, but got Err: {:?}", e),
        }
    }
}
