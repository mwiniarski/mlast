use mm::{lu_decomposition::*, matrix::*};
mod common;

#[test]
fn lu_gauss_2x2() {
    let a = Matrix::from(
        [[2.,1.],
         [8.,12.]]);
    let l = Matrix::from(
        [[1.,0.],
         [0.25,1.]]);
    let u = Matrix::from(
        [[8.,12.],
         [0.,-2.]]);
    let p = Matrix::from(
        [[0.,1.],
         [1.,0.]]);

    let res = lu_gauss(&a);
    assert_eq!(format!("{:.2}", res.l), format!("{:.2}", l));
    assert_eq!(format!("{:.2}", res.u), format!("{:.2}", u));
    assert_eq!(format!("{}", res.p), format!("{}", p));

}

#[test]
fn lu_gauss_row_vector() {
    let a = Matrix::from([[1.,2.,3.]]);
    let l = Matrix::from([[1.]]);
    let u = Matrix::from([[1.,2.,3.]]);
    let p = Matrix::from([[1.]]);

    let res = lu_gauss(&a);
    assert_eq!(format!("{}", res.l), format!("{}", l));
    assert_eq!(format!("{}", res.u), format!("{}", u));
    assert_eq!(format!("{}", res.p), format!("{}", p));
}

#[test]
fn lu_solve_test() {
    let l = Matrix::from(
        [[1.,0.,0.],
         [2.,1.,0.],
         [4.,-1.,1.]]);
    let u = Matrix::from(
        [[1.,0.,2.],
         [0.,-1.,-1.],
         [0.,0.,-1.]]);
    let lu = LUResult { l: l, u: u, p: Matrix::<f64>::identity(3)};
    let b = Matrix::from([[-4.,-6.,-15.]]).transpose();
    let x = Matrix::from([[2.,1.,-3.]]).transpose();

    let res = lu_solve(&lu, &b);
    assert_eq!(format!("{:.2}", res), format!("{:.2}", x)); 
}

#[test]
fn inverse_with_lu() {
    let a = Matrix::from(
        [[1.,0.,2.],
         [2.,-1.,3.],
         [4.,1.,8.]]);
    let inv_a = Matrix::from(
        [[-11.,2.,2.],
         [-4.,0.,1.],
         [6.,-1.,-1.]]);

    let res = inv(&a);
    assert_eq!(format!("{:.2}", common::fix_zeroes(res)), format!("{:.2}", inv_a)); 
}

#[test]
fn permutation_matrix_identity() {
    let a = Matrix::from(
        [[4.,1.,8.],
         [1.,0.,2.],
         [2.,0.5,3.]]);
    let res = lu_gauss(&a);
    assert_eq!(format!("{:.2}", res.p), format!("{:.2}", Matrix::<f64>::identity(a.height()))); 
}

#[test]
fn permutation_matrix() {
    let a = Matrix::from(
        [[3.,1.,6.],
         [2.,1.,3.],
         [1.,1.,1.]]);
    let p = Matrix::from(
        [[1,0,0],
         [0,0,1],
         [0,1,0]]);
    let res = lu_gauss(&a);
    assert_eq!(format!("{}", res.p), format!("{}", p)); 
}
