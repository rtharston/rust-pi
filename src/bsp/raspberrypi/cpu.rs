//! Processor code.

/// Used by `arch` code to determine which core will run boot code.
#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static BOOT_CORE_ID: u64 = 0;

#[path = "../../_arch/aarch64/cpu/mod.rs"]
mod arch_cpu;

/// Include boot to get _start
// mod boot;

pub use arch_cpu::wait_forever;