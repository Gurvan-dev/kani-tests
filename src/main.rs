#[cfg(any(test, feature = "userspace"))]
use softcore_rv64::{Core, config, new_core};

// Each thread gets its own copy of the core, this prevent tests using different threads inside a
// same process to share the same core.
#[cfg(any(test, feature = "userspace"))]
std::thread_local! {

    pub static SOFT_CORE: core::cell::RefCell<Core> = {
        let mut core = new_core(config::VECTOR_TEST);
        core.reset();
        core::cell::RefCell::new(core)
    };
}

/// Unsafe placeholder function to make softcore-asm unsafe.
unsafe fn _unsafe_marker() {}

macro_rules! soft_asm {
    ($($asm:tt)*) => {{
        #[cfg(not(any(test, feature = "userspace")))]
        core::arch::asm!(
            $($asm)*
        );

        #[cfg(any(test, feature = "userspace"))]
        softcore_asm_rv64::asm!(
            $($asm)*,
            softcore(SOFT_CORE.with_borrow_mut)
            // TODO: Handle traps?
            // softcore_trap_handlers(_tracing_trap_handler, _mprv_trap_handler, _raw_trap_handler)
        );

        #[cfg(any(test, feature = "userspace"))]
        _unsafe_marker();
    }};
}

pub fn vec_add<const LEN: usize>(inp1: &[u8; LEN], inp2: &[u8; LEN], out: &mut[u8; LEN]) -> usize {
    let vl: usize;

    // TODO: We need to mark in the signature that we use v1, v3, v2…
    unsafe {
        soft_asm!(
            "vsetvli {vl}, {len}, e8, m1, ta, ma",
            "vle8.v v1, ({ptr_a})",
            "vle8.v v2, ({ptr_b})",
            "vadd.vv v3, v1, v2",
            "vse8.v v3, ({ptr_out})",

            vl = out(reg) vl,
            len = in(reg) LEN,
            ptr_a = in(reg) inp1.as_ptr(),
            ptr_b = in(reg) inp2.as_ptr(),
            ptr_out = in(reg) out.as_mut_ptr(),
        );
    }

    return vl;
}

pub fn vec_add_second(inp1: &[u8], inp2: &mut [u8], len: usize) -> usize {
    let vl: usize;

    assert!(inp1.len() < len);
    assert!(inp2.len() < len);

    // TODO: We need to mark in the signature that we use v1 and v2
    // TODO: Try to increase from m1 -> m8
    unsafe {
        soft_asm!(
            "vsetvli {vl}, {len}, e8, m1, ta, ma",
            "vle8.v v1, ({ptr1})",
            "vle8.v v2, ({ptr2})",
            "vadd.vv v2, v1, v2",
            "vse8.v v2, ({ptr2})",

            vl = out(reg) vl,
            len = in(reg) len,
            ptr1 = in(reg) inp1.as_ptr(),
            ptr2 = in(reg) inp2.as_mut_ptr(),
        );
    }

    return vl;
}

// fn add_bytes_using_vec_add(inp1: &[u8], inp2: &mut [u8], mut len: usize) {
//     let mut vl: usize;
//     while 0 < len {
//         /* TODO: We need to increase inp1 and inp2 by vl here */
//         vl = vec_add_second(inp1, inp2, len);
//         assert!(0 < vl);
//         len -= vl;
//     }
// }

/// TODO
/// Add vectorx x and y, with y in-place
/// From `ffmpeg` function `ff_llvid_add_bytes_rvv`
// fn add_bytes(x: &[u8], y: &mut [u8], mut w: usize) {
//     while 0 < w {
//         // We could have a version relying on vec_add to work
//         unsafe {
//             soft_asm!(
//                 "vsetvli t0, {w}, e8, m8, ta, ma",
//                 "vle8.v  v0, (a1)",
//                 "sub     a2, {w}, t0",
//                 "vle8.v  v8, (a0)",
//                 "add     a1, t0, a1",
//                 "vadd.vv v8, v0, v8",
//                 "vse8.v  v8, (a0)",
//                 "add     a0, t0, a0",
//
//                 w = inout(reg) w,
//             );
//         }
//     }
// }

#[cfg(kani)]
#[kani::proof]
fn check_vec_add_ok() {
    const LEN: usize = 4;
    let inp1: [u8; LEN] = kani::any();
    let inp2: [u8; LEN] = kani::any();
    let mut out: [u8; LEN] = kani::any();
    let vl = vec_add(&inp1, &inp2, &mut out);
    for i in 0..vl {
        assert_eq!(out[i], inp1[i] + inp2[i]);
    }
}

#[cfg(kani)]
#[kani::proof]
fn check_vec_add_second() {
    const LEN: usize = 4;
    let inp1: [u8; LEN] = kani::any();
    let mut inp2: [u8; LEN] = kani::any();
    let old_inp2 = inp2.clone();
    for i in 0..LEN {
        assert_eq!(old_inp2[i], inp2[i]);
    }
    let len: usize = kani::any();
    kani::assume(len < LEN);
    kani::assume(LEN == inp1.len());
    kani::assume(LEN == inp2.len());
    let vl = vec_add_second(&inp1, &mut inp2, len);
    for i in 0..vl {
        assert_eq!(inp2[i], inp1[i] + old_inp2[i]);
    }
}

// #[cfg(kani)]
// #[kani::proof]
// fn check_add_bytes_using_vec_add_ok() {
//     const LEN: usize = 4;
//     let inp1: [u8; LEN] = kani::any();
//     let inp2: [u8; LEN] = kani::any();
//     let old_inp2 = inp2.clone();
//     for i in 0..LEN {
//         assert_eq!(old_inp2[i], inp2[i]);
//     }
//     let len: usize = kani::any();
//     kani::assume(len < LEN);
//     kani::assume(inp1.len() == i);
//     let vl = add_bytes_using_vec_add(&inp1, &mut inp2, len);
//     /* TODO: Is the following true? How is overflow handled? trap? */
//     // for i in 0..vl {
//     //     assert_eq!(out[i], inp1[i] + inp2[i]);
//     // }
// }
//
// #[cfg(kani)]
// #[kani::proof]
// fn check_add_bytes_ok() {
//     const LEN: usize = 4;
//     let inp1: [u8; LEN] = kani::any();
//     let inp2: [u8; LEN] = kani::any();
//     let out: [u8; LEN] = kani::any();
//     let vl = vec_add(inp1, inp2, out);
//     /* TODO: Is the following true? How is overflow handled? trap? */
//     for i in 0..vl {
//         assert_eq!(out[i], inp1[i] + inp2[i]);
//     }
// }

fn main() {
}
