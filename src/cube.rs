use crate::linalg::*;
use std::mem::{self, MaybeUninit};

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum Color {Yellow, Red, Blue, White, Orange, Green}

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum Direction {CW, CCW}

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum Face {Front, Back, Left, Right, Up, Down}

#[derive(Debug)]
pub enum Piece {
    Center (Vector, Color),
    Edge ([Vector; 2], [Color; 2]),
    Corner ([Vector; 3], [Color; 3]),
    Middle,
}

#[derive(Debug)]
pub struct Cube {
    pieces: [Piece; 27],
    rotation: Matrix,
}

#[derive(Debug)]
pub struct CubeFace {
    squares: [Color; 9],
}

impl std::ops::Index<(usize, usize)> for CubeFace {
    type Output = Color;
    fn index(&self, i: (usize, usize)) -> &Self::Output {
        &self.squares[i.0 + i.1*3]
    }
}

impl CubeFace {
    fn white() -> Self {
        Self{squares: [Color::White; 9]}
    }

    fn set(&mut self, c: (usize, usize), col: Color) {
        self.squares[c.0 + c.1*3] = col;
    }
}

impl std::ops::Not for Direction {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::CW => Self::CCW,
            Self::CCW => Self::CW,
        }
    }
}

impl Face {
    fn direction(&self) -> Vector {
        match self {
            Self::Front => Vector::new(0, -1, 0),
            Self::Back  => Vector::new(0, 1, 0),
            Self::Left  => Vector::new(-1, 0, 0),
            Self::Right => Vector::new(1, 0, 0),
            Self::Up    => Vector::new(0, 0, 1),
            Self::Down  => Vector::new(0, 0, -1),
        }
    }

    fn from_direction(v: &Vector) -> Option<Self> {
        match v.as_ref() {
            [0, -1, 0] => Some(Face::Front),
            [0, 1, 0]  => Some(Face::Back),
            [-1, 0, 0] => Some(Face::Left),
            [1, 0, 0]  => Some(Face::Right),
            [0, 0, 1]  => Some(Face::Up),
            [0, 0, -1] => Some(Face::Down),
            _ => None
        }
    }

    fn initial_color(&self) -> Color {
        match self {
            Self::Front => Color::Green,
            Self::Back  => Color::Blue,
            Self::Left  => Color::Orange,
            Self::Right => Color::Red,
            Self::Up    => Color::White,
            Self::Down  => Color::Yellow,
        }
    }

    fn iter() -> impl Iterator<Item=Self> {
        vec![Self::Front, Self::Back, Self::Left, Self::Right, Self::Up, Self::Down]
            .into_iter()
    }

    fn rotation_matrix(&self, dir: Direction) -> Matrix {
        let cw = dir == Direction::CW;
        match self {
            Self::Front => Matrix::rotation_y(!cw),
            Self::Back  => Matrix::rotation_y(cw),
            Self::Left  => Matrix::rotation_x(!cw),
            Self::Right => Matrix::rotation_x(cw),
            Self::Down  => Matrix::rotation_z(!cw),
            Self::Up    => Matrix::rotation_z(cw),
        }
    }

    fn edge_orth(&self) -> Vector {
        match self {
            Self::Front | Self::Back | Self::Left | Self::Right => Vector::new(0, 0, 1),
            Self::Up => Vector::new(0, 1, 0),
            Self::Down => Vector::new(0, -1, 0),
        }
    }

    fn corner_orth(&self) -> Vector {
        match self {
            Self::Front => Vector::new(-1, 0, 1),
            Self::Right => Vector::new(0, -1, 1),
            Self::Left  => Vector::new(0, 1, 1),
            Self::Back  => Vector::new(1, 0, 1),
            Self::Up    => Vector::new(-1, 1, 0),
            Self::Down  => Vector::new(-1, -1, 0),
        }
    }
}

impl Color {
    pub fn short(&self) -> &'static str {
        match self {
            Self::Yellow => "y",
            Self::Red    => "r",
            Self::Blue   => "b",
            Self::Green  => "g",
            Self::White  => "w",
            Self::Orange => "o",
        }
    }
}

impl Piece {
    fn coordinate(&self) -> Vector {
        match self {
            Self::Middle => Vector::new(0, 0, 0),
            Self::Center(v, _) => v.clone(),
            Self::Edge([v1, v2], _) => v1 + v2,
            Self::Corner([v1, v2, v3], _) => {
                let mut r = v1.clone();
                r.add_vec(v2).add_vec(v3);
                r
            }
        }
    }

    fn color(&self, dir: &Vector) -> Option<Color> {
        match self {
            Self::Center(v, c) if dir == v => Some(*c),
            Self::Edge([v, _], [c, _]) if dir == v => Some(*c),
            Self::Edge([_, v], [_, c]) if dir == v => Some(*c),
            Self::Corner([v, _, _], [c, _, _]) if dir == v => Some(*c),
            Self::Corner([_, v, _], [_, c, _]) if dir == v => Some(*c),
            Self::Corner([_, _, v], [_, _, c]) if dir == v => Some(*c),
            _ => None,
        }
    }

    fn rotate(&mut self, rot: &Matrix) {
        match self {
            Self::Middle => (),
            Self::Center(v, _) => *v = rot * &*v,
            Self::Edge([v1, v2], _) => {
                *v1 = rot * &*v1;
                *v2 = rot * &*v2;
            },
            Self::Corner([v1, v2, v3], _) => {
                *v1 = rot * &*v1;
                *v2 = rot * &*v2;
                *v3 = rot * &*v3;
            }
        }
    }
}

impl Cube {
    pub fn new() -> Self {
        let mut p: [MaybeUninit<Piece>; 27] = unsafe {MaybeUninit::uninit().assume_init()};
        let mut b = [false; 27];

        for pie in Self::generate_pieces().into_iter() {
            let c = Self::piece_index(&pie.coordinate());
            p[c] = MaybeUninit::new(pie);
            b[c] = true;
        }

        if !b.iter().all(|x| *x) {
            panic!("pieces was not all initialized!");
        }

        Self{
            pieces: unsafe{mem::transmute::<_, [Piece; 27]>(p)},
            rotation: Matrix::diag(),
        }
    }

    fn generate_pieces() -> Vec<Piece> {
        let mut res = Vec::with_capacity(27);
        res.push(Piece::Middle);

        // fill centers
        for f in Face::iter() {
            res.push(Piece::Center(f.direction(), f.initial_color()));
        }

        // fill edges
        let trans = Matrix::rotation_x(true);
        for x in &[-1, 1, 0] {
            let mut rotator = Vector::new(*x, if *x == 0 {1} else {0}, 1);
            for _ in 0..4 {
                rotator = &trans * &rotator;
                let comp = rotator.components2();
                let colors = [
                    Face::from_direction(&comp[0]).unwrap().initial_color(),
                    Face::from_direction(&comp[1]).unwrap().initial_color()
                ];
                res.push(Piece::Edge(comp, colors));
            }
        }

        // fill corners
        for x in &[-1, 1] {
            let mut rotator = Vector::new(*x, 1, 1);
            for _ in 0..4 {
                rotator = &trans * &rotator;
                let comp = rotator.components();
                let colors = [
                    Face::from_direction(&comp[0]).unwrap().initial_color(),
                    Face::from_direction(&comp[1]).unwrap().initial_color(),
                    Face::from_direction(&comp[2]).unwrap().initial_color()
                ];
                res.push(Piece::Corner(comp, colors));
            }
        }

        res
    }

    fn piece_index(v: &Vector) -> usize {
        ((v.x()+1)*9 + (v.y()+1)*3 + (v.z()+1)) as usize
    }

    pub fn face(&self, f: Face) -> CubeFace {
        let mut cf = CubeFace::white();

        let dir = f.direction();
        let rot = f.rotation_matrix(Direction::CW);

        let mut corner = f.corner_orth();
        corner.add_vec(&dir);
        for c in &[(0, 0), (2, 0), (2, 2), (0, 2)] {
            cf.set(*c, self[&corner].color(&dir).unwrap());
            corner = &rot * &corner;
        }

        let mut edge = f.edge_orth();
        edge.add_vec(&dir);
        for c in &[(1, 0), (2, 1), (1, 2), (0, 1)] {
            cf.set(*c, self[&edge].color(&dir).unwrap());
            edge = &rot * &edge;
        }

        cf.set((1, 1), self[&dir].color(&dir).unwrap());

        cf
    }

    pub fn print_ascii(&self) {
        fn row(f: &CubeFace, r: usize, i: usize) {
            print!(
                "{:<1$}{2} {3} {4}",
                "",
                i,
                f[(0, r)].short(),
                f[(1, r)].short(),
                f[(2, r)].short()
            );
        }

        let up = self.face(Face::Up);
        for i in 0..3 {
            row(&up, i, 8);
            println!("");
        }
        println!("{:<8}{:-<5}", "", "");

        let left = self.face(Face::Left);
        let front = self.face(Face::Front);
        let right = self.face(Face::Right);
        let back = self.face(Face::Back);

        for i in 0..3 {
            row(&left, i, 0);
            print!(" | ");
            row(&front, i, 0);
            print!(" | ");
            row(&right, i, 0);
            print!(" | ");
            row(&back, i, 0);
            println!("");
        }

        println!("{:<8}{:-<5}", "", "");
        let down = self.face(Face::Down);
        for i in 0..3 {
            row(&down, i, 8);
            println!("");
        }
    }

    pub fn rotate(&mut self, face: Face, dir: Direction) {
        unimplemented!();
    }

    fn swap(&mut self, v1: &Vector, v2: &Vector) {
        let i1 = Self::piece_index(v1);
        let i2 = Self::piece_index(v2);
        self.pieces.swap(i1, i2);
    }

    pub fn turn(&mut self, face: Face, dir: Direction) {
        let base = face.direction();
        let rot = face.rotation_matrix(dir);
        let rotn = face.rotation_matrix(!dir);

        let mut corner1 = face.corner_orth();
        let mut corner2 = &rotn * &corner1;
        let mut edge1 = face.edge_orth();
        let mut edge2 = &rotn * &edge1;
        corner1.add_vec(&base);
        corner2.add_vec(&base);
        edge1.add_vec(&base);
        edge2.add_vec(&base);

        self[&corner1].rotate(&rot);
        self[&edge1].rotate(&rot);

        for _ in 0..3 {
            self[&corner2].rotate(&rot);
            self.swap(&corner1, &corner2);
            corner1 = &rotn * &corner1;
            corner2 = &rotn * &corner2;

            self[&edge2].rotate(&rot);
            self.swap(&edge1, &edge2);
            edge1 = &rotn * &edge1;
            edge2 = &rotn * &edge2;
        }
    }

    pub fn turns(&mut self, turns: &str) -> Result<(), ()> {
        for m in turns.split(" ") {
            if m == "" {
                continue;
            }

            let chars: Vec<char> = m.chars().collect();

            if chars.len() > 2 {
                return Err(());
            }

            let face = match chars[0] {
                'R' => Face::Right,
                'L' => Face::Left,
                'U' => Face::Up,
                'D' => Face::Down,
                'B' => Face::Back,
                'F' => Face::Front,
                _ => return Err(()),
            };

            let dir = if chars.len() == 2 && chars[1] == '\'' {
                Direction::CCW
            }
            else{
                Direction::CW
            };

            if chars.len() == 2 && chars[1] == '2' {
                self.turn(face, dir);
            }
            self.turn(face, dir);
        }
        Ok(())
    }
}

impl std::ops::Index<&Vector> for Cube {
    type Output = Piece;
    fn index(&self, rc: &Vector) -> &Self::Output {
        &self.pieces[Cube::piece_index(rc)]
    }
}

impl std::ops::IndexMut<&Vector> for Cube {
    fn index_mut(&mut self, rc: &Vector) -> &mut Self::Output {
        &mut self.pieces[Cube::piece_index(rc)]
    }
}
