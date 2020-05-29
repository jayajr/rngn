use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
    Rem, RemAssign,
};
use libnum::ToPrimitive;

use super::Matrix;

impl Add for Matrix {
    type Output = Option<Matrix>;
    fn add(self, mat: Matrix) -> Option<Matrix> {
        match 
               self.m == mat.m
            && self.n == mat.n {
                true => {
                    let (m,n) = (self.m, self.n);
                    let mut retmat = Matrix::default(m, n);
                    for i in 0..m {
                        for j in 0..n {
                            retmat.data[i][j] = self.data[i][j] + mat.data[i][j];
                        }
                    }
                    Some(retmat)
                }
                false => None,
        }
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, rhs: Matrix) {
        match 
               self.m == rhs.m
            && self.n == rhs.n {
                true => {
                    let (m,n) = (self.m, self.n);
                    for i in 0..m {
                        for j in 0..n {
                            self.data[i][j] += rhs.data[i][j];
                        }
                    }
                }
                false => (),
        }
    }
}

/*
impl Sub for Matrix {
    type Output = Matrix;
    fn sub(self, vec: Matrix) -> Matrix {
        Matrix {
            x: self.x - vec.x,
            y: self.y - vec.y,
            z: self.z - vec.z,
        }
    }
}

impl SubAssign for Matrix {
    fn sub_assign(&mut self, rhs: Matrix) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Matrix {
    type Output = f64;
    fn mul(self, rhs: Matrix) -> f64 {
        self.dot(rhs)
    }
}

impl Rem for Matrix {
    type Output = Matrix;
    fn rem(self, rhs: Matrix) -> Matrix {
        self.cross(rhs)
    }
}

impl RemAssign for Matrix {
    fn rem_assign(&mut self, rhs: Matrix) {
        let (x, y, z) = self.cross(rhs).as_tuple();
        self.x = x;
        self.y = y;
        self.z = z;
    }
}
impl<T: ToPrimitive> Mul<T> for Matrix {
    type Output = Matrix;
    fn mul(self, scalar: T) -> Matrix {
        let s = scalar.to_f64().unwrap();
        Matrix {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

impl<T: ToPrimitive> MulAssign<T> for Matrix {
    fn mul_assign(&mut self, rhs: T) {
        let s = rhs.to_f64().unwrap();
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }
}

impl<T: ToPrimitive> Div<T> for Matrix {
    type Output = Matrix;
    fn div(self, scalar: T) -> Matrix {
        let s = scalar.to_f64().unwrap();
        Matrix {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
        }
    }
}

impl<T: ToPrimitive> DivAssign<T> for Matrix {
    fn div_assign(&mut self, rhs: T) {
        let s = rhs.to_f64().unwrap();
        self.x /= s;
        self.y /= s;
        self.z /= s;
    }
}
*/
