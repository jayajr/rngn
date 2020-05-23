mod impl_ops;
mod impl_vector;

#[derive(Debug, PartialEq, PartialOrd, Copy)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}
