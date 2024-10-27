// use nalgebra::Complex;
// use num::complex::Complex;

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

#[cfg(test)]
pub mod tests {
    use crate::signal_processing::tests::{assert_array2_eq, assert_complex_array1_eq};
    use nalgebra::Complex;
    use ndarray::{array, Array1};

    use super::get_companion_matrix;

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
}
