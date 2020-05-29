/*
use std::fmt::{Display, Error, Formatter};
use std::result::{Result};
use libnum::ToPrimitive;
*/

use super::Matrix;

impl Matrix {
    pub fn default(m: usize, n: usize) -> Self {
        Self {
            data: vec![vec![0.0f64; n]; m],
            m,
            n,
        }
    }
}

impl Clone for Matrix {
    fn clone(&self) -> Matrix {
        Matrix {
            data: self.data.clone(),
            m: self.m,
            n: self.n,
        }
    }
}
