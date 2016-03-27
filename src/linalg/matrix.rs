use std::ops::Add;
use linalg::num::traits::{Num, Zero, One};


// ----- Definitions ---------------------------------------------------------------------------

pub trait MNum : Num + Clone {}

#[allow(dead_code)]
pub enum MatrixOpError {
    Successful,
    SizeMismatch,
    InvalidIndex,
    InvalidSize,
    NotSquareMatrix,
    NotVector,
}

#[allow(dead_code)]
struct Matrix<T: MNum> {
    n:      usize,
    m:      usize,
    data:   Vec<T>,
}

// ----- Manipulator functions implementation ---------------------------------------------------

#[allow(dead_code)]
impl<T: MNum> Matrix<T> {
    fn create_matrix(n: usize, m: usize) -> Result<Matrix<T>, MatrixOpError> {
        if (n < 1) || (m < 1) {
            return Err(MatrixOpError::InvalidSize);
        }
        
        let size: usize = n * m;
        let mut data: Vec<T> = Vec::with_capacity(size);

        for _ in 0..size {
            data.push(T::zero());
        }
        
        return Ok(Matrix { n: n, m: m, data: data });
    }

    fn coord_transform(&self, i: usize, j: usize) -> Result<usize, MatrixOpError> {
        if (i < self.n) && (j < self.m) {
            return Ok(i * self.n + j);
        } else {
            return Err(MatrixOpError::InvalidIndex); 
        }
    }

    fn index_transform(&self, n: usize) -> Result<(usize, usize), MatrixOpError> {
        if n < self.data.len() {
            return Ok((n / self.n.clone(), n % self.n.clone()));
        } else {
            return Err(MatrixOpError::InvalidIndex); 
        }
    }

    fn get_immut(&self, i: usize, j: usize) -> Result<T, MatrixOpError> {
        match self.coord_transform(i, j) {
            Ok(value)   =>  return Ok(self.data.get(value).unwrap().clone()),
            Err(err)    =>  return Err(err),  
        }
    }

    fn get_mut(&mut self, i: usize, j: usize) -> Result<&mut T, MatrixOpError> {
        match self.coord_transform(i, j) {
            Ok(value)   =>  return Ok(self.data.get_mut(value).unwrap()),
            Err(err)    =>  return Err(err),  
        }
    }
    
    fn load_identity(&mut self) -> Result<MatrixOpError, MatrixOpError> {
        if self.n == self.m {
            for idx in 0..self.data.len() {
                match self.index_transform(idx) {
                    Ok((i, j))  => {
                        let item: &mut T = self.data.get_mut(idx).unwrap();

                        if i == j {
                            *item = T::one();
                        } else {
                            *item = T::zero();
                        }
                    },
                    Err(_)    => panic!("Unrecovarable error"),
                }
            }
            return Ok(MatrixOpError::Successful);
        } else {
            return Err(MatrixOpError::NotSquareMatrix);
        }
    }

}


impl<T: MNum> Clone for Matrix<T> {
    fn clone(&self) -> Matrix<T> {
        Matrix { n: self.n, m: self.m, data: self.data.clone() }
    }

}

// ----- Mathematics implementation -------------------------------------------------------------

#[allow(dead_code)]
impl<T: MNum> Matrix<T> {

    fn add_immut(&self, rhs: &Matrix<T>) -> Result<Matrix<T>, MatrixOpError> {
        if (self.n != rhs.n) | (self.m != rhs.m) {
            return Err(MatrixOpError::SizeMismatch)
        }
        
        let mut result: Vec<T> = Vec::with_capacity(self.data.len());

        for idx in 0..self.data.len() {
            let lhsval: T = self.data.get(idx).unwrap().clone();
            let rhsval: T = rhs.data.get(idx).unwrap().clone();
            result.push(lhsval + rhsval);
        }

        return Ok(Matrix { n: self.n, m: self.m, data: result })
    
    }

    fn add_mut(&mut self, rhs: &Matrix<T>) -> Result<MatrixOpError, MatrixOpError> { 
        if (self.n != rhs.n) | (self.m != rhs.m) {
            return Err(MatrixOpError::SizeMismatch)
        }

        for idx in 0..self.data.len() {
            let item: &mut T = self.data.get_mut(idx).unwrap();
            let lhsval = item.clone();
            let rhsval = rhs.data.get(idx).unwrap().clone();
            *item = lhsval + rhsval;
            
        }

        return Ok(MatrixOpError::Successful)
    }
}


// ----- Operator implementation -----------------------------------------------------------------

#[allow(dead_code)]
impl<T: MNum> Add for Matrix<T> {
    type Output = Result<Matrix<T>, MatrixOpError>;

    fn add(self, rhs: Matrix<T>) -> Result<Matrix<T>, MatrixOpError> {
        if (self.n != rhs.n) | (self.m != rhs.m) {
            return Err(MatrixOpError::SizeMismatch)
        }
        
        let mut result: Vec<T> = Vec::with_capacity(self.data.len());

        for idx in 0..self.data.len() {
            let lhsval: T = self.data.get(idx).unwrap().clone();
            let rhsval: T = rhs.data.get(idx).unwrap().clone();
            result.push(lhsval + rhsval);
        }

        return Ok(Matrix { n: self.n, m: self.m, data: result })
    }


}



