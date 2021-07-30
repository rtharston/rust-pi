//! Processor code.

#[path = "_arch/aarch64/cpu.rs"]
mod arch_cpu;

mod boot;

pub use arch_cpu::wait_forever;