mod impl_ops;
mod impl_vector;

#[derive(Debug, PartialEq, PartialOrd, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
