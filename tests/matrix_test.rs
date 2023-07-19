use mm::matrix::*;

#[test]
fn display_precision() {
    let m = Matrix::from([[1.33333333]]);
    assert_eq!(format!("{:.4}", m), "1.3333");
}

#[test]
fn add_matrices1() {
    let m = Matrix::from([[1,1],[1,1]]);
    let n = Matrix::from([[2,2],[2,2]]);
    assert_eq!(format!("{}", m + n), "3,3\n3,3");
}

#[test]
fn add_matrices2() {
    let m = Matrix::from_str([["a","b","c"]]);
    let n = Matrix::from_str([["d","e","f"]]);
    assert_eq!(format!("{}", m + n), "a+d,b+e,c+f");
}

#[test]
fn sub_matrix_string() {
    let m = Matrix::from_str([["a+e","f+b","c"]]);
    let n = Matrix::from_str([["e","f","d"]]);
    let res = Matrix::from_str([["a","b","c-d"]]);
    assert_eq!(format!("{}", m-n), format!("{res}"));
}

#[test]
fn mul_matrices2x2_numbers() {
    let m = Matrix::from([[4,3],[2,1]]);
    let n = Matrix::new_fill(2,2,2);
    assert_eq!(format!("{}", m * n), "14,14\n6,6");
}

#[test]
fn mul_matrices2x2_letters() {
    let m = Matrix::from_str([["a","b"],
                                              ["c","d"]]);
    let n = Matrix::from_str([["e","f"],
                                              ["g","h"]]);
    assert_eq!(format!("{}", m * n), "ae+bg,af+bh\nce+dg,cf+dh");
}

#[test]
fn mul_matrices3x1() {
    let m = Matrix::from_str([["a","b","c"],
                                              ["m","n","o"],
                                              ["r","s","t"]]);
    let n = Matrix::from_str([["x", "y", "z"]]).transpose();
    assert_eq!(format!("{}", m * n), "ax+by+cz\nmx+ny+oz\nrx+sy+tz");
}

#[test]
fn mul_matrices_float() {
    let m = Matrix::new_fill(1, 10, 3.);
    let n = Matrix::new_fill(10, 1, 1./3.);
    let p = m * n;
    assert_eq!(p.height(), 1);
    assert_eq!(p.width(), 1);

    let m = Matrix::new_fill(10, 1, 3.);
    let n = Matrix::new_fill(1, 10, 1./3.);
    let p = m * n;
    assert_eq!(p.height(), 10);
    assert_eq!(p.width(), 10);
}

#[test]
#[should_panic(expected = "Can't multiply matrices AxB: A.width (5) != B.height (6)")]
fn mul_matrices_bad_sizes() {
    let m = Matrix::new_fill(4, 5, 3.);
    let n = Matrix::new_fill(6, 7, 1./3.);
    let _ = m * n;
}

#[test]
fn from() {
    let m = Matrix::from([[1,2,3], [1,2,3]]);
    assert_eq!(format!("{}", m), "1,2,3\n1,2,3");
}

#[test]
fn transpose_1() {
    let m = Matrix::from_str([["a","b","c"], ["d","e","f"]]);
    assert_eq!(format!("{}", m.transpose()), "a,d\nb,e\nc,f");
}

#[test]
fn transpose_2() {
    let m = Matrix::from([[1,2,3]]);
    let transposed = m.transpose();
    assert_eq!(format!("{}", transposed), "1\n2\n3");
    assert_eq!(transposed.height(), 3);
    assert_eq!(transposed.width(), 1);
}

#[test]
fn is_identity_not_squared() {
    let m = Matrix::from([[1,0,0]]);
    assert!(!m.is_identity());
}

#[test]
fn is_identity_1() {
    let m = Matrix::from([[1,2,3],[2,1,3],[2,3,1]]);
    assert!(!m.is_identity());
}

#[test]
fn is_identity_2() {
    let m = Matrix::from([[1,0,0],[0,1,0],[0,0,1]]);
    assert!(m.is_identity());
}

#[test]
fn new_identity() {
    let m: Matrix<u32> = Matrix::identity(3);
    assert_eq!(format!("{}", m), "1,0,0\n0,1,0\n0,0,1");
    assert!(m.is_identity());
}

#[test]
fn transpose_of_product() {

    // transpose of a product equals a reverse product of transposes
    let m = Matrix::from([[1,2,3],[4,5,6],[7,8,9]]);
    let n = Matrix::from([[5,3,5],[7,3,1],[8,8,8]]);
    assert_eq!(format!("{}", (&m * &n).transpose()), format!("{}", n.transpose() * m.transpose()));
}

#[test]
fn dot_product() {

    // transpose of a product equals a reverse product of transposes
    let m = Matrix::from([[1,3,5,6]]);
    let n = Matrix::from([[6,5,4,3]]);
    assert_eq!(m.dot(&n), 59);
}

#[test]
#[should_panic="Dot operation only allowed on vectors of the same length. Vector A length=3, Vector B length=2"]
fn dot_product_different_sizes() {

    // transpose of a product equals a reverse product of transposes
    let m = Matrix::from([[1,2,3]]);
    let n = Matrix::from([[1,2]]);
    assert_eq!(m.dot(&n), 59);
}

#[test]
#[should_panic="Dot operation only allowed on vectors! Matrix A has dim=2, Matrix B has dim=2"]
fn dot_product_matrix() {

    // transpose of a product equals a reverse product of transposes
    let m = Matrix::from([[1,2,3],[1,2,3]]);
    let n = Matrix::from([[1,2,3],[1,2,3]]);
    assert_eq!(m.dot(&n), 59);
}

#[test]
fn norm2_of_vector() {
    let m = Matrix::from([[-4,-3,-2,-1,0,1,2,3,4]]);
    assert_eq!(format!("{:.12}", m.norm()), format!("{}", 7.745966692415f64));
}

#[test]
fn norm_frobenius_of_matrix() {
    let m = Matrix::from([[-4,-3,-2],[-1,0,1],[2,3,4]]);
    assert_eq!(format!("{:.12}", m.norm()), format!("{}", 7.745966692415f64));
}

#[test]
fn mul_scalar() {
    let m = Matrix::from([[-4,-3,-2],[-1,0,1],[2,3,4]]);
    assert_eq!(format!("{}", m * 2), "-8,-6,-4\n-2,0,2\n4,6,8");
}

#[test]
fn cut() {
    let m = Matrix::from([[-4,-3,-2],[-1,0,1],[2,3,4]]);
    let n = Matrix::from([[-1,0,1],[2,3,4]]);
    let c = m.cut(1..,..=2);
    assert_eq!(format!("{}", n), format!("{}", c));
}

#[test]
fn cut_2() {
    let a = Matrix::from(
        [[-1.,-1.,1.],
         [1.,3.,3.],
         [-1.,-1.,5.],
         [1.,3.,7.]]);
    let y = a.cut(.., 0..1);
    let b = Matrix::from([[-1.,1.,-1.,1.]]).transpose();
    assert_eq!(format!("{}", y), format!("{}", b));
}

#[test]
#[should_panic(expected = "Column index out of bounds: [3, 1] / [0, 2]")]
fn cut_bad_index() {
    let m = Matrix::from([[-4,-3,-2],[-1,0,1],[2,3,4]]);
    let _ = m.cut(..,3..2);
}

#[test]
fn set() {
    let mut m = Matrix::from([[-4,-3,-2],[-1,0,1],[2,3,4]]);
    m.set(1, 0, 3);
    let n = Matrix::from([[-4,-3,-2],[3,0,1],[2,3,4]]);
    assert_eq!(format!("{}", m), format!("{}", n));

}

#[test]
fn set_matrix() {
    let mut m = Matrix::from([[-4,-3,-2,-1],[-1,0,1,2],[2,3,4,5]]);
    let n = Matrix::from([[9,8],[17,12]]);
    m.set_matrix(&n, 1, 2);

    let result = Matrix::from([[-4,-3,-2,-1],[-1,0,9,8],[2,3,17,12]]);
    assert_eq!(format!("{}", m), format!("{}", result));
}

#[test]
fn swap_rows_test() {
    let mut m = Matrix::from([[-4,-3,-2,-1],[-1,0,1,2],[2,3,4,5]]);
    let n: Matrix<i32> = Matrix::from([[-4,-3,-2,-1],[2,3,4,5],[-1,0,1,2],]);
    m.swap_rows(1,2);
    assert_eq!(format!("{}", m), format!("{}", n));
}