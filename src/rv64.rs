use softcore_asm_rv64::softcore_init;
use softcore_rv64::prelude::{bvd, BitDynamic, BoundedVec};
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

pub fn memset1(inp: &mut [u8], fill_value: u8) {
    let mut len = inp.len();
    let mut inp: &mut [u8] = inp;

    while 0 < len {
        let vl: usize;
        unsafe {
            soft_asm!(
                "vsetvli {vl}, {len}, e8, m1, ta, ma",
                "vmv.v.x v0, {fill}",
                "vse8.v v0, ({ptr})",

                vl = out(reg) vl,
                fill = in(reg) fill_value,
                len = in(reg) len,
                ptr = in(reg) inp.as_ptr(),
                out("v0") _,
                out("v8") _
            );
            inp = &mut inp[vl..];
            len -= vl;
        }
        assert!(0 < vl);
    }
}

pub fn memset2(inp: &mut [u8], fill_value: u8) {
    let mut len = inp.len();
    let mut inp: &mut [u8] = inp;

    while 0 < len {
        unsafe {
            soft_asm!(
                "sb {fill_value} ({ptr})",
                fill_value = in(reg) fill_value,
                ptr = in(reg) inp.as_ptr(),
            );
            inp = &mut inp[1..];
            len -= 1;
        }
    }
}

pub fn memset3(inp: &mut [u8], fill_value: u8) {
    let mut len = inp.len();
    let mut inp: &mut [u8] = inp;

    while 0 < len {
        inp[0] = fill_value;
        inp = &mut inp[1..];
        len -= 1;
    }
}

pub fn memset4(inp: &mut [u8], fill_value: u8) {
    for i in 0..inp.len() {
        inp[i] = fill_value;
    }
}

pub fn memset5(inp: &mut [u8], fill_value: u8) {
    let mut i = 0usize;

    while i < inp.len() {
        unsafe {
            soft_asm!(
                "sb {fill_value} ({ptr})",
                fill_value = in(reg) fill_value,
                ptr = in(reg) inp[i..].as_ptr(),
            );
            i += 1;
        }
    }
}

pub fn memset6(inp: &mut [u8], fill_value: u8) {
    for i in 0..inp.len() {
        unsafe {
            soft_asm!(
                "sb {fill_value} ({ptr})",
                fill_value = in(reg) fill_value,
                ptr = in(reg) inp[i..].as_ptr(),
            );
        }
    }
}

pub fn memset7(inp: &mut [u8], fill_value: u8) {
    let mut i = 0usize;

    while i < inp.len() {
        let vl: usize;
        let inp2 = &mut inp[i..];
        unsafe {
            soft_asm!(
                "vsetvli {vl}, {len}, e8, m1, ta, ma",
                "vmv.v.x v0, {fill}",
                "vse8.v v0, ({ptr})",

                vl = out(reg) vl,
                fill = in(reg) fill_value,
                len = in(reg) inp2.len(),
                ptr = in(reg) inp2.as_ptr(),
                out("v0") _,
                out("v8") _
            );
        }
        i += vl;
        assert!(0 < vl);
    }
}

pub fn memset_broken(inp: &mut [u8], fill_value: u8) {
    let mut len = inp.len();
    let inp: &mut [u8] = inp;

    while 0 < len {
        let vl: usize;
        unsafe {
            soft_asm!(
                "vsetvli {vl}, {len}, e8, m1, ta, ma",
                "vmv.v.x v0, {fill}",
                "vse8.v v0, ({ptr})",

                vl = out(reg) vl,
                fill = in(reg) fill_value,
                len = in(reg) len,
                ptr = in(reg) inp.as_ptr(),
                out("v0") _,
                out("v8") _
            );
            len -= vl;
        }
        assert!(0 < vl);
    }
}

pub fn add1(inp1: &mut [u8], x: u8) -> usize {
    let len = inp1.len();
    if len == 0 {
        return 0;
    }

    let ptr = inp1.as_mut_ptr();
    let vlen: usize;

    unsafe {
        soft_asm!(
            "vsetvli {vlen}, {len}, e8, m1, ta, ma",
            "vle8.v v1, ({ptr})",
            "vmv.v.x v2, {x}",
            "vadd.vv v1, v1, v2",
            "vse8.v v1, ({ptr})",
            len = in(reg) len,
            ptr = in(reg) ptr,
            x = in(reg) x,
            vlen = out(reg) vlen,

            out("v1") _,
            out("v2") _,
        );
    }

    vlen
}

pub fn add_full(inp: &mut [u8], x: u8) {
    let mut inp_len = inp.len();
    let mut inp_ptr = inp.as_mut_ptr();

    while 0 < inp_len {
        let vlen: usize;
        unsafe {
            soft_asm!(
                "vsetvli {vlen}, {inp_len}, e8, m1, ta, ma",
                "vle8.v v1, ({inp_ptr})",
                "vadd.vx v1, v1, {x}",
                "vse8.v v1, ({inp_ptr})",
                inp_len = in(reg) inp_len,
                inp_ptr = in(reg) inp_ptr,
                x = in(reg) x,
                vlen = out(reg) vlen,

                out("v1") _,
            );
        }
        inp_ptr = unsafe { inp_ptr.add(vlen) };
        inp_len -= vlen;
    }
}

// Verification ------------------------------------------------------------------------------------

#[test]
fn test_memset1() {
    const LEN: usize = 64;

    for fill_value in 0..=2 {
        for seed in 0..=255 {
            let mut inp: [u8; LEN] = std::array::from_fn(|i| {
                (i.wrapping_add(seed as usize)) as u8
            });

            memset1(&mut inp, fill_value);

            for i in 0..inp.len() {
                assert_eq!(
                    inp[i],
                    fill_value,
                    "Failed at seed: {}, index: {}",
                    seed, i
                );
            }
        }
    }
}

#[cfg(kani)]
#[kani::proof]
fn kani_memset1() {
    const LEN: usize = 64;
    let fill_value: u8 = kani::any();
    let mut inp: [u8; LEN] = kani::any();

    memset1(&mut inp, fill_value);

    for i in 0..inp.len() {
        assert_eq!(inp[i], fill_value);
    }
}

#[cfg(kani)]
#[kani::proof]
fn kani_memset2() {
    const LEN: usize = 64;
    let fill_value: u8 = kani::any();
    let mut inp: [u8; LEN] = kani::any();

    memset2(&mut inp, fill_value);

    for i in 0..inp.len() {
        assert_eq!(inp[i], fill_value);
    }
}

#[cfg(kani)]
#[kani::proof]
fn kani_memset3() {
    const LEN: usize = 64;
    let fill_value: u8 = kani::any();
    let mut inp: [u8; LEN] = kani::any();

    memset3(&mut inp, fill_value);

    for i in 0..inp.len() {
        assert_eq!(inp[i], fill_value);
    }
}

#[cfg(kani)]
#[kani::proof]
fn kani_memset4() {
    const LEN: usize = 64;
    let fill_value: u8 = kani::any();
    let mut inp: [u8; LEN] = kani::any();

    memset4(&mut inp, fill_value);

    for i in 0..inp.len() {
        assert_eq!(inp[i], fill_value);
    }
}

#[cfg(kani)]
#[kani::proof]
fn kani_memset5() {
    const LEN: usize = 64;
    let fill_value: u8 = kani::any();
    let mut inp: [u8; LEN] = kani::any();

    memset5(&mut inp, fill_value);

    for i in 0..inp.len() {
        assert_eq!(inp[i], fill_value);
    }
}

#[cfg(kani)]
#[kani::proof]
fn kani_memset6() {
    const LEN: usize = 64;
    let fill_value: u8 = kani::any();
    let mut inp: [u8; LEN] = kani::any();

    memset6(&mut inp, fill_value);

    for i in 0..inp.len() {
        assert_eq!(inp[i], fill_value);
    }
}

#[cfg(kani)]
#[kani::proof]
fn kani_memset7() {
    const LEN: usize = 64;
    let fill_value: u8 = kani::any();
    let mut inp: [u8; LEN] = kani::any();

    memset7(&mut inp, fill_value);

    for i in 0..inp.len() {
        assert_eq!(inp[i], fill_value);
    }
}

#[cfg(kani)]
#[kani::proof]
fn kani_memset_broken() {
    const LEN: usize = 8;
    let fill_value: u8 = kani::any();
    let mut inp: [u8; LEN] = kani::any();

    memset_broken(&mut inp, fill_value);

    for i in 0..inp.len() {
        assert_eq!(inp[i], fill_value);
    }
}

#[test]
fn test_add1() {
    const LEN: usize = 16;

    for add in 0..=255 {
        for seed in 0..=255 {
            let mut inp: [u8; LEN] = std::array::from_fn(|i| {
                (i.wrapping_add(seed as usize)) as u8
            });
            let inp_copy = inp;

            let vl = add1(&mut inp, add);

            for i in 0..vl {
                assert_eq!(
                    inp[i],
                    inp_copy[i].wrapping_add(add),
                    "Failed at seed: {}, index: {}",
                    seed, i
                );
            }

            for i in vl..inp.len() {
                assert_eq!(
                    inp[i],
                    inp_copy[i],
                    "Failed at seed: {}, index: {}",
                    seed, i
                );
            }
        }
    }
}

#[cfg(kani)]
#[kani::proof]
fn kani_add1() {
    const LEN: usize = 4;
    let mut inp: [u8; LEN] = kani::any();
    let inp_copy: [u8; LEN] = inp;
    let x: u8 = kani::any();

    let vl: usize = add1(&mut inp, x);

    for i in 0..vl {
        assert_eq!(inp_copy[i].wrapping_add(x), inp[i]);
    }

    for i in vl..inp.len() {
        assert_eq!(inp_copy[i], inp[i]);
    }
}

#[cfg(kani)]
#[kani::proof]
fn kani_add_full() {
    const LEN: usize = 4;
    let mut inp: [u8; LEN] = kani::any();
    let x: u8 = kani::any();
    let inp_old = inp;
    add_full(&mut inp, x);

    for i in 0..LEN {
        assert_eq!(inp_old[i].wrapping_add(x), inp[i]);
    }
}

pub fn xor_cipher(inp: &mut [u8], key: &[u8]) {
    let mut inp_ptr = inp.as_mut_ptr();
    let mut inp_len = inp.len();
    let mut key_idx = 0usize;

    while inp_len > 0 {
        let vl: usize;

        unsafe {
            soft_asm!(
                "vsetvli {vl}, {avl}, e8, m1, ta, ma",
                "vle8.v v1, ({inp_ptr})",
                "vxor.vx v1, v1, {key_byte}",
                "vse8.v v1, ({inp_ptr})",

                vl = out(reg) vl,
                avl = in(reg) inp_len,
                inp_ptr = in(reg) inp_ptr,
                key_byte = in(reg) key[key_idx] as usize,
                out("v1") _,
            );
        }

        inp_ptr = unsafe { inp_ptr.add(vl) };
        inp_len -= vl;
        key_idx = (key_idx + vl) % key.len();
    }
}

#[cfg(kani)]
#[kani::proof]
fn kani_xor_cipher() {
    const INP_LEN: usize = 64;
    const KEY_LEN: usize = 64;
    let mut inp: [u8; INP_LEN] = kani::any();
    let key: [u8; KEY_LEN] = kani::any();
    let inp_old = inp;
    xor_cipher(&mut inp, &key);

    for i in 0..INP_LEN {
        let key_idx = i % KEY_LEN;
        assert_eq!(inp[i], inp_old[i] ^ key[key_idx]);
    }
}
