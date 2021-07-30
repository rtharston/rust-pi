//! BSP Processor code.

/// Used by `arch` code to determine which core will run boot code.
#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static BOOT_CORE_ID: u64 = 0;