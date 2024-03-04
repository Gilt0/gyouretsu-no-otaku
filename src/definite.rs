use std::fmt;

use crate::numeric::Numeric;
use crate::matrix::Matrix;
use crate::symmetric::{ Algorithm, Symmetric };

#[derive(Debug, PartialEq)]
pub enum CholeskyDecompositionError {
    MatrixIsNotPositiveDefinite,
    AlgorithmFailed
}

impl fmt::Display for CholeskyDecompositionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CholeskyDecompositionError::MatrixIsNotPositiveDefinite => {
                write!(f, "CHolesky Decomposition failed because matrix is not positive definite.")
            },
            CholeskyDecompositionError::AlgorithmFailed => {
                write!(f, "CHolesky Decomposition failed because selected algorithm failed.")
            }
        }
    }
}

impl std::error::Error for CholeskyDecompositionError {}


pub trait PositiveDefinite {
    fn is_positive_definite(&self) -> bool;
    fn cholesky(&self) -> Result<Matrix<f64>, CholeskyDecompositionError>;
}

impl<T> PositiveDefinite for Matrix<T> where T: Numeric {
    fn is_positive_definite(&self) -> bool {
        if !self.is_symmetric() { return false }
        if let Ok((eigen_values, _)) = self.eigen_decomposition(Algorithm::Jacobi) {
            for value in &eigen_values {
                if *value <= Numeric::zero() { return false }
            }    
        } else { return false }
        true
    }

    // Source: https://www.astro.umd.edu/~ricotti/NEWWEB/teaching/ASTR415/InClassExamples/NR3/code/cholesky.h
    fn cholesky(&self) -> Result<Matrix<f64>, CholeskyDecompositionError> {
        if !self.is_positive_definite() { return Err(CholeskyDecompositionError::MatrixIsNotPositiveDefinite) }
        let n = self.rows();
        let mut l = self.copy_to::<f64>().unwrap();
        for i in 0..n {
            for j in i..n {
                let mut sum = l[(i, j)];
                for k in 0..i {
                    sum -= l[(i, k)] * l[(j, k)];
                }
                l[(j, i)] = if i == j {
                    if sum <= 0f64 { return Err(CholeskyDecompositionError::AlgorithmFailed) }
                    sum.sqrt()
                } else {
                    sum / l[(i, i)]
                };
            }
        }
        for i in 0..n { for j in 0..i { l[(j, i)] = 0f64;} }
        Ok(l)
    }
}

