extern crate num;

use std::fmt::{Display, Error, Formatter};
use std::result::{Result};

use super::Vector;

impl Vector {
    pub fn new<T: num::ToPrimitive>(
        x: T,
        y: T,
        z: T,
    ) -> Self {
        let (a, b, c) = (
            x.to_f64().expect("Requires numeric type"),
            y.to_f64().expect("Requires numeric type"),
            z.to_f64().expect("Requires numeric type"),
        );

        Vector {
            x: a,
            y: b,
            z: c,
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
