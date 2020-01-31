
use crate::{LtDisable, LtEnable, LtStack, Borrower};

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

#[test]
fn basic() {
    let mut n: u32 = 0;
    let mut stack = LtStack::empty();
    
    struct B;
    impl<'l> Borrower<'l, Foo<'static>> for B {
        type Borrowed = Foo<'l>;
        type Iterator = Option<Foo<'l>>;
        
        fn apply(self, top: &'l mut Foo<'l>) -> Option<Foo<'l>> {
            Some(Foo(top.0))
        }
    }
    
    stack.push(Foo(&mut n));
    
    for _ in 0..99 {
        stack.grow(B);
    }
    while let Some(Foo(r)) = stack.pop() {
        *r += 1;
    }
    assert_eq!(n, 100)
}