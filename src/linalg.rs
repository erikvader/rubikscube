use std::ops;
use std::fmt;
use std::mem::{self, MaybeUninit};

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Vector([i32; 3]);

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Matrix([i32; 9]);

impl Vector {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self([x, y, z])
    }

    pub fn zeros() -> Self {
        Self([0; 3])
    }

    #[inline]
    pub fn x(&self) -> i32 {
        self.0[0]
    }

    #[inline]
    pub fn y(&self) -> i32 {
        self.0[1]
    }

    #[inline]
    pub fn z(&self) -> i32 {
        self.0[2]
    }

    pub fn add_vec(&mut self, other: &Vector) -> &mut Self {
        for i in 0..3 {
            self.0[i] += other.0[i];
        }
        self
    }

    pub fn mul_vec(&mut self, other: &Vector) -> &mut Self {
        for i in 0..3 {
            self.0[i] *= other.0[i];
        }
        self
    }

    pub fn mul_scal(&mut self, scalar: i32) -> &mut Self {
        for i in 0..3 {
            self.0[i] *= scalar;
        }
        self
    }

    pub fn components(&self) -> [Vector; 3] {
        [
            Vector::new(self[1], 0, 0),
            Vector::new(0, self[2], 0),
            Vector::new(0, 0, self[3])
        ]
    }

    pub fn components2(&self) -> [Vector; 2] {
        let mut i = 0;
        let mut res: [MaybeUninit<Vector>; 2] = unsafe{MaybeUninit::uninit().assume_init()};

        for j in 1..=3 {
            if self[j] != 0 {
                if i >= 2 {
                    panic!("too many non-zero components");
                }
                let mut v = Vector::zeros();
                v[j] = self[j];
                res[i] = MaybeUninit::new(v);
                i += 1;
            }
        }

        if i < 2 {
            panic!("too few non-zero components");
        }

        unsafe{mem::transmute::<_, [Vector; 2]>(res)}
    }
}

impl ops::Add for &Vector {
    type Output = Vector;
    fn add(self, other: Self) -> Self::Output {
        let mut r = self.clone();
        r.add_vec(other);
        r
    }
}

impl ops::Mul for &Vector {
    type Output = i32;
    fn mul(self, other: Self) -> Self::Output {
        self.x() * other.x() +
        self.y() * other.y() +
        self.z() * other.z()
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x(), self.y(), self.z())
    }
}

impl ops::Index<usize> for Vector {
    type Output = i32;
    fn index(&self, rc: usize) -> &Self::Output {
        &self.0[rc-1]
    }
}

impl ops::IndexMut<usize> for Vector {
    fn index_mut(&mut self, rc: usize) -> &mut Self::Output {
        &mut self.0[rc-1]
    }
}

impl From<&[i32; 3]> for Vector {
    fn from(a: &[i32; 3]) -> Self {
        Vector::new(a[0], a[1], a[2])
    }
}

impl std::cmp::PartialEq<[i32; 3]> for Vector {
    fn eq(&self, other: &[i32; 3]) -> bool {
        &self.0 == other
    }
}

impl std::convert::AsRef<[i32; 3]> for Vector {
    fn as_ref(&self) -> &[i32; 3] {
        &self.0
    }
}

impl Matrix {
    pub fn zeros() -> Self {
        Self([0; 9])
    }

    pub fn diag() -> Self {
        Self([1, 0, 0, 0, 1, 0, 0, 0, 1])
    }

    pub fn rotation_x(cw: bool) -> Self {
        let s = if cw {-1} else {1};
        Self([
            1, 0, 0,
            0, 0, -s,
            0, s, 0
        ])
    }

    pub fn rotation_y(cw: bool) -> Self {
        let s = if cw {-1} else {1};
        Self([
            0, 0, s,
            0, 1, 0,
            -s, 0, 0
        ])
    }

    pub fn rotation_z(cw: bool) -> Self {
        let s = if cw {-1} else {1};
        Self([
            0, -s, 0,
            s, 0, 0,
            0, 0, 1
        ])
    }

    pub fn mul_mat(&self, other: &Self) -> Matrix {
        let mut res = Self::zeros();
        for i in 1..=3 {
            for j in 1..=3 {
                for k in 1..=3 {
                    res[(i, j)] += self[(i, k)] * other[(k, j)];
                }
            }
        }
        res
    }

    pub fn mul_vec(&self, vec: &Vector) -> Vector {
        let mut res = Vector::zeros();
        for r in 1..=3 {
            for c in 1..=3 {
                res[r] += self[(r, c)] * vec[c];
            }
        }
        res
    }
}

impl ops::Index<(usize, usize)> for Matrix {
    type Output = i32;
    fn index(&self, rc: (usize, usize)) -> &Self::Output {
        &self.0[(rc.0-1)*3 + rc.1-1]
    }
}

impl ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, rc: (usize, usize)) -> &mut Self::Output {
        &mut self.0[(rc.0-1)*3 + rc.1-1]
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 1..=3 {
            write!(f, "|{}, {}, {}|", self[(r,1)], self[(r,2)], self[(r,3)])?;
            if r < 3 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

impl ops::Mul<&Vector> for &Matrix {
    type Output = Vector;
    fn mul(self, other: &Vector) -> Self::Output {
        self.mul_vec(other)
    }
}

impl ops::Mul for &Matrix {
    type Output = Matrix;
    fn mul(self, other: &Matrix) -> Self::Output {
        self.mul_mat(other)
    }
}
