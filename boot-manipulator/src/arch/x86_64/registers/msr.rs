//! Definitions of interfaces for Model Specific Registers.

use core::arch::asm;

/// Reads a [`u64`] from the msr at location `msr`.
pub fn read_msr(msr: u32) -> u64 {
    let rax: u64;
    let rdx: u64;
    
    unsafe { asm!("rdmsr", in("ecx") msr, lateout("eax") rax, lateout("edx") rdx) }

    (rax as u64) | ((rdx as u64) << 32)
}

/// Writes `value` to the msr at location `msr`.
///
/// # Safety
/// - `value` must be a valid value for `msr` and must not break assumptions.
pub unsafe fn write_msr(msr: u32, value: u64) {
    unsafe { 
        asm!("wrmsr", in("ecx") msr, in("eax") value as u32, in("edx") ((value >> 32) as u32)) 
    }
}
