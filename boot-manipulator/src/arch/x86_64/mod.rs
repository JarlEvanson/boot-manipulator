//! Definitions and code of `x86_64` specific items.

use crate::arch::ArchitectureOps;

/// Implementation of [`ArchitectureOps`] for the `x86_64` environment.
pub struct X86_64;

impl ArchitectureOps for X86_64 {}
