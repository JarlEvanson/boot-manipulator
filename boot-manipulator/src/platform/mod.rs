//! Abstracts platform specific code.

use core::{error, fmt};

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

    /// Allocates `frame_count` frames.
    ///
    /// # Errors
    ///
    /// Returns [`OutOfMemoryError`] when `frame_count` frames cannot be allocated.
    fn allocate_frames(frame_count: usize) -> Result<u64, OutOfMemoryError>;

    /// Maps the `frame_count` frames located at `frame_base` into virtual memory.
    ///
    /// # Errors
    ///
    /// - Returns [`MapFailure::OutOfMemoryError`] if additional memory needs to be allocated to map the
    ///     provided frames.
    /// - Returns [`MapFailure::VirtualMemoryFailure`] if an error occurs that deals with virtual
    ///     memory.
    fn map_frames(frame_base: u64, frame_count: usize) -> Result<*mut u8, MapFailure>;
}

/// The error returned from [`PlatformOps::allocate_frames`] when the platform is out of memory to allocate.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct OutOfMemoryError;

impl fmt::Display for OutOfMemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "frame allocation failed")
    }
}

impl error::Error for OutOfMemoryError {}

/// An error mapping the requested frames to a location in virtual memory.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum MapFailure {
    /// While allocating memory for the mapping, [`OutOfMemoryError`] was returned.
    OutOfMemoryError,
    /// An error occurred while attempting to map the frames into virtual memory.
    VirtualMemoryFailure,
}

impl fmt::Display for MapFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfMemoryError => write!(f, "allocation of supporting memory failed"),
            Self::VirtualMemoryFailure => write!(f, "virtual memory mapping failed"),
        }
    }
}

impl error::Error for MapFailure {}

/// Dummy platform to allow for development.
pub struct DummyPlatform;

impl PlatformOps for DummyPlatform {
    type LoggingInitializationError = core::fmt::Error;

    fn initialize_logger() -> Result<&'static dyn log::Log, Self::LoggingInitializationError> {
        unimplemented!()
    }

    fn allocate_frames(_: usize) -> Result<u64, OutOfMemoryError> {
        unimplemented!()
    }

    fn map_frames(_: u64, _: usize) -> Result<*mut u8, MapFailure> {
        unimplemented!()
    }
}
