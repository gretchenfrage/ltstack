#![no_std]

//! # pow_of_2
//! 
//! Integer-like types which can only represent powers of 2. Internally,
//! they are stored as a one-byte exponent. This allows them to implement
//! arithmetic operators with other, simpler artithmetic operators.

#[cfg(test)]
mod tests;

use core::{
    u8,
    ops::{Shl, Mul, Div, MulAssign, DivAssign},
    mem::size_of,
    fmt::{self, Formatter, Display, Debug},
    any::type_name,
    marker::PhantomData,
};


pub trait UInt: Copy + Shl<Output=Self> + Display {
    fn one() -> Self;
    fn from_u8(b: u8) -> Self;
}

macro_rules! impl_uint {
    ($($t:ty),*)=>{$(
        impl UInt for $t {
            #[inline(always)] fn one() -> $t { 1 } 
            #[inline(always)] fn from_u8(b: u8) -> $t { b as $t }
        }
    )*};
}
impl_uint!(usize, u8, u16, u32, u64, u128);


/// Unsigned integer powers of 2.
///
/// Internally just an exponent. Consequentially takes 
/// advantage of bit-manipulation. Runtime-checked to be 
/// valid values of `T`. 
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PowOf2<T: UInt> {
    exp: u8,
    p: PhantomData<T>,
}

impl<T: UInt> PowOf2<T> {
    /// Fails if above `T`s domains.
    #[inline(always)]
    pub fn try_from_exp(exp: u8) -> Option<Self> {
        if (exp as usize) < (size_of::<T>() * 8) {
            Some(PowOf2 { exp, p: PhantomData })
        } else { None }
    }
    
    /// Panics if above `T`s domains.
    #[inline(always)]
    pub fn from_exp(exp: u8) -> Self {
        PowOf2::try_from_exp(exp)
            .unwrap_or_else(|| panic!("exponent {} beyond domain of {}", 
                exp, type_name::<T>()))
    }
    
    /// Raise self to a power.
    /// 
    /// Fails if above `T`'s domain.
    #[inline(always)]
    pub fn try_pow(self, p: u8) -> Option<Self> {
        u8::checked_add(self.exp, p)
            .and_then(PowOf2::try_from_exp)
    }
    
    /// Raise self to a power.
    /// 
    /// Panics if above `T`'s domain.
    #[inline(always)]
    pub fn pow(self, p: u8) -> Self {
        let exp2 = u8::checked_add(self.exp, p)
            .unwrap_or_else(|| panic!("sum of exponents {} and {} cannot \
                be represented by u8", self.exp, p));
        PowOf2::from_exp(exp2)
    }
    
    /// Raise self to a negative power.
    ///
    /// Should not panic, simply bottoms out at zero.
    #[inline(always)]
    pub fn pow_neg(self, p: u8) -> Self {
        PowOf2 { 
            exp: u8::saturating_sub(self.exp, p),
            p: PhantomData,
        }
    }
    
    /// Represent as non-exponent.
    ///
    /// Should not fail at this point (would fail earlier).
    #[inline(always)]
    pub fn to_uint(self) -> T {
        T::one() << T::from_u8(self.exp)
    }
    
    /// Get exponent.
    #[inline(always)]
    pub fn exp(self) -> u8 {
        self.exp
    }
}


// ==== type-enhanced arithmetic ====


impl<T: UInt> Mul<PowOf2<T>> for PowOf2<T> {
    type Output = PowOf2<T>;
    
    #[inline(always)]
    fn mul(self, rhs: PowOf2<T>) -> PowOf2<T> {
        self.pow(rhs.exp)
    }
}

impl<T: UInt> MulAssign<PowOf2<T>> for PowOf2<T> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: PowOf2<T>) {
        *self = *self * rhs;
    }
}

impl<T: UInt> Div<PowOf2<T>> for PowOf2<T> {
    type Output = PowOf2<T>;
    
    #[inline(always)]
    fn div(self, rhs: PowOf2<T>) -> PowOf2<T> {
        self.pow_neg(rhs.exp)
    }
}

impl<T: UInt> DivAssign<PowOf2<T>> for PowOf2<T> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: PowOf2<T>) {
        *self = *self / rhs;
    }
}


/// Unitary representation of 2, for `PowOf2` arithmetic.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Two;

impl<T: UInt> Mul<Two> for PowOf2<T> {
    type Output = PowOf2<T>;
    
    #[inline(always)]
    fn mul(self, Two: Two) -> PowOf2<T> {
        self.pow(1)
    }
}

impl<T: UInt> MulAssign<Two> for PowOf2<T> {
    #[inline(always)]
    fn mul_assign(&mut self, Two: Two) {
        *self = *self * Two;
    }
}

impl<T: UInt> Div<Two> for PowOf2<T> {
    type Output = PowOf2<T>;
    
    #[inline(always)]
    fn div(self, Two: Two) -> PowOf2<T> {
        self.pow_neg(1)
    }
}

impl<T: UInt> DivAssign<Two> for PowOf2<T> {
    #[inline(always)]
    fn div_assign(&mut self, Two: Two) {
        *self = *self / Two;
    }
}


// ==== power of 2 constants ====

macro_rules! pow_of_2_consts {
    (#[$attr:meta] for $ty:ty {$( ($($t:tt)*) )*})=>{
        $( pow_of_2_consts!(@try_recurse ($($t)*) ); )*
        pow_of_2_consts!(@items #[$attr] $ty {} {$( ($($t)*) )*});
    };
    
    (@items #[$attr:meta] $ty:ty {$($accum:tt)*} 
            {})=>{ 
        impl PowOf2<$ty> {$($accum)*}
        
        #[$attr] impl PowOf2<usize> {$($accum)*}
        
        //#[cfg(target_pointer_width = stringify!($ty))]
        //impl PowOf2<usize> {$($accum)*}
    };
    (@items #[$attr:meta] $ty:ty {$($accum:tt)*} 
            {($name:ident : $exp:expr) $($t:tt)*})=>{
        pow_of_2_consts!(@items #[$attr] $ty { 
            $($accum)* 
            pub const $name: Self = PowOf2 {
                exp: $exp,
                p: PhantomData,
            };
        } { $($t)* });
    };
    (@items #[$attr:meta] $ty:ty {$($accum:tt)*} 
            {(#[$attr2:meta] for $ty2:ty {$( ($($t2:tt)*) )*}) $($t:tt)*})=>{
        pow_of_2_consts!(@items #[$attr] $ty
            {$($accum)*}
            {$( ( $($t2)* ) )* $($t)*});
    };
    
    (@try_recurse ( #[$attr:meta] for $ty:ty {$( ($($t:tt)*) )*} ))=>{
        pow_of_2_consts!( #[$attr] for $ty {$( ($( $t )*) )*} );
    };
    (@try_recurse ( $name:ident : $exp:expr ))=>{};
}

pow_of_2_consts! {
    #[cfg(target_pointer_width = "128")] for u128 {
        (#[cfg(target_pointer_width = "64")] for u64 {
            (#[cfg(target_pointer_width = "32")] for u32 {
                (#[cfg(target_pointer_width = "16")] for u16 {
                    (#[cfg(target_pointer_width = "8")] for u8 {
                        (_1: 0)
                        (_2: 1)
                        (_4: 2)
                        (_8: 3)
                        (_16: 4)
                        (_32: 5)
                        (_64: 6)
                        (_128: 7)
                    })
                    (_256: 8)
                    (_512: 9)
                    (KIBI: 10)
                })
                (MEBI: 20)
                (GIBI: 30)
            })
            (TEBI: 40)
            (PEBI: 50)
            (EXBI: 60)
            
        })
        (ZEBI: 70)
        (YOBI: 80)
        (_2E90: 90)
        (_2E100: 100)
        (_2E110: 110)
        (_2E120: 120)
    }
}

// ==== formatters ====

impl<T: UInt> Debug for PowOf2<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("PowOf2")
            .field("exp", &self.exp)
            .finish()
    }
}

impl<T: UInt> Display for PowOf2<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&format_args!("2^{}", self.exp), f)
    }
}

impl Debug for Two {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { 
        Debug::fmt(&2, f)
    }
}

impl Display for Two {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { 
        Display::fmt(&2, f)
    }
}