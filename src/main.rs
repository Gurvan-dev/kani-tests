#[cfg(any(test, feature = "userspace", kani))]
use softcore_rv64::{Core, config, new_core};

// Macros and utils---------------------------------------------------------------------------------

#[cfg(any(test, feature = "userspace", kani))]
std::thread_local! {

    pub static SOFT_CORE: core::cell::RefCell<Core> = {
        let mut core = new_core(config::VECTOR_TEST);
        core.reset();
        core::cell::RefCell::new(core)
    };
}

unsafe fn _unsafe_marker() {}

macro_rules! soft_asm {
    ($($asm:tt)*) => {{
        #[cfg(not(any(test, feature = "userspace", kani)))]
        core::arch::asm!(
            $($asm)*
        );

        #[cfg(any(test, feature = "userspace", kani))]
        softcore_asm_rv64::asm!(
            $($asm)*,
            softcore(SOFT_CORE.with_borrow_mut)
            // TODO: Handle traps?
            // softcore_trap_handlers(_tracing_trap_handler, _mprv_trap_handler, _raw_trap_handler)
        );

        #[cfg(any(test, feature = "userspace", kani))]
        _unsafe_marker();
    }};
}

// Functions to verify -----------------------------------------------------------------------------

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
/// TODO: We could try to do it in-place first?
pub fn vec_add_second_partial(inp1: &[u8], inp2: &mut [u8], len: usize) -> usize {
    let vl: usize;

    assert!(len < inp1.len());
    assert!(len < inp2.len());

    // TODO: We need to mark in the signature that we use vector registers
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
        );
    }

    assert!(len == 0 || 0 < vl);

    return vl;
}

/* TODO: Might be easier to do this not in place */
fn vec_add_second(mut inp1: &[u8], mut inp2: &mut [u8], mut len: usize) {
    let mut vl: usize;
    while 0 < len {
        // TODO: We need to increase inp1 and inp2 by vl here
        vl = vec_add_second_partial(inp1, inp2, len);
        /* TODO (Loop invariant): Say that we have the addition we want */
        assert!(0 < vl);
        inp1 = &inp1[vl..];
        inp2 = &mut inp2[vl..];
        len -= vl;
    }

    assert!(len == 0);
}


// Proofs ------------------------------------------------------------------------------------------

#[cfg(kani)]
#[kani::proof]
fn check_vsetvli_non_zero() {
    let len: usize = 4;
    let vl = vsetvli(len);
    assert!(len == 0 || 0 < vl);
}

#[cfg(kani)]
#[kani::proof]
fn check_vec_add_second_partial() {
    const LEN: usize = 4;
    const LEN_COPY: usize = 4;
    let inp1: [u8; LEN] = kani::any();
    let mut inp2: [u8; LEN] = kani::any();
    let old_inp2 = inp2.clone();
    for i in 0..LEN {
        assert_eq!(old_inp2[i], inp2[i]);
    }
    kani::assume(LEN_COPY < LEN);
    kani::assume(LEN == inp1.len());
    kani::assume(LEN == inp2.len());
    let vl = vec_add_second_partial(&inp1, &mut inp2, LEN_COPY);
    for i in 0..vl {
        assert_eq!(inp2[i], inp1[i] + old_inp2[i]);
    }
}

#[cfg(kani)]
#[kani::proof]
fn check_vec_add_second() {
    const LEN: usize = 4;
    const LEN_COPY: usize = 4;
    let inp1: [u8; LEN] = kani::any();
    let mut inp2: [u8; LEN] = kani::any();
    let old_inp2 = inp2.clone();
    for i in 0..LEN {
        assert_eq!(old_inp2[i], inp2[i]);
    }
    kani::assume(LEN_COPY < LEN);
    kani::assume(LEN == inp1.len());
    kani::assume(LEN == inp2.len());
    vec_add_second(&inp1, &mut inp2, LEN_COPY);
    for i in 0..LEN_COPY {
        assert_eq!(inp2[i], inp1[i] + old_inp2[i]);
    }
}

fn main() {}
