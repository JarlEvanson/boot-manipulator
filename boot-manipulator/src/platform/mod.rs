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

    /// Returns the identity of the calling processor.
    ///
    /// This number is in the range from 0 to [`PlatformOps::get_processor_count()`] - 1.
    fn processor_identity() -> usize {
        0
    }

    /// Returns the total number of logical processors in the system.
    ///
    /// # Panics
    ///
    /// This function may panic if called on a processor other than the bootstrap processor.
    fn get_processor_count() -> usize {
        // Simple implementation pretending there is only 1 processor.
        1
    }

    /// Executes the provided `function` on all processors.
    ///
    /// # Panics
    ///
    /// This function may panic if called on a processor other than the bootstrap processor.
    /// This function may panic if called in a nested manner.
    fn execute_on_all_processors(
        function: fn(*mut core::ffi::c_void),
        argument: *mut core::ffi::c_void,
    ) {
        // Simple implementation pretending there is only 1 processor.
        function(argument)
    }
}

/// Dummy platform to allow for development.
pub struct DummyPlatform;

impl PlatformOps for DummyPlatform {
    type LoggingInitializationError = core::fmt::Error;

    fn initialize_logger() -> Result<&'static dyn log::Log, Self::LoggingInitializationError> {
        unimplemented!()
    }
}
