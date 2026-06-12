use crate::functions::*;

#[cfg(kani)]
#[kani::proof]
fn check_vsetvli_non_zero() {
    let len: usize = 4;
    let vl = vsetvli(len);
    assert!(len == 0 || 0 < vl);
}

#[cfg(kani)]
#[kani::proof]
fn check_vec_asm_add_second_partial() {
    const LEN: usize = 4;
    let len_copy: usize = kani::any();
    let inp1: [u8; LEN] = kani::any();
    let mut inp2: [u8; LEN] = kani::any();
    let old_inp2 = inp2.clone();
    for i in 0..LEN {
        assert_eq!(old_inp2[i], inp2[i]);
    }
    kani::assume(len_copy <= LEN);
    kani::assume(LEN == inp1.len());
    kani::assume(LEN == inp2.len());

    let vl = vec_add_second_partial(&inp1, &mut inp2, len_copy);

    for i in 0..vl {
        assert_eq!(inp2[i], inp1[i].wrapping_add(old_inp2[i]));
    }
}

#[cfg(kani)]
#[kani::proof]
fn check_vec_pure_add_second_partial() {
    const LEN: usize = 1024;
    let len_copy: usize = kani::any();
    let inp1: [u8; LEN] = kani::any();
    let mut inp2: [u8; LEN] = kani::any();
    let old_inp2 = inp2.clone();
    kani::assume(len_copy < LEN);
    for i in 0..LEN {
        assert_eq!(old_inp2[i], inp2[i]);
    }
    kani::assume(len_copy <= LEN);
    kani::assume(LEN == inp1.len());
    kani::assume(LEN == inp2.len());

    let vl = vec_add_second_partial_pure(&inp1, &mut inp2, len_copy);

    for i in 0..vl {
        assert_eq!(inp2[i], inp1[i].wrapping_add(old_inp2[i]));
    }
}

#[cfg(kani)]
#[kani::proof]
fn check_vec_pure_add_second() {
    const LEN: usize = 128;
    let len_copy: usize = kani::any();
    let inp1: [u8; LEN] = kani::any();
    let mut inp2: [u8; LEN] = kani::any();
    let old_inp2 = inp2.clone();
    kani::assume(len_copy < LEN);
    for i in 0..LEN {
        assert_eq!(old_inp2[i], inp2[i]);
    }
    kani::assume(len_copy <= LEN);
    kani::assume(LEN == inp1.len());
    kani::assume(LEN == inp2.len());

    let vl = vec_add_second_pure(&inp1, &mut inp2, len_copy);

    for i in 0..len_copy {
        assert_eq!(inp2[i], inp1[i].wrapping_add(old_inp2[i]));
    }
}

#[cfg(kani)]
#[kani::proof]
fn check_vec_asm_add_second() {
    const LEN: usize = 128;
    let len_copy: usize = kani::any();
    let inp1: [u8; LEN] = kani::any();
    let mut inp2: [u8; LEN] = kani::any();
    let old_inp2 = inp2.clone();
    kani::assume(len_copy < LEN);
    for i in 0..LEN {
        assert_eq!(old_inp2[i], inp2[i]);
    }
    kani::assume(len_copy <= LEN);
    kani::assume(LEN == inp1.len());
    kani::assume(LEN == inp2.len());

    let vl = vec_add_second(&inp1, &mut inp2, len_copy);

    for i in 0..len_copy {
        assert_eq!(inp2[i], inp1[i].wrapping_add(old_inp2[i]));
    }
}

#[cfg(kani)]
#[kani::proof]
fn check_vec_asm_set_partial() {
    const LEN: usize = 2;
    let len_fill: usize = 1;
    const FILL_VALUE: u8 = 0;
    let mut inp: [u8; LEN] = [1, 2];

    kani::assume(len_fill <= LEN);
    kani::assume(LEN == inp.len());

    let vl = vec_set_partial(&mut inp, len_fill, FILL_VALUE);

    for i in 0..vl {
        assert_eq!(inp[i], FILL_VALUE);
    }
}
