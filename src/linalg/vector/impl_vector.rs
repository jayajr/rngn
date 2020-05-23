use std::fmt::{Display, Error, Formatter};
use std::result::{Result};
use libnum::ToPrimitive;

use super::Vector;

impl Vector {
    pub fn new <T: ToPrimitive, U: ToPrimitive, V: ToPrimitive>(
        x: T,
        y: U,
        z: V,
    ) -> Self {
        let (a, b, c) = (
            x.to_f64().unwrap(),
            y.to_f64().unwrap(),
            z.to_f64().unwrap(),
        );

        Vector {
            x: a,
            y: b,
            z: c,
        }
    }

    pub fn as_array(self) -> [f64;3] {
        [self.x, self.y, self.z]
    }

    pub fn from_array<T: ToPrimitive + Copy>(array: [T;3]) -> Vector {
        Vector::new(array[0], array[1], array[2])
    }

    pub fn as_tuple(self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn from_tuple<T: ToPrimitive, U: ToPrimitive, V:ToPrimitive>(
        tuple: (T, U, V)
    ) -> Vector {
        Vector::new(tuple.0, tuple.1, tuple.2)
    }
}

impl Clone for Vector {
    fn clone(&self) -> Vector {
        Vector {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Display for Vector {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(
            fmt,
            "LinAlgVector::( x: {}, y: {}, z: {} )",
            self.x,
            self.y,
            self.z
        )

    }
}
