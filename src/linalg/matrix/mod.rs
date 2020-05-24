mod impl_ops;
mod impl_matrix;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Matrix {
    // data: &[T], Lifetimes??
    m: usize,
    n: usize,

}

