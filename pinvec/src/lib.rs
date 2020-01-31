
#[cfg(test)]
mod tests;

extern crate pinbuf;
extern crate pow_of_2;

use std::usize;
use pinbuf::PinBuffer;
use pow_of_2::{PowOf2, Two};

pub struct PinVec<T> {
    bufs: Vec<PinBuffer<T>>,
    buf_0_len: PowOf2<usize>,
    len: usize,
}

impl<T> PinVec<T> {
    pub fn new(buf_0_len: PowOf2<usize>) -> Self {
        PinVec {
            bufs: Vec::new(),
            buf_0_len,
            len: 0,
        }
    }
    
    /// Compute the outer and inner indices where an 
    /// element would go, by element index.
    fn calc_index(&self, elem_i: usize) -> (usize, usize) {
        let buf_unit_i = elem_i >> self.buf_0_len.exp();
        let outer = {
            let mut t = 1;
            while (buf_unit_i + 1) >= (1 << t) {
                t += 1;
            }
            t - 1
        };
        let inner = elem_i - (
            (1 << outer << self.buf_0_len.exp()) 
                - self.buf_0_len.to_uint());
        (outer, inner)
    }
}
