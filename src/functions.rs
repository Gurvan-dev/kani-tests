use softcore_asm_rv64::softcore_init;
use softcore_rv64::prelude::{bvd, BoundedVec};
use softcore_rv64::{Core, config, new_core};

const CONFIG: softcore_rv64::raw::Config = config::VECTOR_TEST;
softcore_init!(CONFIG);

macro_rules! soft_asm {
    ($($asm:tt)*) => {
        softcore_asm_rv64::asm!(
            $($asm)*,
            softcore(self)
        )
    };
}

pub fn vsetvli(len: usize) -> usize {
    let vl: usize;
    unsafe {
        soft_asm!(
            "vsetvli {vl}, {len}, e8, m1, ta, ma",
            vl = out(reg) vl,
            len = in(reg) len,
        )
    }
    return vl;
}

/// add `inp1` to `inp2` in place
/// Inspired by `ffmpeg` function `ff_llvid_add_bytes_rvv`
pub fn vec_add_second_partial(inp1: &[u8], inp2: &mut [u8], len: usize) -> usize {
    let vl: usize;

    assert!(len <= inp1.len());
    assert!(len <= inp2.len());

    // TODO: Try to increase from m1 -> m8
    unsafe {
        soft_asm!(
            "vsetvli {vl}, {len}, e8, m1, ta, ma",
            "vle8.v v0, ({ptr1})",
            "vle8.v v8, ({ptr2})",
            "vadd.vv v8, v0, v8",
            "vse8.v v8, ({ptr2})",

            vl = out(reg) vl,
            len = in(reg) len,
            ptr1 = in(reg) inp1.as_ptr(),
            ptr2 = in(reg) inp2.as_mut_ptr(),
            out("v0") _,
            out("v8") _
        );
    }

    assert!(len == 0 || 0 < vl);

    return vl;
}

/// add `inp1` to `inp2` in place
/// Inspired by `ffmpeg` function `ff_llvid_add_bytes_rvv`
pub fn vec_add_second_partial_pure(inp1: &[u8], inp2: &mut [u8], len: usize) -> usize {
    assert!(len <= inp1.len());
    assert!(len <= inp2.len());

    const AMOUNT: usize = 128;
    let vl = std::cmp::min(len, AMOUNT);
    for i in 0..len {
        inp2[i] = inp1[i].wrapping_add(inp2[i]);
    }

    assert!(len == 0 || 0 < vl);

    return vl;
}

// TODO: Might be easier to do this not in place
pub fn vec_add_second(mut inp1: &[u8], mut inp2: &mut [u8], mut len: usize) {
    let mut vl: usize;
    while 0 < len {
        vl = vec_add_second_partial(inp1, inp2, len);
        // TODO Loop invariant?
        assert!(0 < vl);
        inp1 = &inp1[vl..];
        inp2 = &mut inp2[vl..];
        len -= vl;
    }

    assert!(len == 0);
}

pub fn vec_add_second_pure(mut inp1: &[u8], mut inp2: &mut [u8], mut len: usize) {
    let mut vl: usize;
    while 0 < len {
        // TODO: We need to increase inp1 and inp2 by vl here
        vl = vec_add_second_partial_pure(inp1, inp2, len);
        // TODO (Loop invariant): Say that we have the addition we want
        assert!(0 < vl);
        inp1 = &inp1[vl..];
        inp2 = &mut inp2[vl..];
        len -= vl;
    }

    assert!(len == 0);
}

pub fn vec_set_partial<const FILL_VALUE: u8>(inp: &mut [u8], len: usize) -> usize {
    let vl: usize;

    assert!(len <= inp.len());

    unsafe {
        soft_asm!(
            "vsetvli {vl}, {len}, e8, m1, ta, ma",
            "vmv.v.x v0, {fill}",
            "vse8.v v0, ({ptr})",

            vl = out(reg) vl,
            fill = in(reg) FILL_VALUE,
            len = in(reg) len,
            ptr = in(reg) inp.as_ptr(),
            out("v0") _,
            out("v8") _
        );
    }

    assert!(len == 0 || 0 < vl);

    return vl;
}

pub fn vec_set<const LEN: usize, const FILL_VALUE: u8>(inp: &mut [u8; LEN]) {
    let mut vl: usize;
    let mut len = LEN;
    let mut inp: &mut [u8] = inp;
    while 0 < len {
        vl = vec_set_partial::<FILL_VALUE>(inp, len);
        assert!(0 < vl);
        inp = &mut inp[vl..];
        len -= vl;
    }

    assert!(len == 0);
}

// Tests -------------------------------------------------------------------------------------------

#[test]
fn check_vec_asm_set_partial() {
    const LEN: usize = 350;
    const FILL_VALUE: u8 = 0;
    const INITIAL_VALUE: u8 = 5;

    for len_fill in 0..LEN {
        let mut inp: [u8; LEN] = [INITIAL_VALUE; LEN];

        let vl = vec_set_partial::<FILL_VALUE>(&mut inp, len_fill);

        for i in 0..LEN {
            assert_eq!(inp[i], if i < vl { FILL_VALUE } else { INITIAL_VALUE });
        }
    }
}

#[test]
fn check_vec_asm_set() {
    const LEN: usize = 350;
    const FILL_VALUE: u8 = 0;
    const INITIAL_VALUE: u8 = 5;

    let mut inp: [u8; LEN] = [INITIAL_VALUE; LEN];

    vec_set::<_, FILL_VALUE>(&mut inp);

    for i in 0..LEN {
        assert_eq!(inp[i], FILL_VALUE);
    }
}
