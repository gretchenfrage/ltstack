//! # Pinvec
//! 
//! A growable vector-like structure which never moves its contents,
//! and guarantees this contract through the pin api.

extern crate pow_of_2;

/// Pinned non-growing buffer.
pub mod buf;

/// Pinned growable buffer.
pub mod vec;

pub use self::{
    buf::PinBuffer,
    vec::PinVec,
};