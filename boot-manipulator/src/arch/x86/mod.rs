//! Definitions and code of `x86_64` specific items.

use crate::arch::{
    x86_common::virtualization, ArchitectureOps, VirtualizationOps, VirtualizationSupported,
};

/// Implementation of [`ArchitectureOps`] for the `x86` environment.
pub struct X86;

impl ArchitectureOps for X86 {
    type Virtualization = Virtualization;
}

/// Implementation of [`VirtualizationOps`] for the `x86` architecture.
pub struct Virtualization;

impl VirtualizationOps for Virtualization {
    fn is_supported() -> Option<VirtualizationSupported> {
        virtualization::supported_technology().map(|_| {
            // SAFETY:
            // Virtualization is supported on this processor.
            unsafe { VirtualizationSupported::new() }
        })
    }
}
