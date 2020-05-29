mod impl_ops;
mod impl_matrix;

/*
  Matrix { m: 2, n: 5 }
        n →
  m [ [ 0 0 0 0 0 ]
  ↓   [ 0 0 0 0 0 ] ]

 */

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Matrix {
    pub data: Vec<Vec<f64>>,
    pub m: usize,
    pub n: usize,
}
