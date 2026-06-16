use softcore_rv64::prelude::*;
use crate::minimal;

// Implementation ----------------------------------------------------------------------------------

fn memset(mut input: &mut [u8], value: u8) {
    let mut len = input.len();
    while 0 < len {
        let vl = minimal::vset(input.len());
        minimal::memmove(minimal::RegIdx::V1, bvd(minimal::ELEM_SIZE as i128, value as u64));
        minimal::memstore8(bvd(64, input.as_mut_ptr() as u64), minimal::RegIdx::V1);
        input = &mut input[vl..];
        len -= vl;
    }
}

// Verification ------------------------------------------------------------------------------------

#[cfg(kani)]
#[kani::proof]
fn check_memset() {
    const LEN: usize = 32;
    let FILL_VALUE: u8 = kani::any();
    let mut inp: [u8; LEN] = kani::any();

    let vl = memset(&mut inp, FILL_VALUE);

    for i in 0..inp.len() {
        assert_eq!(inp[i], FILL_VALUE);
    }
}
