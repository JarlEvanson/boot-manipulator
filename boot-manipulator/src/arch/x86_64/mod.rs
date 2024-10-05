//! Definitions and code of `x86_64` specific items.

use crate::arch::{ArchitectureOps, VirtualizationOps};

/// Implementation of [`ArchitectureOps`] for the `x86_64` environment.
pub struct X86_64;

impl ArchitectureOps for X86_64 {
    type Virtualization = Virtualization;
}

/// Implementation of [`VirtualizationOps`] for the `x86_64` architecture.
pub struct Virtualization;

impl VirtualizationOps for Virtualization {}
