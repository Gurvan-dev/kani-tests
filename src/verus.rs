use vstd::prelude::*;

use softcore_asm_rv64::softcore_init;
use softcore_rv64::prelude::bvd;
use softcore_rv64::{Core, config, new_core};

verus! {
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

    pub fn check_vsetvli_non_zero() {
        let len: usize = 4;
        let vl = vsetvli(len);
        assert(len == 0 || 0 < vl);
    }
}
