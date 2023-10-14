use crate::matrix::*;

pub struct LUResult {
    pub l: Matrix<f64>,
    pub u: Matrix<f64>,
    pub p: Matrix<f64>
}

pub fn lu_gauss(a: &Matrix<f64>) -> LUResult {

    // Setup L - sqare of size of height of A
    let mut l: Matrix<f64> = Matrix::new_fill(a.height(), a.height(), 0.);

    // Setup U - same size as A, is our working matrix
    let mut u: Matrix<f64> = a.clone();

    // Setup P - permutation matrix
    let mut p: Matrix<f64> = Matrix::identity(a.height());

    // Each row is the basis for Gaussian Elimination
    let iterations = u.width().min(u.height());
    for row in 0..iterations {

        // Find the row to pivot to the top - with highest element in the column
        let mut biggest = (0, 0.);
        for pivot_row in row..u.height() {
            let val = &u.get(pivot_row, row);
            if val.abs() > biggest.1 {
                biggest = (pivot_row, val.abs());
            }
        }

        // If there are only "0s" in column, skip it
        if biggest.1 <= f64::EPSILON {
            continue;
        }

        // Swap two rows and save the permutation
        u.swap_rows(row, biggest.0);
        p.swap_rows(row, biggest.0);
        l.swap_rows(row, biggest.0);

        // Grab the first number in the row (which is on diagonal because all prior are 0)
        let pivot = u.get(row, row);

        for row_index in (row + 1)..u.height() {

            // Compute quotient between pivot and every number in the column below it
            let quotient = u.get(row_index, row) / pivot;

            // Save it to L in the same position as in U
            l.set(row_index, row, quotient);

            // Substract the whole row above * quotient from current row
            for col_index in row..u.width() {
                u.set(row_index, col_index, u.get(row_index, col_index) - quotient * u.get(row, col_index));
            }
        }
    }

    LUResult { l: Matrix::identity(l.height()) + l, u: u, p: p }
}

pub fn lu_solve(lu: &LUResult, b: &Matrix<f64>) -> Matrix<f64> {
    if b.width() != 1 {
        panic!("b must be in form of a column vector, b=[{},{}]", b.height(), b.height());
    }

    // PA = LU, Ax = b, so LUx = Pb. If y = Ux, then Ly = Pb
    // Step 1. Solve Ly = Pb for y using forward substitution
    let mut y = Matrix::new_fill(b.height(), b.width(), 0.);
    let b = &lu.p * b;

    for row in 0..y.height() {
        let mut new_y = b.get(row, 0);

        for col in 0..row {
            new_y -= y.get(col, 0) * lu.l.get(row, col); 
        }

        y.set(row, 0, new_y);
    }

    // Step 2. Solve Ux = y for x using backward substitution
    let mut x = Matrix::new_fill(y.height(), y.width(), 0.);

    for row in (0..y.height()).rev() {
        let mut new_x = y.get(row, 0);

        for col in ((row + 1)..y.height()).rev() {
            new_x -= x.get(col, 0) * lu.u.get(row, col); 
        }

        new_x /= lu.u.get(row, row);

        x.set(row, 0, new_x);
    }

    x
}

pub fn inv(a: &Matrix<f64>) -> Matrix<f64> {
    if a.height() != a.width() {
        panic!("Cannot inverse a non-square matrix! A=[{},{}]", a.height(), a.width());
    }

    let lu = lu_gauss(&a);
    let mut res: Matrix<f64> = Matrix::identity(a.height());

    for col in 0..a.width() {
        res.set_matrix(&lu_solve(&lu, &res.cut(.., col..col+1)), 0, col);
    }

    res
}