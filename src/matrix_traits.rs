pub trait MatrixElem {
    fn add(self, rhs: Self) -> Self;
    fn mul(self, rhs: Self) -> Self;
    fn to_f64(self) -> f64;
}

pub trait MatrixZeroOne {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
    fn one() -> Self;
    fn is_one(&self) -> bool;
}

///
/// Strings
///
impl MatrixElem for String {
    fn add(self, rhs: Self) -> Self {
        format!("{}+{}", self, rhs)
    }

    fn mul(self, rhs: Self) -> Self {
        format!("{}{}", self, rhs)
    }

    fn to_f64(self) -> f64 {
        panic!("Cannot convert string to f64")
    }
}

///
/// Numericals
///
macro_rules! impl_matrix_traits {
    ($type:ty, $zero:expr, $one:expr) => {
        impl MatrixElem for $type {
            fn add(self, rhs: Self) -> Self {
                self + rhs
            }

            fn mul(self, rhs: Self) -> Self {
                self * rhs
            }

            fn to_f64(self) -> f64 {
                self as f64
            }
        }

        impl MatrixZeroOne for $type {
            fn zero() -> $type {
                $zero
            }

            fn one() -> $type {
                $one
            }

            fn is_zero(&self) -> bool {
                &$zero == self
            }

            fn is_one(&self) -> bool {
                &$one == self
            }
        }
    };
}

impl_matrix_traits!(u8, 0, 1);
impl_matrix_traits!(u16, 0, 1);
impl_matrix_traits!(u32, 0, 1);
impl_matrix_traits!(u64, 0, 1);
impl_matrix_traits!(usize, 0, 1);

impl_matrix_traits!(i8, 0, 1);
impl_matrix_traits!(i16, 0, 1);
impl_matrix_traits!(i32, 0, 1);
impl_matrix_traits!(i64, 0, 1);
impl_matrix_traits!(isize, 0, 1);

impl_matrix_traits!(f32, 0.0, 1.0);
impl_matrix_traits!(f64, 0.0, 1.0);
