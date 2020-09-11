mod linalg;

use linalg::*;

fn main() {
    let v1 = Vector::new(0, 1, 2);
    let v2 = Vector::new(1, 1, 1);
    let v3 = &v1 * &v2;
    println!("{}", v3);

    let mut m = Matrix::diag();
    m[(2, 2)] = 0;
    println!("{}", v1.mul_mat(&m).calc());
}
