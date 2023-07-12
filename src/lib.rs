mod matrix_traits;
use matrix_traits::*;
use std::{fmt::{self}, ops::{self, RangeBounds}};

pub struct Matrix<T> {
    rows: Vec<Vec<T>>
}

impl<T: Clone, const M: usize, const N: usize> From<[[T; M]; N]> for Matrix<T> {
    fn from(arr: [[T; M]; N]) -> Self {
        let mut m = Matrix::new();
        for row in arr {
            m.rows.push(Vec::from(row));
        }    
        m
    }
}

pub trait FromStr<T> : Sized {
    fn from_str(_: T) -> Self;
}

impl<const M: usize, const N: usize> FromStr<[[&str; M]; N]> for Matrix<String> {
    fn from_str(arr: [[&str; M]; N]) -> Self {
        let mut m: Matrix<String> = Matrix::with_capacity(N, M);
        for (index, row) in arr.iter().enumerate() {
            for elem in row {
                m.rows[index].push(elem.to_string());
            }
        }
        m
    }
}

impl<T: Clone> Matrix<T> {
    pub fn new_fill(height: usize, width: usize, value: T) -> Self {
        Matrix { rows: vec![vec![value; width]; height]}
    }

    pub fn transpose(self) -> Self {
        let mut ret = Matrix::with_capacity(self.width(), self.height());
        for row_index in 0..self.height() {
            for column_index in 0..self.width() {
                ret.push_to_row(column_index, self.rows(row_index)[column_index].clone());
            }
        }
        ret
    }

    pub fn cut<R: RangeBounds<usize>, C: RangeBounds<usize>>(&self, rows: R, cols: C) -> Self {
        let row_start = match rows.start_bound() {
            ops::Bound::Included(x) => x.clone(),
            ops::Bound::Excluded(x) => x + 1,
            ops::Bound::Unbounded => 0
        };

        let row_end = match rows.end_bound() {
            ops::Bound::Included(x) => x.clone(),
            ops::Bound::Excluded(x) => x - 1,
            ops::Bound::Unbounded => self.height() - 1
        };

        let col_start = match cols.start_bound() {
            ops::Bound::Included(x) => x.clone(),
            ops::Bound::Excluded(x) => x + 1,
            ops::Bound::Unbounded => 0
        };

        let col_end = match cols.end_bound() {
            ops::Bound::Included(x) => x.clone(),
            ops::Bound::Excluded(x) => x - 1,
            ops::Bound::Unbounded => self.width() - 1
        };

        if row_start >= self.height() || row_end >= self.height() || row_end < row_start {
            panic!("Row index out of bounds: [{row_start}, {row_end}] / [0, {}]", self.width() - 1)
        }

        if col_start >= self.width() || col_end >= self.width() || col_end < col_start {
            panic!("Column index out of bounds: [{col_start}, {col_end}] / [0, {}]", self.width() - 1)
        }
        
        let height = row_end - row_start + 1;
        let width = col_end - col_start + 1;
        let mut ret: Matrix<T> = Matrix::with_capacity(height, width);
        for row_index in 0..height {
            for col_index in 0..width {
                ret.rows[row_index].push(self.rows[row_index + row_start][col_index + col_start].clone());
            }
        }
        ret
    }

    pub fn set(&mut self, row_index: usize, col_index: usize, value: T) {
        self.rows[row_index][col_index] = value;
    }

    pub fn get(&self, row_index: usize, col_index: usize) -> T {
        self.rows[row_index][col_index].clone()
    }
}

impl<T: Clone> Clone for Matrix<T> {
    fn clone(&self) -> Self {
        Self { rows: self.rows.clone() }
    }
}

impl<T: MatrixZeroOne> Matrix<T> {

    pub fn is_identity(&self) -> bool {
        if self.height() != self.width() {
            return false;
        }
        
        for row_index in 0..self.height() {
            for col_index in 0..self.width() {

                if row_index == col_index {
                    if self.rows(row_index)[col_index].is_one() {
                        continue;
                    }
                }
                else {
                    if self.rows(row_index)[col_index].is_zero() {
                        continue;
                    }
                }

                return false;
            }
        }

        true
    }

    pub fn identity(size: usize) -> Matrix<T> {
        let mut ret: Matrix<T> = Matrix::with_capacity(size, size);
        for vert_index in 0..size {

            for horr_index in 0..size {
                if vert_index == horr_index {
                    ret.push_to_row(vert_index, T::one());
                }
                else {
                    ret.push_to_row(vert_index, T::zero());
                }
            }
        }
        ret
    }
}

impl<T: Clone + MatrixElem> ops::Add for Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.height() != rhs.height() {
            panic!("Can't add matrices A+B: A.height ({}) != B.height ({})", self.height(), rhs.height())
        }

        if self.width() != rhs.width() {
            panic!("Can't add matrices A+B: A.width ({}) != B.width ({})", self.width(), rhs.width())
        }

        let mut ret = Matrix::with_capacity(self.height(), self.width());
        for row_index in 0..self.height() {
            for col_index in 0..self.width() {
                ret.push_to_row(row_index, self.rows(row_index)[col_index].clone().add(rhs.rows(row_index)[col_index].clone()));
            }
        }
        ret
    }
}

impl<T: Clone + MatrixElem> ops::Mul<T> for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: T) -> Self::Output {
        self.mul_scalar(&rhs)
    }
}

impl<T: Clone + MatrixElem> ops::Mul for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_matrix(&rhs)
    }
}

impl<T: Clone + MatrixElem> ops::Mul for &Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_matrix(rhs)
    }
}

impl<T: Clone + MatrixElem> Matrix<T> {
    fn mul_matrix(&self, rhs: &Self) -> Self {
        if self.width() != rhs.height() {
            panic!("Can't multiply matrices AxB: A.width ({}) != B.height ({})", self.width(), rhs.height())
        }

        let mut ret: Matrix<T> = Matrix::with_capacity(self.height(), rhs.width());
        for vert_index in 0..self.height() {

            for horr_index in 0..rhs.width() {
                
                // Initialize new cell with first multiplication to avoid requiring T::zero()
                let mut new_val = self.rows(vert_index)[0].clone().mul(rhs.rows(0)[horr_index].clone());
                for i in 1..self.width() {    

                    new_val = new_val.add(
                        self.rows(vert_index)[i].clone()
                        .mul(
                        rhs.rows(i)[horr_index].clone())
                    );
                }
                ret.push_to_row(vert_index, new_val);
            }
        }
        ret
    }

    fn mul_scalar(&self, rhs: &T) -> Self {
        let mut ret: Matrix<T> = Matrix::with_capacity(self.height(), self.width());
        for vert_index in 0..self.height() {
            for horr_index in 0..self.width() {
                ret.push_to_row(vert_index, self.rows(vert_index)[horr_index].clone().mul(rhs.clone()));
            }
        }
        ret
    }


    pub fn dot(&self, other: &Self) -> T {

        // Only allowed if both matrices have dim == 1
        if self.height() != 1 || other.height() != 1 {
            panic!("Dot operation only allowed on vectors! Matrix A has dim={}, Matrix B has dim={}", self.height(), other.height())
        }

        if self.width() != other.width() {
            panic!("Dot operation only allowed on vectors of the same length. Vector A length={}, Vector B length={}", self.width(), other.width())
        }

        let mut ret = self.rows(0)[0].clone().mul(other.rows(0)[0].clone());
        for i in 1..self.width() {
            ret = ret.add(self.rows(0)[i].clone().mul(other.rows(0)[i].clone()));
        }
        ret
    }
    
    pub fn norm(&self) -> f64 {
        let mut ret = 0.0f64;
        for i in 0..self.height() {
            for j in 0..self.width() {
                ret += self.rows(i)[j].clone().to_f64().powi(2);
            }
        }

        ret.sqrt()
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (row, vector) in self.rows.iter().enumerate() {
            for (col, value) in vector.iter().enumerate() {
                match f.precision() {
                    Some(precision) => { write!(f, "{:.*}", precision, value)? },
                    None => { write!(f, "{}", value)? }
                }
    
                if col < vector.len() - 1 {
                    write!(f, ",")?
                }
            }

            if row < self.rows.len() - 1 {
                write!(f, "\n")?
            }
        }
        
        Ok(())
    }
}

impl<T> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix { rows: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.rows.len() == 0
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn width(&self) -> usize {
        if self.is_empty() {
            return 0;
        }

        self.rows[0].len()
    }

    fn with_capacity(height: usize, width: usize) -> Matrix<T> {
        let mut rows: Vec<Vec<T>> = Vec::with_capacity(height);
        for _ in 0..height {
            rows.push(Vec::with_capacity(width));
        }
        Matrix { rows: rows }
    }

    fn rows(&self, index: usize) -> &Vec<T> {
        &self.rows[index]
    }

    fn push_to_row(&mut self, index: usize, value: T) {
        self.rows[index].push(value);
    }
}