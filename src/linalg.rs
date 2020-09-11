use std::ops;
use std::fmt;

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Vector([i32; 3]);

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Matrix([i32; 9]);

pub struct PartialVector(Vector);

impl PartialVector {
    fn new(init: &Vector) -> Self {
        Self(init.clone())
    }

    pub fn calc(self) -> Vector {
        self.0
    }

    pub fn add(mut self, other: &Vector) -> Self {
        for i in 1..=3 {
            self.0.0[i] += other.0[i];
        }
        self
    }

    pub fn mul_vec(mut self, other: &Vector) -> Self {
        for i in 0..3 {
            self.0.0[i] *= other.0[i];
        }
        self
    }

    pub fn mul(mut self, scalar: i32) -> Self {
        for i in 0..3 {
            self.0.0[i] *= scalar;
        }
        self
    }

    pub fn mul_mat(self, matrix: &Matrix) -> Self {
        let mut res = Vector::zeros();
        for r in 1..=3 {
            for c in 1..=3 {
                res.0[r-1] += matrix[(r, c)] * self.0.0[c-1];
            }
        }
        Self(res)
    }
}

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

    pub fn math(&self) -> PartialVector {
        PartialVector::new(self)
    }

    pub fn add(&self, other: &Vector) -> PartialVector {
        self.math().add(other)
    }

    pub fn mul_vec(&self, other: &Vector) -> PartialVector {
        self.math().mul_vec(other)
    }

    pub fn mul(&self, scalar: i32) -> PartialVector {
        self.math().mul(scalar)
    }

    pub fn mul_mat(&self, matrix: &Matrix) -> PartialVector {
        self.math().mul_mat(matrix)
    }
}

impl ops::Add for &Vector {
    type Output = Vector;
    fn add(self, other: Self) -> Self::Output {
        self.add(other).calc()
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

impl Matrix {
    pub fn zeros() -> Self {
        Self([0; 9])
    }

    pub fn diag() -> Self {
        Self([1, 0, 0, 0, 1, 0, 0, 0, 1])
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
        other.mul_mat(&self).calc()
    }
}
