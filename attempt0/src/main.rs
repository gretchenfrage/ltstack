
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


#[derive(Debug)]
struct Foo<'a>(&'a mut u32);
impl<'a> LtDisable<Foo<'static>> for Foo<'a> {
    unsafe fn into_static(self) -> Foo<'static> {
        std::mem::transmute(self)
    }
}
impl<'o> LtEnable<'o> for Foo<'static> {
    type Output = Foo<'o>;
    
    unsafe fn give_lifetime(self) -> Foo<'o> { self }
    
    unsafe fn give_lifetime_ref<'s>(&'s self) -> &'s Foo<'o> {
        std::mem::transmute(self)
    }
    
    unsafe fn give_lifetime_mut<'s>(&'s mut self) -> &'s mut Foo<'o> { 
        std::mem::transmute(self)
    }
}

use std::cell::UnsafeCell;

struct LtStack<'base, S>
where
    S: 'static,
    S: for<'a> LtEnable<'a>
{
    vec: Vec<UnsafeCell<S>>,
    
    p: std::marker::PhantomData<&'base mut ()>,
}

trait Borrower<'l, S>: Sized
where
    S: 'static,
    S: for<'a> LtEnable<'a>
{
    type Borrowed: LtDisable<S> + 'l;
    type Iterator: IntoIterator<Item=Self::Borrowed>;
    
    fn apply<'v>(self, top: &'v mut <S as LtEnable<'l>>::Output) -> Self::Iterator;
}

impl<'l, S, F, I> Borrower<'l, S> for F
where
    S: 'static,
    S: for<'a> LtEnable<'a>,
    F: for<'v> FnOnce(&'v mut <S as LtEnable<'l>>::Output) -> I,
    I: IntoIterator,
    I::Item: LtDisable<S> + 'l,
{
    type Borrowed = I::Item;
    type Iterator = I;
    
    fn apply<'v>(self, top: &'v mut <S as LtEnable<'l>>::Output) -> Self::Iterator {
        self(top)
    }
}

impl<'base, S> LtStack<'base, S>
where
    S: 'static,
    S: for<'a> LtEnable<'a>
{
    // == constructors ==

    fn empty() -> Self {
        LtStack { vec: Vec::new(), p: std::marker::PhantomData }
    }
    
    // == mutators ==
    
    fn push<E>(&mut self, elem: E)
    where
        E: LtDisable<S> + 'base,
    {
        unsafe { self.vec.push(UnsafeCell::new(elem.into_static())) };
    }
    
    fn pop<'s>(&'s mut self) -> Option<<S as LtEnable<'s>>::Output> {
        self.vec.pop().map(|cell| unsafe {
            cell.into_inner().give_lifetime()
        })
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
            let top = &mut self.vec[len - 1];
            let top = (&mut *top.get()).give_lifetime_mut();
        
            let iter = f.apply(top);
            for elem in iter {
                self.vec.push(UnsafeCell::new(elem.into_static()));
            }
        }
        
        true
    }
    
    /*
    fn grow<F>(&mut self, f: F)
    where
        F: for<'s, 'a> FnOnce<(&'s mut <S as LtEnable<'a>>::Output,)>
        
        {}
        */
    /*
    fn grow<F, I, E>(&mut self, f: F) 
    where for<'s> for<'a> F: FnOnce(&'s mut <S as LtEnable<'a>>::Output) -> I, I: IntoIterator<Item=E>, E: LtDisable<S> + 'a
    {}
    */
    
    // == accessors ==
    
    fn len(&self) -> usize { self.vec.len() }
    
    fn top<'s>(&'s mut self) -> Option<&mut <S as LtEnable<'s>>::Output> {
        match self.vec.len() {
            0 => None,
            l => Some(unsafe {
                let elem = &mut self.vec[l - 1];
                (&mut *elem.get()).give_lifetime_mut()
            })
        }
    }
    
    
    
    // pop
    // grow
    // iter (immutable)
}

fn main() {
    let mut n: u32 = 7;
    let mut stack = LtStack::empty();
    
    stack.push(Foo(&mut n));
    stack.grow(|foo: &mut Foo| Some(Foo(foo.0)));
    
    /*
    stack.push(Foo(&mut n));
    *stack.top().unwrap().0 += 1;
    dbg!(stack.top());
    
    //stack.push(Foo(&mut n));
    *stack.top().unwrap().0 += 1;
    dbg!(stack.top());
    
    *stack.pop().unwrap().0 += 1;
    */
    
    dbg!(n);
}