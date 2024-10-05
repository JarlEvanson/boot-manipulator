//! Definitions and interfaces to interact with VMX.

use x86::instructions::cpuid::{cpuid, has_cpuid};

/// Bit indicating that VMX entensions are supported on this processor.
const VMXE_SUPPORTED_BIT: u32 = 1 << 5;

/// Returns `true` if virtualization is supported; otherwise returns `false`.
pub fn is_supported() -> bool {
    if !has_cpuid() {
        return false;
    }

    // SAFETY:
    // This processor supports `cpuid`.
    let ecx = unsafe { cpuid(1, 0) }.ecx;
    ecx & VMXE_SUPPORTED_BIT == VMXE_SUPPORTED_BIT
}
