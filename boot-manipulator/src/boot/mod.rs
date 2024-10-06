//! Abstracts the boot environment into [`BootOps`] used by the hypervisor to initialize.

use core::{error, fmt, ptr::NonNull};

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

    /// The size of a frame from this [`BootOps`].
    const FRAME_SIZE: usize;

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
    fn map_frames(frame_base: u64, frame_count: usize) -> Result<NonNull<u8>, MapFailure>;

    /// Unmaps the `frame_count` frames located at `frame_base` from virtual memory.
    ///
    /// # Safety
    ///
    /// The caller must ensure that no references into the allocation remain, and that the memory
    /// at the allocation is not used after it is freed.
    unsafe fn unmap_frames(frame_base: u64, frame_count: usize);

    /// Deallocates the `frame_count` frames at `frame_base`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that no references into the allocation remain, and that the memory
    /// at the allocation is not used after it is freed.
    unsafe fn deallocate_frames(frame_base: u64, frame_count: usize);
}

/// The error returned from [`BootOps::allocate_frames()`] when the platform is out of memory to allocate.
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

impl From<OutOfMemoryError> for MapFailure {
    fn from(_: OutOfMemoryError) -> Self {
        Self::OutOfMemoryError
    }
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

/// Dummy boot structure to allow for development.
pub struct DummyBootInterface;

impl BootOps for DummyBootInterface {
    type LoggingInitializationError = core::fmt::Error;

    const FRAME_SIZE: usize = 0;

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

    fn allocate_frames(_: usize) -> Result<u64, OutOfMemoryError> {
        unimplemented!()
    }

    fn map_frames(_: u64, _: usize) -> Result<NonNull<u8>, MapFailure> {
        unimplemented!()
    }

    unsafe fn unmap_frames(_: u64, _: usize) {
        unimplemented!()
    }

    unsafe fn deallocate_frames(_: u64, _: usize) {
        unimplemented!()
    }
}
