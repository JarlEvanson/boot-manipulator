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

    /// Returns the identity of the calling processor.
    ///
    /// The identity is a number from `0..get_processor_count()`.
    fn processor_identity() -> usize;

    /// Returns the total number of logical processors in the system.
    ///
    /// # Panics
    ///
    /// This function may panic if called on a processor other than the bootstrap processor.
    fn get_processor_count() -> usize;

    /// Executes the provided `function` on all processors.
    ///
    /// # Panics
    ///
    /// This function may panic if called on a processor other than the bootstrap processor.
    /// This function may panic if called in a nested manner.
    fn execute_on_all_processors(
        function: fn(*mut core::ffi::c_void),
        argument: *mut core::ffi::c_void,
    );
}

/// Dummy boot structure to allow for development.
pub struct DummyBootInterface;

impl BootOps for DummyBootInterface {
    type LoggingInitializationError = core::fmt::Error;

    fn initialize_logger() -> Result<&'static dyn log::Log, Self::LoggingInitializationError> {
        unimplemented!()
    }

    fn processor_identity() -> usize {
        unimplemented!()
    }

    fn get_processor_count() -> usize {
        unimplemented!()
    }

    fn execute_on_all_processors(_: fn(*mut core::ffi::c_void), _: *mut core::ffi::c_void) {
        unimplemented!()
    }
}
