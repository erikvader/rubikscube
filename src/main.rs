mod linalg;
mod cube;

use linalg::*;
use cube::*;

fn main() {
    let mut cube = Cube::new();
    cube.turns("F D L2 B2 D' B U2 R' F2 L B' R' F' L2 U R2 D' F' D F D' F D2 L2 U'").unwrap();
    cube.print_ascii();
}
