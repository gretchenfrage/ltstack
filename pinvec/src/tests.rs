
use std::usize;
use crate::PinVec;
use pow_of_2::PowOf2;

#[test]
fn calc_index_precomp() {
    let num_to_check: usize = 10000;
    for buf_zero_len in (0..10).map(PowOf2::<usize>::from_exp) {
        let mut buffer_size = buf_zero_len.to_uint();
        let mut outer = 0;
        let mut correct: Vec<(usize, usize)> = Vec::new();
        while correct.len() < num_to_check {
            for inner in 0..buffer_size {
                correct.push((outer, inner));
            }
            outer += 1;
            buffer_size *= 2;
        }
    
        let pin_vec: PinVec<()> = PinVec::new(buf_zero_len);
        for (elem_i, &(outer_i, inner_i)) in correct.iter().enumerate() {
            let (outer_i2, inner_i2) = pin_vec.calc_index(elem_i);
            assert_eq!(outer_i, outer_i2);
            assert_eq!(inner_i, inner_i2);
        }
    }
}

#[test]
fn calc_index_high_magnitude() {
    if std::env::var("HUGE_TEST").is_ok() {
        let num_to_check = usize::MAX / 2;
        for buf_zero_len in (0..(std::mem::size_of::<usize>() * 6))
            .map(|p| PowOf2::<usize>::from_exp(p as u8))     
        {
            let pin_vec: PinVec<()> = PinVec::new(buf_zero_len);
        
            let mut buffer_size = buf_zero_len.to_uint();
            let mut outer = 0;
            let mut elem_i = 0;
            while elem_i < num_to_check {
                for inner in 0..buffer_size {
                    let (outer2, inner2) = pin_vec.calc_index(elem_i);
                    assert_eq!(outer, outer2);
                    assert_eq!(inner, inner2);
                    elem_i += 1;
                
                    if elem_i % 1000000 == 0 { println!("{}", elem_i); }
                }
                outer += 1;
                buffer_size *= 2;
            }
        }
    }
}
