//! Abstracts the boot environment into [`BootOps`] used by the hypervisor to initialize.

use core::error;

#[cfg(target_os = "uefi")]
mod uefi;

/// The [`BootOps`] for this boot environment.
#[cfg(target_os = "uefi")]
pub type BootInterface = uefi::Uefi;

/// The [`BootOps`] for this boot environment.
#[cfg(not(target_os = "uefi"))]
pub type BootInterface = DummyBootInterface;

/// Describes the basic set of APIs required for setting up `boot-manipulator`.
pub trait BootOps {
    /// The type of error returned from [`BootOps::initialize_logger()`].
    type LoggingInitializationError: error::Error;

    /// Returns the boot logging API.
    ///
    /// This should only be called once.
    ///
    /// # Errors
    ///
    /// Returns [`Self::LoggingInitializationError`] if boot logger initialization fails.
    fn initialize_logger() -> Result<&'static dyn log::Log, Self::LoggingInitializationError>;
}

/// Dummy boot structure to allow for development.
pub struct DummyBootInterface;

impl BootOps for DummyBootInterface {
    type LoggingInitializationError = core::fmt::Error;

    fn initialize_logger() -> Result<&'static dyn log::Log, Self::LoggingInitializationError> {
        unimplemented!()
    }
}
