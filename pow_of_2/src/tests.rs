
use crate::{PowOf2, Two};
use core::{
    usize,
    mem::size_of,
};

const SBITS: usize = size_of::<usize>() * 8;

#[test]
fn create() {
    for e in 0..(SBITS - 4) {
        let _: PowOf2<usize> = PowOf2::from_exp(e as u8);
    }
}

#[test]
fn to_uint() {
    for e in 0..(SBITS - 4) {
        let n: PowOf2<usize> = PowOf2::from_exp(e as u8);
        assert_eq!(n.to_uint(), usize::pow(2, e as u32));
    }
}

#[test]
fn mul() {
    for e0 in 0..(SBITS / 4) {
        for e1 in 0..(SBITS / 4) {
            let n0: PowOf2<usize> = PowOf2::from_exp(e0 as u8);
            let n1: PowOf2<usize> = PowOf2::from_exp(e1 as u8);
            assert_eq!(
                (n0 * n1).to_uint(),
                usize::pow(2, e0 as u32) * usize::pow(2, e1 as u32)
            );
        }
    }
}

#[test]
fn div() {
    for e0 in 0..(SBITS / 4) {
        for e1 in 0..(SBITS / 2) {
            let n0: PowOf2<usize> = PowOf2::from_exp(e0 as u8);
            let n1: PowOf2<usize> = PowOf2::from_exp(e1 as u8);
            assert_eq!(
                (n0 / n1).to_uint(),
                usize::max(
                    usize::pow(2, e0 as u32) / usize::pow(2, e1 as u32),
                    1,
                )
            );
        }
    }
}

#[test]
fn mul_2() {
    for e in 0..(SBITS - 4) {
        let n: PowOf2<usize> = PowOf2::from_exp(e as u8);
        assert_eq!(
            (n * Two).to_uint(),
            n.to_uint() * 2,
        );
    }
}

#[test]
fn div_2() {
    for e in 0..(SBITS - 4) {
        let n: PowOf2<usize> = PowOf2::from_exp(e as u8);
        assert_eq!(
            (n / Two).to_uint(),
            usize::max(n.to_uint() / 2, 1),
        );
    }
}

#[test]
#[should_panic]
fn oob() {
    let _: PowOf2<usize> = PowOf2::from_exp(200);
}

#[test]
#[should_panic]
fn oob_mul_2() {
    let _: PowOf2<usize> = PowOf2::from_exp(SBITS as u8 - 1) * Two;
}

#[test]
#[should_panic]
fn oob_64() {
    let _: PowOf2<usize> = PowOf2::from_exp(SBITS as u8);
}

#[test]
fn not_oob_63() {
    let _: PowOf2<usize> = PowOf2::from_exp(SBITS as u8 - 1);
}

#[test]
#[should_panic]
fn mul_oob_63() {
    let _: PowOf2<usize> = PowOf2::from_exp(SBITS as u8 - 1) * PowOf2::from_exp(SBITS as u8 - 1);
}

#[test]
#[should_panic]
fn mul_oob_32() {
    let _: PowOf2<usize> = PowOf2::from_exp(SBITS as u8 / 2) * PowOf2::from_exp(SBITS as u8 / 2);
}
