use std::fmt;

use crate::numeric::Numeric;
use crate::matrix::{ Matrix, MatrixBuilder, Swap };
use crate::square::Square;

pub trait Symmetric {
    fn is_symmetric(&self) -> bool;
    fn eigen_decomposition(&self, algorithm: Algorithm) -> Result<(Vec<f64>, Matrix<f64>), EigenDecompositionError>;
}

impl<T> Symmetric for Matrix<T> where T: Numeric {
    fn is_symmetric(&self) -> bool {
        if !self.is_square() { return false }
        for i in 0..self.rows() {
            for j in 0..=i {
                if self[(i, j)] != self[(j, i)] { return false}
            }
        }
        true
    }

    fn eigen_decomposition(&self, algorithm: Algorithm) -> Result<(Vec<f64>, Matrix<f64>), EigenDecompositionError> {
        match algorithm {
            Algorithm::Jacobi => {
                EigenDecomposition::<T>::decompose(&JacobiDecomposition, self)
            },
        }
        
    }
}


#[derive(Debug)]
pub struct EigenDecompositionError;

impl fmt::Display for EigenDecompositionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred while computing eigen values")
    }
}

impl std::error::Error for EigenDecompositionError {}

pub enum Algorithm {
    Jacobi,
    // Other algorithms may come here
}      

pub trait EigenDecomposition<T> {
    fn decompose(&self, m: &Matrix<T>) -> Result<(Vec<f64>, Matrix<f64>), EigenDecompositionError> where T: Numeric;
}

pub struct JacobiDecomposition;

impl<T> EigenDecomposition<T> for JacobiDecomposition where T: Numeric {
// Source: https://www.astro.umd.edu/~ricotti/NEWWEB/teaching/ASTR415/InClassExamples/NR3/code/eigen_sym.h
fn decompose(&self, m: &Matrix<T>) -> Result<(Vec<f64>, Matrix<f64>), EigenDecompositionError> {
        if !m.is_symmetric() { return Err(EigenDecompositionError) }
        let n = m.rows();
        let eps = std::f64::EPSILON;
        let mut a = m.copy_to::<f64>().unwrap();
        let mut v = MatrixBuilder::<f64>::new().identity(n).build().unwrap();
        let mut d = vec![0f64; n];
        let mut _nrot = 0u64;
        let mut theta: f64;
        let mut b = vec![0f64; n];
        let mut z = vec![0f64; n];
        for ip in 0..n {
            b[ip] = a[(ip, ip)];
            d[ip] = a[(ip, ip)];
            z[ip] = 0f64;
        }
        for i in 1..50 {
            let mut sm = 0f64;
            for ip in 0..(n-1) {
                for iq in (ip+1)..n {
                    sm += a[(ip, iq)].abs();
                }
            }
            if sm == 0f64 {
                match eigen_sort(&mut d, &mut v) {
                    Ok(()) => return Ok((d, v)),
                    Err(_) => return Err(EigenDecompositionError)
                }
            }
            let tresh = if i < 4 {
                sm * 0.2f64 / (n * n) as f64
            } else {
                0f64
            };
            for ip in 0..(n-1) {
                for iq in ip+1..n {
                    let g = a[(ip, iq)].abs() * 100f64;
                    if i > 4 && g <= eps * d[ip].abs() && g <= eps * d[iq].abs() {
                        a[(ip, iq)] = 0f64;
                    } else if a[(ip, iq)].abs() > tresh {
                        let h = d[iq]-d[ip];
                        let t: f64 = if g <= eps * h.abs() {
                            a[(ip, iq)] / h
                        } else {
                            theta = h * 0.5f64 / a[(ip, iq)];
                            let denominator: f64 = theta.abs() + (theta * theta + 1f64).sqrt();
                            if theta < 0f64 {
                                - 1f64 / denominator
                            } else {
                                1f64 / denominator
                            }
                        };
                        let c = 1f64 / (t * t + 1f64).sqrt();
                        let s = t * c;
                        let tau=s/(c + 1f64);
                        let h=t*a[(ip, iq)];
                        z[ip] -= h;
                        z[iq] += h;
                        d[ip] -= h;
                        d[iq] += h;
                        a[(ip, iq)] = 0f64;
                        for j in 0..ip {
                            rot(&mut a,s,tau,j,ip,j,iq);
                        }
                        for j in (ip + 1)..iq {
                            rot(&mut a,s,tau,ip,j,j,iq);
                        }
                        for j in (iq + 1)..n {
                            rot(&mut a,s,tau,ip,j,iq,j);
                        }
                        for j in 0..n {
                            rot(&mut v,s,tau,j,ip,j,iq);
                        }
                        _nrot += 1;
                    }
                }
            }
            for ip in 0..n {
                b[ip] += z[ip];
                d[ip] = b[ip];
                z[ip] = 0f64;
            }
        }
        return Err(EigenDecompositionError)
    }     
}

pub fn eigen_sort<T> (eigen_values: &mut [T], eigen_matrix: &mut Matrix<T>) -> Result<(), EigenDecompositionError>
where T: Numeric {
    if !eigen_matrix.is_square() {
        Err(EigenDecompositionError)
    } else {
        let n: usize = eigen_values.len();
        let mut j_max: usize;
        let mut eigen_max: T;
        for j in 0usize..(n - 1) {
            j_max = j;
            eigen_max = eigen_values[j_max];
            for k in j..n { 
                if eigen_values[k] >= eigen_max {
                    j_max = k;
                    eigen_max = eigen_values[k];
                }
            }
            if j_max != j {
                eigen_values[j_max] = eigen_values[j];
                eigen_values[j] = eigen_max;
                if eigen_matrix.swap_cols(j, j_max).is_err() { return Err(EigenDecompositionError) }
            }
        }
        Ok(())    
    }
}

pub fn rot<T: Numeric>(a: &mut Matrix<T>, s: T, tau: T, i: usize, j: usize, k: usize, l: usize)
{
    let g = a[(i, j)];
    let h = a[(k, l)];
    a[(i, j)] = g - s * (h + g * tau);
    a[(k, l)] = h + s * (g - h * tau);
}
