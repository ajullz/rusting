/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {

    assert!(!mat1.is_empty(), "mat1 is empty");
    assert!(!mat2.is_empty(), "mat2 is empty");

    let rows_1 = mat1.len();
    let cols_1 = mat1[0].len();
    let rows_2 = mat2.len();
    let cols_2 = mat2[0].len();

    assert_eq!(cols_1, rows_2, "Matrix dimensions do not match");

    let mut result: Matrix = vec![vec![0f32; cols_2]; rows_1];

    for r in 0..rows_1 {
        for c in 0..cols_2 {
            for k in 0..cols_1 {
                result[r][c] += mat1[r][k] * mat2[k][c];
            }
        }
    }
    result
}