//! Definitions of interfaces for Model Specific Registers.

use core::arch::asm;

/// Reads a [`u64`] from the msr at location `msr`.
///
/// # Safety
/// - `msr` must exist.
pub unsafe fn read_msr(msr: u32) -> u64 {
    let rax: u64;
    let rdx: u64;

    unsafe { asm!("rdmsr", in("ecx") msr, lateout("eax") rax, lateout("edx") rdx) }

    (rax as u64) | ((rdx as u64) << 32)
}

/// Writes `value` to the msr at location `msr`.
///
/// # Safety
/// - `msr` must exist.
/// - `value` must be a valid value for `msr` and must not break assumptions.
pub unsafe fn write_msr(msr: u32, value: u64) {
    unsafe {
        asm!("wrmsr", in("ecx") msr, in("eax") value as u32, in("edx") ((value >> 32) as u32))
    }
}

pub const FEATURE_CONTROL: u32 = 0x3a;

pub const VMX_REVISION: u32 = 0x480;

pub const VMX_CR0_FIXED0: u32 = 0x486;
pub const VMX_CR0_FIXED1: u32 = 0x487;

pub const VMX_CR4_FIXED0: u32 = 0x488;
pub const VMX_CR4_FIXED1: u32 = 0x489;
