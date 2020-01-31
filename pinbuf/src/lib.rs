
use std::{
    pin::Pin,
    marker::Unpin,
    ptr::drop_in_place,
    mem::replace,
};

/// Wraps `Vec<T>` and disallows re-allocation.
pub struct PinBuffer<T> {
    vec: Vec<T>
}

impl<T> PinBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        PinBuffer { vec: Vec::with_capacity(capacity) }
    }
    
    /// Current element length.
    pub fn len(&self) -> usize { self.vec.len() }
    
    /// Current element capacity.
    pub fn capacity(&self) -> usize { self.vec.capacity() }
    
    /// Whether capacity allows to push another element.
    pub fn can_push(&self) -> bool {
        self.len() < self.capacity()
    }
    
    /// Push an element. Panics if at capacity.
    pub fn push(&mut self, elem: T) {
        assert!(self.can_push(), "push to full PinBuffer");
        self.vec.push(elem);
    }
    
    /// Pop and drop the top element. 
    ///
    /// Return false if already empty.
    /// 
    /// Returning element would violate `Pin` variants.
    pub fn remove_top(&mut self) -> bool {
        let len = match self.vec.len() {
            0 => return false,
            l => l,
        };
        unsafe {
            // take special care to drop element without moving it
            let top: *mut T = &mut self.vec[len - 1];
            self.vec.set_len(len - 1);
            drop_in_place(top);
        }
        true
    }
    
    /// Pop and return the top element.
    ///
    /// Only possible if the element type is `Unpin`.
    pub fn pop(&mut self) -> Option<T> 
    where T: Unpin {
        self.vec.pop() 
    }
    
    /// Pop and return the top element, ignoring `Pin` 
    /// invariants.
    pub unsafe fn pop_unchecked(&mut self) -> Option<T> {
        self.vec.pop()
    }
    
    /// Override an existing element.
    ///
    /// Quoting [std::pin::Pin](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.set):
    /// 
    /// > This overwrites pinned data, but that is okay:
    /// > its destructor gets run before being overwritten,
    /// > so no pinning guarantee is violated.
    pub fn set(&mut self, index: usize, elem: T) {
        self.vec[index] = elem;
    }
    
    /// Take and replace an existing element.
    ///
    /// Only possible if the element type is `Unpin`.
    /// 
    /// Panics on failure.
    pub fn replace(&mut self, index: usize, repl: T) -> T 
    where T: Unpin {
        replace(&mut self.vec[index], repl)
    }
    
    /// Get by index as pinned shared ref, or panic.
    pub fn idx_ref(&self, index: usize) -> Pin<&T> {
        unsafe { Pin::new_unchecked(&self.vec[index]) }
    }
    
    /// Get by index as pinned mutable ref, or panic.
    pub fn idx_mut(&mut self, index: usize) -> Pin<&mut T> {
        unsafe { Pin::new_unchecked(&mut self.vec[index]) } 
    }
    
    /// Get by index as pinned shared ref.
    pub fn get_ref(&self, index: usize) -> Option<Pin<&T>> {
        self.vec.get(index)
            .map(|r| unsafe { Pin::new_unchecked(r) })
    }
    
    /// Get by index as pinned mutable ref.
    pub fn get_mut(&mut self, index: usize) -> Option<Pin<&mut T>> {
        self.vec.get_mut(index)
            .map(|r| unsafe { Pin::new_unchecked(r) })
    }
}