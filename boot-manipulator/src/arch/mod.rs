//! Abstracts platform specific code.

use core::error;

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

    /// Disables interrupts.
    fn disable_interrupts();

    /// Enables interrupts.
    fn enable_interrupts();
}

/// Describes the basic set of virtualization APIs required for setting up `boot-manipulator`.
pub trait VirtualizationOps {
    /// Various errors that can occur during the initialization of a processor.
    type InitializeProcessorError: error::Error;

    /// Returns `true` if virtualization is supported on this processor; otherwise returns `false`.
    fn is_supported() -> bool;

    /// Initializes the virtualization technology on the calling processor.
    ///
    /// # Errors
    ///
    /// This function may return any errors that occur during the execution of this function.
    fn initialize_processor() -> Result<(), Self::InitializeProcessorError>;
}

/// Dummy architecture to allow for easier development.
pub struct DummyArch;

impl ArchitectureOps for DummyArch {
    type Virtualization = DummyVirtualization;

    fn disable_interrupts() {
        unimplemented!()
    }

    fn enable_interrupts() {
        unimplemented!()
    }
}

/// Dummy virtualization implementation to allow for easier development.
pub struct DummyVirtualization;

impl VirtualizationOps for DummyVirtualization {
    type InitializeProcessorError = core::fmt::Error;

    fn is_supported() -> bool {
        unimplemented!()
    }

    fn initialize_processor() -> Result<(), Self::InitializeProcessorError> {
        unimplemented!()
    }
}
