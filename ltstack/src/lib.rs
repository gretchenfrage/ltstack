
extern crate pinvec;

#[cfg(test)]
mod tests;

use std::{
    cell::UnsafeCell,
    pin::Pin,
};
use pinvec::PinVec;

trait LtDisable<S>: Sized
where
    S: 'static,
    S: for<'a> LtEnable<'a>,
{
    unsafe fn into_static(self) -> S;
}

trait LtEnable<'o>: Sized {
    type Output: 'o;
    
    unsafe fn give_lifetime(self) -> Self::Output;
    
    unsafe fn give_lifetime_ref<'s>(&'s self) -> &'s Self::Output;
    
    unsafe fn give_lifetime_mut<'s>(&'s mut self) -> &'s mut Self::Output;
}

struct LtStack<'base, S>
where
    S: 'static,
    S: for<'a> LtEnable<'a>
{
    vec: PinVec<UnsafeCell<S>>,
    
    p: std::marker::PhantomData<&'base ()>,
}

trait Borrower<'l, S>: Sized
where
    S: 'static,
    S: for<'a> LtEnable<'a>
{
    type Borrowed: LtDisable<S> + 'l;
    type Iterator: IntoIterator<Item=Self::Borrowed>;
    
    fn apply(self, top: &'l mut <S as LtEnable<'l>>::Output) -> Self::Iterator;
}

impl<'base, S> LtStack<'base, S>
where
    S: 'static,
    S: for<'a> LtEnable<'a>
{
    // == constructors ==

    fn empty() -> Self {
        LtStack { vec: PinVec::default(), p: std::marker::PhantomData }
    }
    
    // == mutators ==
    
    fn push<E>(&mut self, elem: E)
    where
        E: LtDisable<S> + 'base,
    {
        unsafe { self.vec.push(UnsafeCell::new(elem.into_static())) };
    }
    
    fn pop<'s>(&'s mut self) -> Option<<S as LtEnable<'s>>::Output> {
        unsafe { 
            self.vec.pop_unchecked().map(|cell| 
                cell.into_inner().give_lifetime())
        }
    }
    
    fn grow<F>(&mut self, f: F) -> bool
    where
        F: for<'l> Borrower<'l, S>
    {
        let len = match self.vec.len() {
            0 => return false,
            l => l,
        };
        
        unsafe {
            //let top = &mut self.vec[len - 1]; TODO
            let top = Pin::into_inner_unchecked(
                self.vec.idx_mut(len - 1));
            let top = (&mut *top.get()).give_lifetime_mut();
        
            let iter = f.apply(top);
            for elem in iter {
                self.vec.push(UnsafeCell::new(elem.into_static()));
            }
        }
        
        true
    }

    // == accessors ==
    
    fn len(&self) -> usize { self.vec.len() }
    
    fn top<'s>(&'s mut self) -> Option<&mut <S as LtEnable<'s>>::Output> {
        match self.vec.len() {
            0 => None,
            l => Some(unsafe {
                let elem = Pin::into_inner_unchecked(
                    self.vec.idx_mut(l - 1));
                //let elem = &mut self.vec[l - 1]; TODO
                (&mut *elem.get()).give_lifetime_mut()
            })
        }
    }
    
    
    
    // pop
    // grow
    // iter (immutable)
}

