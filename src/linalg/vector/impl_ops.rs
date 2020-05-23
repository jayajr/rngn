use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
    Rem, RemAssign,
};
use libnum::ToPrimitive;

use super::Vector;

impl Vector {
    pub fn dot(self, b: Vector) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z 
    }

    pub fn cross(self, b: Vector) -> Vector {
        Vector {
            x: self.y * b.z - self.z * b.y,
            y: self.z * b.x - self.x * b.z,
            z: self.x * b.y - self.y * b.x,
        }
    }

    pub fn magnitude(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn unit(self) -> Vector {
        let mag = self.magnitude();
        Vector {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, vec: Vector) -> Vector {
        Vector {
            x: self.x + vec.x,
            y: self.y + vec.y,
            z: self.z + vec.z,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, vec: Vector) -> Vector {
        Vector {
            x: self.x - vec.x,
            y: self.y - vec.y,
            z: self.z - vec.z,
        }
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Vector {
    type Output = f64;
    fn mul(self, rhs: Vector) -> f64 {
        self.dot(rhs)
    }
}

impl Rem for Vector {
    type Output = Vector;
    fn rem(self, rhs: Vector) -> Vector {
        self.cross(rhs)
    }
}

impl RemAssign for Vector {
    fn rem_assign(&mut self, rhs: Vector) {
        let (x, y, z) = self.cross(rhs).as_tuple();
        self.x = x;
        self.y = y;
        self.z = z;
    }
}
impl<T: ToPrimitive> Mul<T> for Vector {
    type Output = Vector;
    fn mul(self, scalar: T) -> Vector {
        let s = scalar.to_f64().unwrap();
        Vector {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

impl<T: ToPrimitive> MulAssign<T> for Vector {
    fn mul_assign(&mut self, rhs: T) {
        let s = rhs.to_f64().unwrap();
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }
}

impl<T: ToPrimitive> Div<T> for Vector {
    type Output = Vector;
    fn div(self, scalar: T) -> Vector {
        let s = scalar.to_f64().unwrap();
        Vector {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
        }
    }
}

impl<T: ToPrimitive> DivAssign<T> for Vector {
    fn div_assign(&mut self, rhs: T) {
        let s = rhs.to_f64().unwrap();
        self.x /= s;
        self.y /= s;
        self.z /= s;
    }
}



#[test]
fn eq_ord_test() {
    let a = Vector::new(1.0, 1.1, 1.0);
    let a2 = a.clone();
    let b = Vector::new(2.0, 2.0, 2.0);

    assert!(a < b);
    assert!(a <= b);
    assert!(a != b);
    assert!(a == a2);
}

#[test]
fn add_sub_test() {
    let mut a = Vector::new(0, 3, 4);
    let b = Vector::new(4, 1, 0);
    let c = Vector::new(4, 4, 4);

    assert!(a + b == c);
    assert!(c - a == b);

    let d = a.clone();
    a += b;
    assert!(a == c);
    a -= b;
    assert!(a == d)
}

#[test]
fn mul_div_test() {
    let mut a = Vector::new(0, 3, 4);
    let s = 3;
    let b = Vector::new(0, 9, 12);

    assert!(a * s == b);
    assert!(b / s == a);

    let c = a.clone();
    a *= s;
    assert!(a == b);
    a /= s;
    assert!(a == c)

}

#[test]
fn dot_test() {
    let e0 = Vector::new(1,0,0);
    let e1 = Vector::new(0,1,0);
    let e2 = Vector::new(0,0,1);
    let e3 = Vector::new(1,1,1);

    assert!(e0.dot(e1) == 0.0);
    assert!(e2.dot(e3) == 1.0);
    assert!(e3.dot(e3) == 3.0);

    assert!(e0*e1 == 0.0);
    assert!(e2*e3 == 1.0);
    assert!(e3*e3 == 3.0);
}

#[test]
fn cross_test() {
    let mut a = Vector::new(2,3,1);
    let b = Vector::new(1,2,0);
    let c = Vector::new(-2,1,1);

    assert!(a.cross(b) == c);
    assert!(a % b == c);

    a %= b;
    assert!(a == c);
}

#[test]
fn magnitude_test() {
    let a = Vector::new(0,1,1);
    assert!(a.magnitude() == (2.0 as f64).sqrt());

    let b = Vector::new(3,0,4);
    assert!(b.magnitude() == 5.0);

    let c = Vector::new(5, 12, 0);
    assert!(c.magnitude() == 13.0);
}

#[test]
fn unit_test() {
    let a = Vector::new(0, 0, 1);
    assert!(a.unit() == a);

    let b = Vector::new(1, 1, 0);

    let rad2 = (2.0 as f64).sqrt();
    let allowed_error = 1e-10;
    assert!(b.unit() - Vector::new(rad2, rad2, 0) < Vector::new(allowed_error, allowed_error, allowed_error));
}