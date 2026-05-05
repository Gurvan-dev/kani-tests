#[cfg(any(test, feature = "userspace"))]
use softcore_rv64::{Core, config, new_core};

// Each thread gets its own copy of the core, this prevent tests using different threads inside a
// same process to share the same core.
#[cfg(any(test, feature = "userspace"))]
std::thread_local! {
    /// A software RISC-V core that emulates a real CPU.
    ///
    /// When running Miralis in user-space (on any architecture) we use an in-process software
    /// implementation of a RISC-V core. We perform all privileged operations that would normally
    /// require assembly against that software core. For instance, all CSR operations are executed
    /// against the software core.
    ///
    /// The software core can be queried, for instance to check which addresses are currently
    /// protected by the PMP, or if there are any pending interrupts. This enables testing how
    /// Miralis interacts with the hardware directly from unit tests, rather than within QEMU.
    ///
    /// We use one core per thread to prevent interference among threads, such as when running
    /// `cargo test`. Therefore, the core lives in threat local storage and must be access using
    /// the `thread_loca!` API.
    ///
    /// Usage:
    ///
    /// ```
    /// SOFT_CORE.with_borrow_mut(|core| {
    ///     // The `core` can be accessed within the closure
    ///     core.set(reg::X1, 0x42);
    ///     core.csrrw(reg::X0, csr::MSCRATCH, reg::X1).unwrap();
    /// });
    /// ```
    pub static SOFT_CORE: core::cell::RefCell<Core> = {
        let mut core = new_core(config::U74);
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
            softcore(SOFT_CORE.with_borrow_mut),
            softcore_trap_handlers(_tracing_trap_handler, _mprv_trap_handler, _raw_trap_handler)
        );

        #[cfg(any(test, feature = "userspace"))]
        _unsafe_marker();
    }};
}
