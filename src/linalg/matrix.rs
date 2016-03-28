use linalg::num::traits::{Num, Zero, One};


// ----- Definitions ---------------------------------------------------------------------------

pub trait MNum : Num + Clone {
}


#[allow(dead_code)]
pub enum MatrixOpResult {
    Successful,
    SizeMismatch,
    InvalidIndex,
    InvalidSize,
    NotSquareMatrix,
    NotVector,
}


#[allow(dead_code)]
pub struct Matrix<T: MNum> {
    n:      usize,
    m:      usize,
    data:   Vec<T>,
}


// ----- Helper functions ----------------------------------------------------------------------


#[allow(dead_code)]
fn add_mnum<T: MNum>(rhs: T, lhs: T) -> T {
    rhs + lhs
}

#[allow(dead_code)]
fn sub_mnum<T: MNum>(rhs: T, lhs: T) -> T {
    rhs - lhs
}

// ----- Traits ---------------------------------------------------------------------------------

pub trait TMatrix<T: MNum> {

// ----- Manipulation functions

    fn new(n: usize, m: usize) -> Result<Matrix<T>, MatrixOpResult>;
    fn new_filled(n: usize, m: usize, filler: &Vec<T>) -> Result<Matrix<T>, MatrixOpResult>;
    fn coord_transform(&self, i: usize, j: usize) -> Result<usize, MatrixOpResult>;
    fn index_transform(&self, n: usize) -> Result<(usize, usize), MatrixOpResult>;
    fn get_n(&self) -> usize;
    fn get_m(&self) -> usize;
    fn get_immut(&self, i: usize, j: usize) -> Result<T, MatrixOpResult>;
    fn get_mut(&mut self, i: usize, j: usize) -> Result<&mut T, MatrixOpResult>;
    fn load_identity(&mut self) -> Result<MatrixOpResult, MatrixOpResult>;

// ----- Mathematical functions

    fn by_each_element_immut(&self, rhs: &Matrix<T>, op_function: &Fn(T, T) -> T) -> Result<Matrix<T>, MatrixOpResult>;
    fn by_each_element_mut(&mut self, rhs: &Matrix<T>, op_function: &Fn(T, T) -> T) -> Result<MatrixOpResult, MatrixOpResult>;
    fn add_immut(&self, rhs: &Matrix<T>) -> Result<Matrix<T>, MatrixOpResult>;
    fn add_mut(&mut self, rhs: &Matrix<T>) -> Result<MatrixOpResult, MatrixOpResult>;
    fn sub_immut(&self, rhs: &Matrix<T>) -> Result<Matrix<T>, MatrixOpResult>;
    fn sub_mut(&mut self, rhs: &Matrix<T>) -> Result<MatrixOpResult, MatrixOpResult>;
    fn mul_immut(&self, rhs: &T) -> Result<Matrix<T>, MatrixOpResult>;
    fn mul_mut(&mut self, rhs: &T) -> Result<MatrixOpResult, MatrixOpResult>;
    fn xmul_immut(&self, rhs: &Matrix<T>) -> Result<Matrix<T>, MatrixOpResult>;
    fn xmul_mut(&mut self, rhs: &Matrix<T>) -> Result<MatrixOpResult, MatrixOpResult>;
    fn transpose_immut(&self) -> Result<Matrix<T>, MatrixOpResult>;
    fn transpose_mut(&mut self) -> Result<MatrixOpResult, MatrixOpResult>;

}

// ----- Manipulator functions implementation ---------------------------------------------------

impl<T: MNum> Clone for Matrix<T> {
    fn clone(&self) -> Matrix<T> {
        Matrix { n: self.n, m: self.m, data: self.data.clone() }
    }

}

#[allow(dead_code)]
impl<T: MNum> TMatrix<T> for Matrix<T> {
    fn new(n: usize, m: usize) -> Result<Matrix<T>, MatrixOpResult> {
        if (n < 1) || (m < 1) {
            return Err(MatrixOpResult::InvalidSize);
        }
        
        let size: usize = n * m;
        let mut data: Vec<T> = Vec::with_capacity(size);

        for _ in 0..size {
            data.push(T::zero());
        }
        
        return Ok(Matrix { n: n, m: m, data: data });
    }

    fn new_filled(n: usize, m: usize, filler: &Vec<T>) -> Result<Matrix<T>, MatrixOpResult>
    {
        if (n < 1) || (m < 1) {
            return Err(MatrixOpResult::InvalidSize);
        }

        let size: usize = n * m;

        if filler.len() != size {
            return Err(MatrixOpResult::SizeMismatch);
        }

        return Ok(Matrix { n: n, m: m, data: filler.clone() });
    }

    fn coord_transform(&self, i: usize, j: usize) -> Result<usize, MatrixOpResult> {
        if (i < self.n) && (j < self.m) {
            return Ok(i * self.n + j);
        } else {
            return Err(MatrixOpResult::InvalidIndex); 
        }
    }

    fn index_transform(&self, n: usize) -> Result<(usize, usize), MatrixOpResult> {
        if n < self.data.len() {
            return Ok((n / self.n.clone(), n % self.n.clone()));
        } else {
            return Err(MatrixOpResult::InvalidIndex); 
        }
    }
    
    fn get_n(&self) -> usize {
        return self.n;
    }

    fn get_m(&self) -> usize {
        return self.m;
    }

    fn get_immut(&self, i: usize, j: usize) -> Result<T, MatrixOpResult> {
        match self.coord_transform(i, j) {
            Ok(value)   =>  return Ok(self.data.get(value).unwrap().clone()),
            Err(err)    =>  return Err(err),  
        }
    }

    fn get_mut(&mut self, i: usize, j: usize) -> Result<&mut T, MatrixOpResult> {
        match self.coord_transform(i, j) {
            Ok(value)   =>  return Ok(self.data.get_mut(value).unwrap()),
            Err(err)    =>  return Err(err),  
        }
    }
    
    fn load_identity(&mut self) -> Result<MatrixOpResult, MatrixOpResult> {
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
            return Ok(MatrixOpResult::Successful);
        } else {
            return Err(MatrixOpResult::NotSquareMatrix);
        }
    }

// ----- Mathematics implementation -------------------------------------------------------------

    fn by_each_element_immut(&self, rhs: &Matrix<T>, op_function: &Fn(T, T) -> T) -> Result<Matrix<T>, MatrixOpResult> {
        if (self.n != rhs.n) | (self.m != rhs.m) {
            return Err(MatrixOpResult::SizeMismatch)
        }
        
        let mut clone = self.clone();
        match clone.by_each_element_mut(rhs, op_function) {
            Ok(_)       => Ok(clone),
            Err(err)    => Err(err),
        }
    }

    fn by_each_element_mut(&mut self, rhs: &Matrix<T>, op_function: &Fn(T, T) -> T) -> Result<MatrixOpResult, MatrixOpResult> {
        if (self.n != rhs.n) | (self.m != rhs.m) {
            return Err(MatrixOpResult::SizeMismatch)
        }

        for idx in 0..self.data.len() {
            let item: &mut T = self.data.get_mut(idx).unwrap();
            let lhsval = item.clone();
            let rhsval = rhs.data.get(idx).unwrap().clone();
            *item = op_function(lhsval, rhsval);
            
        }

        return Ok(MatrixOpResult::Successful)
    }

    fn add_immut(&self, rhs: &Matrix<T>) -> Result<Matrix<T>, MatrixOpResult> {
        let fun = add_mnum;
        return self.by_each_element_immut(rhs, &fun);
    }

    fn add_mut(&mut self, rhs: &Matrix<T>) -> Result<MatrixOpResult, MatrixOpResult> { 
        let fun = add_mnum;
        return self.by_each_element_mut(rhs, &fun);
    }

    fn sub_immut(&self, rhs: &Matrix<T>) -> Result<Matrix<T>, MatrixOpResult> {
        let fun = sub_mnum;
        return self.by_each_element_immut(rhs, &fun);
    }

    fn sub_mut(&mut self, rhs: &Matrix<T>) -> Result<MatrixOpResult, MatrixOpResult> { 
        let fun = sub_mnum;
        return self.by_each_element_mut(rhs, &fun);
    }

    fn mul_immut(&self, rhs: &T) -> Result<Matrix<T>, MatrixOpResult> {
        let mut clone = self.clone();
        match clone.mul_mut(rhs) {
            Ok(_)       => Ok(clone),
            Err(err)    => Err(err),
        }
    }

    fn mul_mut(&mut self, rhs: &T) -> Result<MatrixOpResult, MatrixOpResult> {
        for item in &mut self.data {
            let lhsval: T = item.clone();
            let rhsval: T = rhs.clone();
            *item = lhsval * rhsval;
        }

        Ok(MatrixOpResult::Successful)
    }

    fn xmul_immut(&self, rhs: &Matrix<T>) -> Result<Matrix<T>, MatrixOpResult> {
        if self.m != rhs.n {
            return Err(MatrixOpResult::SizeMismatch);
        }
        
        let mut result = {
            match Matrix::new(self.n, rhs.m) {
                Ok(value)   => value,
                Err(err)    => return Err(err),
            }
        };
        
        for i in 0..result.n {
            for j in 0..result.n {
                for k in 0..result.m {
                    let nval: &mut T = try!(result.get_mut(i, k));
                    *nval = nval.clone() + try!(self.get_immut(i, k)) * try!(rhs.get_immut(j, k)); 
                }
            }
        }

        Ok (result)
    }

    fn xmul_mut(&mut self, rhs: &Matrix<T>) -> Result<MatrixOpResult, MatrixOpResult> {
        match self.xmul_immut(rhs) {
            Ok(value)   => {
                *self = value;
                Ok(MatrixOpResult::Successful)
            },
            Err(err)    => Err(err)
        }
    }

    fn transpose_immut(&self) -> Result<Matrix<T>, MatrixOpResult> {
        let size: usize = self.n * self.m;
        let mut buffer: Vec<T> = Vec::with_capacity(size);
        
        for j in 0..self.m {
            for i in 0..self.n {
                buffer.push(try!(self.get_immut(i, j)).clone());
            }
        }
        
        Ok (Matrix { n: self.m, m: self.n, data: buffer })
    }

    fn transpose_mut(&mut self) -> Result<MatrixOpResult, MatrixOpResult> {
        *self = try!(self.transpose_immut());
        Ok (MatrixOpResult::Successful)
    }
}


// ----- Operator implementation -----------------------------------------------------------------

