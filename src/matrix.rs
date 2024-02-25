use crate::fraction::Fraction;
use crate::error::MatrixOperationError;
use std::fmt;

pub struct Matrix {
    n: usize,
    m: usize,
    content: Vec<Vec<Fraction>>
}

impl Matrix {
    pub fn new(m: usize, n: usize) -> Self {
        Self {
            n: n,
            m: m,
            content: vec![vec![Fraction::from(0); n]; m]
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, i: usize, j: usize) -> Option<Fraction> {
        if i < self.m && j < self.n {
            Some(self.content[i][j].clone())
        }
        else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn put(&mut self, i: usize, j: usize, value: Fraction) 
        -> Result<(), MatrixOperationError> 
    {
        if i < self.m && j < self.n {
            self.content[i][j] = value;
            Ok(())
        }
        else {
            Err(MatrixOperationError::IndexOutOfBounds)
        }
    }

    /// Adding scale*i to j
    #[allow(dead_code)]
    pub fn line_add(&mut self, i: usize, scale: Fraction, j: usize)
        -> Result<&mut Self, MatrixOperationError> 
    {
        if i >= self.m || j >= self.m {
            return Err(MatrixOperationError::IndexOutOfBounds);
        }
        
        if i == j {
            return Err(MatrixOperationError::InvalidOperation);
        }
        
        for k in 0..self.n {
            self.content[j][k] = self.content[j][k] + scale * self.content[i][k];
        }
        Ok(self)
    }

    #[allow(dead_code)]
    pub fn line_mul(&mut self, i: usize, scale: Fraction)
        -> Result<&mut Self, MatrixOperationError> 
    {
        if i >= self.m {
            return Err(MatrixOperationError::IndexOutOfBounds);
        }

        if scale == Fraction::from(0) {
            return Err(MatrixOperationError::InvalidOperation);
        }

        for k in 0..self.n {
            self.content[i][k] = scale * self.content[i][k]
        }
        Ok(self)
    }

    #[allow(dead_code)]
    pub fn line_swap(&mut self, i: usize, j: usize) 
        -> Result<&mut Self, MatrixOperationError>
    {
        if i >= self.m || j >= self.m {
            return Err(MatrixOperationError::IndexOutOfBounds);
        }
        if i == j {
            return Ok(self);
        }

        for k in 0..self.n {
            let tmp = self.content[i][k];
            self.content[i][k] = self.content[j][k];
            self.content[j][k] = tmp;
        }
        Ok(self)
    }

    #[allow(dead_code)]
    pub fn eliminate(&mut self, with_const_vector: bool) {
        let lim = if with_const_vector {
            (self.n - 1).min(self.m)
        } else {
            self.n.min(self.m)
        };

        let mut current_r: usize = 0;
        
        for k in 0..lim {
            let mut is_zero_vector = true;
            let mut target: usize = 0;
            for i in current_r..self.m {
                if !self.content[i][k].is_zero() {
                    is_zero_vector = false;
                    target = i;
                    break;
                }
            }

            if !is_zero_vector {
                let line_scale = self.content[current_r][current_r].clone().inverse().unwrap();
                self.line_swap(target, current_r).unwrap()
                    .line_mul(current_r, line_scale).unwrap();

                println!{"--------\n{}\n--------", self};

                for i in 0..self.m {
                    if i == current_r || self.content[i][current_r].is_zero() {
                        continue;
                    }

                    self.line_add(current_r, Fraction::from(-1)*self.content[i][current_r], i).unwrap();

                    println!{"--------\n{}\n--------", self};
                }
                
                current_r += 1;
            }
        }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        for i in 0..self.m {
            let mut line = String::new();
            for j in 0..self.n {
                line += &format!{"{}", self.content[i][j]};
                if j+1 != self.n {
                    line += " ";
                }
            }

            if i+1 != self.m {
                line += "\n";
            }

            res += &line;
        }

        write!(f, "{res}")
    }
}
