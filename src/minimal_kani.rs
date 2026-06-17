use softcore_rv64::prelude::*;
use crate::minimal;

// Implementation ----------------------------------------------------------------------------------

pub fn memset(mut input: &mut [u8], value: u8) {
    let mut len = input.len();
    while 0 < len {
        let vl = minimal::vset(input.len(), minimal::VecSize::S8);
        minimal::xmove(minimal::RegIdx::X1, bvd(64, input.as_mut_ptr() as u64));
        minimal::vxmove(minimal::RegIdxVec::V1, bvd(8, value as u64));
        minimal::vstore8(minimal::RegIdx::X1, minimal::RegIdxVec::V1);
        input = &mut input[vl..];
        len -= vl;
    }
}

// Verification ------------------------------------------------------------------------------------

#[test]
fn check_memset() {
    const LEN: usize = 32;
    let fill_value: u8 = 1;
    let mut inp: [u8; LEN] = [0; LEN];

    memset(&mut inp, fill_value);

    for i in 0..inp.len() {
        assert_eq!(inp[i], fill_value, "{} != {} at indice {}", inp[i], fill_value, i);
    }
}

#[cfg(kani)]
#[kani::proof]
fn check_memset() {
    const LEN: usize = 32;
    let fill_value: u8 = kani::any();
    let mut inp: [u8; LEN] = kani::any();

    memset(&mut inp, fill_value);

    for i in 0..inp.len() {
        assert_eq!(inp[i], fill_value);
    }
}
