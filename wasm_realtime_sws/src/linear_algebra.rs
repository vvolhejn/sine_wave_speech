// use nalgebra::Complex;
// use num::complex::Complex;

use thiserror::Error;

use nalgebra::Complex;
use ndarray::*;
use ndarray::{Array1, Array2};
use nshare::{IntoNalgebra, IntoNdarray1};

/// Find the roots of a monic polynomial..
/// `coefs` should not contain the x^n coefficient.
/// The coefficients are in increasing order of degree, meaning
/// coefs[i] is the coefficient of x^i.
pub fn find_roots(coefs: ArrayView1<f32>) -> Array1<Complex<f32>> {
    let companion_matrix = get_companion_matrix(coefs);
    let eigenvalues = companion_matrix.into_nalgebra().complex_eigenvalues();
    eigenvalues.into_ndarray1()
}

/// Given the coefficients of a monic polynomial, return the companion matrix.
/// `coefs` should not contain the x^n coefficient.
/// /// The coefficients are in increasing order of degree, meaning
/// coefs[i] is the coefficient of x^i.
fn get_companion_matrix(coefs: ArrayView1<f32>) -> Array2<f32> {
    let degree = coefs.len() + 1;
    let mut matrix = Array2::zeros((degree - 1, degree - 1));
    for i in 0..degree - 2 {
        matrix[(i, i + 1)] = 1.0;
    }
    for i in 0..degree - 1 {
        matrix[(degree - 2, i)] = -coefs[i];
    }
    matrix
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

#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::signal_processing::tests::{
        assert_array1_eq, assert_array2_eq, assert_complex_array1_eq,
    };
    use nalgebra::Complex;
    use ndarray::{array, Array1};

    #[test]
    fn test_companion_matrix() {
        let polynomial = array![8., -3., 2.];
        let actual = get_companion_matrix(polynomial.view());
        let expected = array![[0., 1., 0.,], [0., 0., 1.,], [-8., 3., -2.,]];
        assert_array2_eq(&actual, &expected, 1e-6);
    }

    #[test]
    fn test_find_roots() {
        // Test cases as (polynomial coefficients, expected roots)
        let test_cases = vec![
            // x^2
            (
                array![0., 0.],
                array![Complex::new(0., 0.), Complex::new(0., 0.)],
            ),
            // x^2 + 1
            (
                array![1., 0.,],
                array![Complex::new(0., -1.), Complex::new(0., 1.)],
            ),
            // x^2 - 1
            (
                array![-1., 0.,],
                array![Complex::new(-1., 0.), Complex::new(1., 0.)],
            ),
            // x^2 - 2x + 1 = (x-1)^2
            (
                array![1., -2.,],
                array![Complex::new(1., 0.), Complex::new(1., 0.)],
            ),
            // x^3 - x^2 - 2x + 2
            // To test numbers that aren't integers
            (
                array![2., -2., -1.,],
                array![
                    Complex::new(-(2. as f32).sqrt(), 0.),
                    Complex::new(1., 0.),
                    Complex::new((2. as f32).sqrt(), 0.),
                ],
            ),
            // x^3 - x^2 - 2x + 3
            // Mix of real and complex roots, weird numbers.
            // Ground truth computed via Wolfram Alpha
            (
                array![3., -2., -1.,],
                array![
                    Complex::new(-1.5468, 0.),
                    Complex::new(1.2734, -0.56382),
                    Complex::new(1.2734, 0.56382),
                ],
            ),
        ];

        for (polynomial, expected_roots) in test_cases {
            let roots = super::find_roots(polynomial.view());
            let mut roots = roots.to_vec();
            // The sorting is stable, meaning this sorts lexicographically
            // by (re, im).
            roots.sort_by(|a, b| a.im.partial_cmp(&b.im).unwrap());
            roots.sort_by(|a, b| a.re.partial_cmp(&b.re).unwrap());
            let roots = Array1::from(roots);

            assert_complex_array1_eq(&roots, &expected_roots, 1e-4);
        }
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
                assert_array1_eq(&x, &expected, 1e-6);
            }
            Err(e) => panic!("Expected Ok result, but got Err: {:?}", e),
        }
    }
}
