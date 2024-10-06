//! Definitions and code of `x86_64` specific items.

use crate::arch::{
    x86_common::virtualization, ArchitectureOps, VirtualizationOps, VirtualizationSupported,
};

/// Implementation of [`ArchitectureOps`] for the `x86_64` environment.
pub struct X86_64;

impl ArchitectureOps for X86_64 {
    type Virtualization = Virtualization;
}

/// Implementation of [`VirtualizationOps`] for the `x86_64` architecture.
pub struct Virtualization;

impl VirtualizationOps for Virtualization {
    type InitializeProcessorError = virtualization::InitializeProcessorError;

    type ProcessorState = virtualization::ProcessorState;

    fn is_supported() -> Option<VirtualizationSupported> {
        virtualization::supported_technology().map(|_| {
            // SAFETY:
            // Virtualization is supported on this processor.
            unsafe { VirtualizationSupported::new() }
        })
    }

    fn initialize_processor(
        _: VirtualizationSupported,
    ) -> Result<Self::ProcessorState, Self::InitializeProcessorError> {
        virtualization::initialize_processor()
    }
}
