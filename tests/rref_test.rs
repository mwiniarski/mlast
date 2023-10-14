use mm::{rref::*, matrix::*};

use crate::common::fix_zeroes;
mod common;

#[test]
fn magic_matrix() {
    let a = Matrix::from(
        [[8.,1.,6.],
         [3.,5.,7.],
         [4.,9.,2.]]);
    
    let res = rref(&a);
    assert_eq!(format!("{:.2}", res), format!("{:.2}", Matrix::<f64>::identity(a.height())));
}

#[test]
fn tall_matrix() {
    let a = Matrix::from(
        [[1.,2.,3.],
         [2.,4.,6.],
         [2.,6.,8.],
         [2.,8.,10.]]);

    let r = Matrix::from(
       [[1.,0.,1.],
        [0.,1.,1.],
        [0.,0.,0.],
        [0.,0.,0.]]);

    let res = rref(&a);
    assert_eq!(format!("{:.2}\n", r), format!("{:.2}\n", res));
}

#[test]
fn wide_matrix() {
    let a = Matrix::from(
        [[1.,2.,2.,2.],
         [2.,4.,6.,8.],
         [3.,6.,8.,10.]]);

    let r = Matrix::from(
       [[1.,2.,0.,-2.],
        [0.,0.,1.,2.],
        [0.,0.,0.,0.]]);

    let res = rref(&a);
    assert_eq!(format!("{:.2}\n", r), format!("{:.2}\n", fix_zeroes(res)));
}

#[test]
fn get_rank() {
    let a = Matrix::from(
        [[1.,2.,2.,2.],
         [2.,4.,6.,8.],
         [3.,6.,8.,10.]]);
    let r = rank(&a);
    assert_eq!(r, 2);

    let r = rank(&a.transpose());
    assert_eq!(r, 2);

    let m = Matrix::from(
        [[8.,1.,6.],
         [3.,5.,7.],
         [4.,9.,2.]]);
    let r = rank(&m);
    assert_eq!(r, 3);
}

// #[test]
// #[allow(dead_code)]
// fn array_oob() {
//     use std::{fs::OpenOptions, any::Any};
//     macro_rules! assert_can_mut {
//         ($m1:expr, $m2:expr) => {
//             const fn can_mul(m1: M, m2: M) -> bool {
//                 return m1.c == m2.c
//             }

            
//             const _: () = {
//                 if !can_mul($m1, $m2) {
//                     assert!(false, "ASD");
//                 }
//             };
//         }
//     }

//     struct M {
//         c: usize
//     }

//     impl M {
//         const fn from<const N: usize>(_: [usize; N]) -> M {
//             M { c: N }
//         }
//     }
//     const a: M = M::from([1,2]);
//     let b = M::from([1,2,3]);

//     let x = 5;
// }

