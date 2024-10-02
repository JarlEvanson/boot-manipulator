//! Definitions and code of `x86_64` specific items.

mod virtualization;

use crate::arch::ArchitectureOps;

/// Implementation of [`ArchitectureOps`] for the `x86_64` environment.
pub struct X86_64;

impl ArchitectureOps for X86_64 {
    type Virtualization = virtualization::Multiplexer;
}
