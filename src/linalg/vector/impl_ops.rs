use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
};

use super::Vector;

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