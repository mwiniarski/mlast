use crate::matrix::*;

pub struct QRResult<T> {
    pub r: Matrix<T>,
    pub h: Vec<Matrix<T>>
}

/*
 * Computes householder reflector of the full first column of given matrix
 */
pub fn householder(a: &Matrix<f64>) -> Matrix<f64> {

    // TODO: replace with view
    let y = a.cut(.., ..1);

    // Instead of creating a e = (1, 0, ... 0) vector only care about first value
    let y1 = y.get(0,0);
    let y1_sign = if y1 >= 0. { 1. } else { -1. };
    let mut w = y;

    // w = y + sign(y1)||y||e - only the first value is modified
    w.set(0, 0, y1 + y1_sign * w.norm());

    let v = (1./w.norm()) * w;
    let vvt2 = v.clone() * v.transpose() * 2.;
    Matrix::identity(vvt2.height()) - vvt2
}

pub fn qr(a: &Matrix<f64>) -> QRResult<f64> {
    // Make a copy of A
    let mut a = a.clone();

    // Number of reflectors is the lower number between width and height
    let h_count = a.width().min(a.height());

    // The size of reflector (it is square) is matrix height
    let h_size = a.height();

    // Resulting reflectors
    let mut hs: Vec<Matrix<f64>> = vec![];

    for col_index in 0..h_count {

        // Take a piece of A
        let cut = a.cut(col_index.., col_index..);

        // Compute the reflector of first column
        let h = householder(&cut);

        // Save the reflector to return
        let mut full_reflector: Matrix<f64> = Matrix::identity(h_size);
        full_reflector.set_matrix(&h, col_index, col_index);
        hs.push(full_reflector);

        // Apply the reflector to the piece of A
        let r_part = h * cut;

        // Copy the result to A
        a.set_matrix(&r_part, col_index, col_index);
    }

    QRResult { r: a, h: hs }
}

pub fn q_from_reflectors(hs: &Vec<Matrix<f64>>) -> Matrix<f64>
{
    if hs.is_empty() {
        panic!("Can't compute Q from a vector of H reflectors, vector is empty")
    }

    let mut res = hs[0].clone();
    for index in 1..hs.len() {
        res = &res * &hs[index];
    }
    res
}