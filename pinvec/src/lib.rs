
extern crate pow_of_2;

/// Pinned non-growing buffer.
pub mod buf;

/// Pinned growable buffer.
pub mod vec;

pub use self::{
    buf::PinBuffer,
    vec::PinVec,
};