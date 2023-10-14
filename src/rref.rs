use crate::matrix::*;

const EPS: f64 = 1e-12;

pub fn rref(a: &Matrix<f64>) -> Matrix<f64> {
    
    // Setup R - same size as A, is our working matrix
    let mut r: Matrix<f64> = a.clone();

    // Each row is the basis for Gaussian Elimination
    let mut row = 0;
    for col in 0..r.width() {

        // Find the row to pivot to the top - with highest element in the column
        let mut biggest = (0, 0.);
        for pivot_row in row..r.height() {
            let val = &r.get(pivot_row, col);
            if val.abs() > biggest.1 {
                biggest = (pivot_row, val.abs());
            }
        }

        // If there are only "0s" in column, skip it - col is incremented but row is not
        if biggest.1 <= EPS {
            continue;
        }

        // Swap two rows
        r.swap_rows(row, biggest.0);

        // Grab the first number in the row 
        let pivot = r.get(row, col);

        // If pivot is not close to 1, divide whole row by it
        if f64::abs(pivot - 1.) > EPS {
            for col_index in col..r.width() {
                r.set(row, col_index, r.get(row, col_index) / pivot);
            }
        }

        // Substract current row from all other rows
        for row_index in 0..r.height() {
            if row_index == row {
                continue;
            }

            // Quotient between pivot (which is equal 1) and the number in the current column in other row
            let quotient = r.get(row_index, col);

            // Substract the whole row to the right of current column
            for col_index in col..r.width() {
                r.set(row_index, col_index, r.get(row_index, col_index) - quotient * r.get(row, col_index));
            }
        }

        row += 1;
    }

    r
}

pub fn rank(a: &Matrix<f64>) -> usize {

    let reduced = rref(a);
    let mut rank = 0;

    for col in 0..reduced.width() {

        // If we stumble upon 0, don't increase rank
        if reduced.get(rank, col) < EPS {
            continue;
        }

        rank += 1;
    }

    rank
}