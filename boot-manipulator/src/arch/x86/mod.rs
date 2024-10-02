//! Definitions of `x86` specific items

mod virtualization;

use crate::arch::ArchitectureOps;

/// Implementation of [`ArchitectureOps`] for the `x86` environment.
pub struct X86;

impl ArchitectureOps for X86 {
    type Virtualization = virtualization::Multiplexer;
}
