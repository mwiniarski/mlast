use mm::matrix::Matrix;

pub fn fix_zeroes(mut a: Matrix<f64>) -> Matrix<f64>
{
    let eps = 0.0000001f64;
    for row in 0..a.height() {
        for col in 0..a.width() {
            if a.get(row,col).abs() < eps {
                a.set(row, col, 0.);
            }
        }
    }

    a
}