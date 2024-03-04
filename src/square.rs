use crate::numeric::Numeric;
use crate::matrix::{ Matrix };

pub trait Square {
    fn is_square(&self) -> bool;
}

impl<T> Square for Matrix<T> where T: Numeric {
    fn is_square(&self) -> bool {
        self.rows() == self.cols()
    }
}

