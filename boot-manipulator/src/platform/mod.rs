//! Abstracts platform specific code.

use core::error;

#[cfg(target_os = "uefi")]
mod uefi;

/// The active [`Platform`].
#[cfg(target_os = "uefi")]
pub type Platform = uefi::Uefi;

/// The active [`Platform`].
#[cfg(not(target_os = "uefi"))]
pub type Platform = DummyPlatform;

/// Describes the basic set of platform APIs required for setting up `boot-manipulator`.
pub trait PlatformOps {
    /// The type of error returned from [`PlatformOps::initialize_logger()`].
    type LoggingInitializationError: error::Error;

    /// Returns the platform logging API.
    ///
    /// This should only be called once.
    ///
    /// # Errors
    ///
    /// Returns [`Self::LoggingInitializationError`] if platform logger initialization fails.
    fn initialize_logger() -> Result<&'static dyn log::Log, Self::LoggingInitializationError>;
}

/// Dummy platform to allow for development.
pub struct DummyPlatform;

impl PlatformOps for DummyPlatform {
    type LoggingInitializationError = core::fmt::Error;

    fn initialize_logger() -> Result<&'static dyn log::Log, Self::LoggingInitializationError> {
        unimplemented!()
    }
}
