//! Abstracts platform specific code.

use core::{error, marker::PhantomData};

#[cfg(target_arch = "x86")]
mod x86;

#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86_common;

/// The active [`Architecture`].
#[cfg(target_arch = "x86")]
pub type Architecture = x86::X86;

/// The active [`Architecture`].
#[cfg(target_arch = "x86_64")]
pub type Architecture = x86_64::X86_64;

/// The active [`Architecture`].
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub type Architecture = DummyArch;

/// Describes the basic set of architecture APIs required for setting up `boot-manipulator`.
pub trait ArchitectureOps {
    /// Architectural virtualization APIs.
    type Virtualization: VirtualizationOps;
}

/// Describes the basic set of virtualization APIs required for setting up `boot-manipulator`'s
/// hypervisor.
pub trait VirtualizationOps {
    /// Various errors that can occur during the initialization of a processor.
    type InitializeProcessorError: error::Error;

    /// Processor specific state associated with virtualization.
    type ProcessorState;

    /// Returns [`VirtualizationSupported`] if virtualization is supported on the calling
    /// processor.
    fn is_supported() -> Option<VirtualizationSupported>;

    /// Initializes the virtualization technology on the calling processor.
    ///
    /// # Errors
    ///
    /// This function may return any errors that occur during the execution of this function.
    fn initialize_processor(
        supported: VirtualizationSupported,
    ) -> Result<Self::ProcessorState, Self::InitializeProcessorError>;
}

/// Dummy architecture to allow for easier development.
pub struct DummyArch;

impl ArchitectureOps for DummyArch {
    type Virtualization = DummyVirtualization;
}

/// Marker type indicating that the processor supports virtualization.
pub struct VirtualizationSupported(PhantomData<*mut ()>);

impl VirtualizationSupported {
    /// Returns a new [`VirtualizationSupported`].
    ///
    /// # Safety
    ///
    /// This function must not be called on a processor that does not support virtualization.
    pub const unsafe fn new() -> Self {
        Self(PhantomData)
    }
}

/// Dummy virtualization implementation to allow for easier development.
pub struct DummyVirtualization;

impl VirtualizationOps for DummyVirtualization {
    type InitializeProcessorError = core::fmt::Error;

    type ProcessorState = ();

    fn is_supported() -> Option<VirtualizationSupported> {
        unimplemented!()
    }

    fn initialize_processor(
        _: VirtualizationSupported,
    ) -> Result<(), Self::InitializeProcessorError> {
        unimplemented!()
    }
}
