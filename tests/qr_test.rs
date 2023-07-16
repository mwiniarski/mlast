use mm::{qr_factorization::*, matrix::*};

fn fix_zeroes(a: &mut Matrix<f64>)
{
    let eps = 0.0000001f64;
    for row in 0..a.height() {
        for col in 0..a.width() {
            if a.get(row,col).abs() < eps {
                a.set(row, col, 0.);
            }
        }
    }
}

#[test]
fn householder_test() {
    let a = Matrix::from(
        [[-1.,-1.,1.],
         [1.,3.,3.],
         [-1.,-1.,5.],
         [1.,3.,7.]]);
    let h1 = Matrix::from(
        [[-0.5, 0.5, -0.5, 0.5],
         [0.5, 0.833, 0.167, -0.167],
         [-0.5, 0.167, 0.833, 0.167],
         [0.5, -0.167, 0.167, 0.833]]);
    println!("{:.3}",householder(&a));
    assert_eq!(format!("{:.2}", householder(&a)), format!("{:.2}", h1));
}


#[test]
fn qr_test_qs() {
    let a = Matrix::from(
        [[-1.,-1.,1.],
         [1.,3.,3.],
         [-1.,-1.,5.],
         [1.,3.,7.]]);

    let result = qr(&a);

    let h0 = Matrix::from(
        [[-0.5,0.5,-0.5,0.5],	
         [0.5,0.83333333,0.16666667,-0.16666667],
         [-0.5,0.16666667,0.83333333,0.16666667],
         [0.5,-0.16666667,0.16666667,0.83333333]]);
    let h1 = Matrix::from(
        [[1.,0.,0.,0.],
         [0.,-0.66666667,-0.33333333,-0.66666667],	
         [0.,-0.33333333,0.93333333,-0.13333333],
         [0.,-0.66666667,-0.13333333,0.73333333]]);
    let h2 = Matrix::from(
        [[1.,0.,0.,0.],
         [0.,1.,0.,0.],
         [0.,0.,-0.8,-0.6],
         [0.,0.,-0.6,0.8]]);

    assert_eq!(format!("{:.2}", result.h[0]), format!("{:.2}", h0));
    assert_eq!(format!("{:.2}", result.h[1]), format!("{:.2}", h1));
    assert_eq!(format!("{:.2}", result.h[2]), format!("{:.2}", h2));
}

#[test]
fn qr_test_r() {
    let a = Matrix::from(
        [[-1.,-1.,1.],
         [1.,3.,3.],
         [-1.,-1.,5.],
         [1.,3.,7.]]);
    
    let mut result = qr(&a);

    let r = Matrix::from(
        [[2.,4.,2.],
         [0.,-2.,-8.],
         [0.,0.,-4.],
         [0.,0.,0.]]);

    fix_zeroes(&mut result.r);
    assert_eq!(format!("{:.2}", result.r), format!("{:.2}", r));
}

#[test]
fn qr_test_wide_matrix()
{
    let a = Matrix::from([[1.,2.,3.]]);
    let q = Matrix::from([[-1.]]);
    let r = Matrix::from([[-1.,-2.,-3.]]);
    let res = qr(&a);
    assert_eq!(format!("{:.2}", res.r), format!("{:.2}", r));

    assert_eq!(res.h.len(), 1);
    assert_eq!(format!("{:.2}", res.h[0]), format!("{:.2}", q));
}

#[test]
fn qr_test_tall_matrix()
{
    let a = Matrix::from([[1.,4.,8.]]).transpose();
    let r = Matrix::from([[-9.,0.,0.]]).transpose();

    let mut res = qr(&a);
    fix_zeroes(&mut res.r);
    assert_eq!(format!("{:.2}", res.r), format!("{:.2}", r));

    assert_eq!(res.h.len(), 1);
    assert_eq!(format!("{:.5}", res.h[0].get(2,2)), format!("{:.5}",0.2888888));
}

#[test]
fn compute_q()
{
    let mut hs = vec![];
    hs.push(Matrix::from(
        [[-0.5,0.5,-0.5,0.5],	
         [0.5,0.83333333,0.16666667,-0.16666667],
         [-0.5,0.16666667,0.83333333,0.16666667],
         [0.5,-0.16666667,0.16666667,0.83333333]]));
    hs.push(Matrix::from(
        [[1.,0.,0.,0.],
         [0.,-0.66666667,-0.33333333,-0.66666667],	
         [0.,-0.33333333,0.93333333,-0.13333333],
         [0.,-0.66666667,-0.13333333,0.73333333]]));
    hs.push(Matrix::from(
        [[1.,0.,0.,0.],
         [0.,1.,0.,0.],
         [0.,0.,-0.8,-0.6],
         [0.,0.,-0.6,0.8]]));

    let q = Matrix::from(
        [[-0.5,-0.5,0.5,0.5],
         [0.5,-0.5,0.5,-0.5],
         [-0.5,-0.5,-0.5,-0.5],
         [0.5,-0.5,-0.5,0.5]]);

    assert_eq!(format!("{:.2}", q_from_reflectors(&hs)), format!("{:.2}", q));
}