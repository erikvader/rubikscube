mod linalg;
mod cube;

use cube::*;

fn main() {
    let mut cube = Cube::new();
    cube.turns("F D Y L2 B2 D' X' B U2 R' F2 Z L B' R' Z' F' L2 U X' Y R2 D' F' X' X' D F D' Y F D2 L2 U' X").unwrap();
    cube.print_ascii();
}
