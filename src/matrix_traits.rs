pub trait MatrixElem {
    fn add(self, rhs: Self) -> Self;
    fn sub(self, rhs: Self) -> Self;
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

    // A bit overengineered but fun
    fn sub(self, rhs: Self) -> Self {
        let s = "+".to_owned() + &rhs;
        match self.find(&s) {
            Some(_) => self.replacen(&s, "", 1),
            None => {
                let s = rhs.clone() + "+";
                match self.find(&s) {
                    Some(_) => self.replacen(&s, "", 1),
                    None => return format!("{}-{}", self, rhs)
                }
            }
        }
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
    ($zero:expr, $one:expr, $($type:ty),* $(,)*) => {$(
        impl MatrixElem for $type {
            fn add(self, rhs: Self) -> Self {
                self + rhs
            }

            fn sub(self, rhs: Self) -> Self {
                self - rhs
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
    )*};
}

impl_matrix_traits!(0, 1, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);
impl_matrix_traits!(0., 1., f32, f64);
