//! Definitions and code of `x86_64` specific items.

mod virtualization;

use crate::arch::ArchitectureOps;

/// Implementation of [`ArchitectureOps`] for the `x86_64` environment.
pub struct X86_64;

impl ArchitectureOps for X86_64 {
    type Virtualization = virtualization::Multiplexer;

    fn disable_interrupts() {
        // SAFETY:
        // `cli` is safe to execute.
        unsafe { core::arch::asm!("cli", options(nomem, preserves_flags)) }
    }

    fn enable_interrupts() {
        // SAFETY:
        // `sti` is safe to execute.
        unsafe { core::arch::asm!("sti", options(nomem, preserves_flags)) }
    }
}
