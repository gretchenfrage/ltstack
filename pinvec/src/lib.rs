
#[cfg(test)]
mod tests;

extern crate pinbuf;
extern crate pow_of_2;

use std::{
    usize,
    pin::Pin,
};
use pinbuf::PinBuffer;
use pow_of_2::PowOf2;

pub struct PinVec<T> {
    buffers: Vec<PinBuffer<T>>,
    buf_0_len: PowOf2<usize>,
    len: usize,
}

/// Assert that an index is in bounds, then make a 
/// call to `$s.calc_index($i)`.
macro_rules! valid_index {
    ($s:expr, $i:expr)=>{{
        if $i >= $s.len() {
            panic!("index {} out of bounds, pinvec \
                length = {}", $i, $s.len);
        }
        $s.calc_index($i)
    }};
}

impl<T> PinVec<T> {
    /// New, empty `PinVec`.
    pub fn new(buf_0_len: PowOf2<usize>) -> Self {
        PinVec {
            buffers: Vec::new(),
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
    
    /// Compute the correct capacity for a buffer, given 
    /// its outer index.
    fn correct_buffer_size(&self, outer_i: usize) -> usize {
        1 << self.buf_0_len.exp() << outer_i
    }
    
    /// Current length in elements.
    pub fn len(&self) -> usize { self.len }
    
    /// Push an element onto the top of the vector.
    ///
    /// Allocates more  memory if necessary, but never
    /// moves existing elements do, as that would violate
    /// `Pin` invariants.
    pub fn push(&mut self, elem: T) {
        // elem's index will be current length
        let (outer, inner) = self.calc_index(self.len());
        
        // potentially add a new buffer
        if outer >= self.buffers.len() {
            debug_assert_eq!(outer, self.buffers.len());
            self.buffers.push(PinBuffer::new(
                self.correct_buffer_size(outer)));
        }
        
        // push to buffer
        let buffer = &mut self.buffers[outer];
        debug_assert_eq!(buffer.len(), inner);
        buffer.push(elem);
        
        // maintain tracking data
        self.len += 1;
    }
    
    /// Pop and drop the top element. 
    ///
    /// Return false if already empty.
    /// 
    /// Returning element would violate `Pin` variants.
    pub fn remove_top(&mut self) -> bool {
        if self.len == 0 { return false; }
        
        let top_buffer_i = self.buffers.len() - 1;
        let top_buffer = &mut self.buffers[top_buffer_i];
        
        // remove top element
        debug_assert!(top_buffer.len() > 1);
        top_buffer.remove_top();
        
        // potentially remove top buffer
        // (in addition to reducing memory footprint, other
        //  assertions rely on the invariant that all 
        //  buffers will be nonempty of elements)
        if top_buffer.len() == 0 {
            self.buffers.pop();
        }
        
        // maintain tracking data
        self.len -= 1;
        
        true
    }
    
    /// Override an existing element.
    ///
    /// Quoting [std::pin::Pin](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.set):
    /// 
    /// > This overwrites pinned data, but that is okay:
    /// > its destructor gets run before being overwritten,
    /// > so no pinning guarantee is violated.
    pub fn set(&mut self, index: usize, elem: T) {
        let (outer, inner) = valid_index!(self, index);
        self.buffers[outer].set(inner, elem);
    }
    
    /// Get by index as pinned shared ref, or panic.
    pub fn idx_ref(&self, index: usize) -> Pin<&T> {
        let (outer, inner) = valid_index!(self, index);
        self.buffers[outer].idx_ref(inner)
    }
    
    /// Get by index as pinned mutable ref, or panic.
    pub fn idx_mut(&mut self, index: usize) -> Pin<&mut T> {
        let (outer, inner) = valid_index!(self, index);
        self.buffers[outer].idx_mut(inner)
    }
    
    /// Get by index as pinned shared ref.
    pub fn get_ref(&self, index: usize) -> Option<Pin<&T>> {
        let (outer, inner) = valid_index!(self, index);
        self.buffers[outer].get_ref(inner)
    }
    
    /// Get by index as pinned mutable ref.
    pub fn get_mut(&mut self, index: usize) -> Option<Pin<&mut T>> {
        let (outer, inner) = valid_index!(self, index);
        self.buffers[outer].get_mut(inner)
    }
}


impl<T> Default for PinVec<T> {
    fn default() -> Self {
        PinVec::new(PowOf2::<usize>::_64)
    }
}