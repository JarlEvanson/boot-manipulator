//! Architecture and boot environment independent implementation of the hypervisor.

use core::{
    error, fmt, mem,
    sync::atomic::{AtomicPtr, AtomicU8, AtomicUsize, Ordering},
};

use crate::{
    arch::{Architecture, ArchitectureOps, VirtualizationOps},
    boot::{BootInterface, BootOps},
};

/// Helper type to reduce typing when using [`Virtualization`].
type Virtualization = <Architecture as ArchitectureOps>::Virtualization;

/// The hypervisor is unitialized.
const HYPERVISOR_STATE_UNINITIALIZED: u8 = 0;
/// The hypervisor is starting.
const HYPERVISOR_STATE_STARTING: u8 = 1;
/// The hypervisor is active.
const HYPERVISOR_STATE_ACTIVE: u8 = 2;

/// The current state of the hypervisor.
static HYPERVISOR_STATE: AtomicU8 = AtomicU8::new(HYPERVISOR_STATE_UNINITIALIZED);
/// The number of processors this hypervisor controls.
static PROCESSOR_COUNT: AtomicUsize = AtomicUsize::new(0);
/// The processor specific state.
static PROCESSOR_STATE: AtomicPtr<<Virtualization as VirtualizationOps>::ProcessorState> =
    AtomicPtr::new(core::ptr::null_mut());

/// Initializes the hypervisor.
///
/// # Errors
/// - Returns [`HypervisorInitializationError::AlreadyActive`] when the hypervisor is already in
///     use.
pub fn initialize() -> Result<(), HypervisorInitializationError> {
    'anti_race_loop: {
        let mut state = HYPERVISOR_STATE.load(Ordering::Relaxed);
        while state == HYPERVISOR_STATE_UNINITIALIZED {
            match HYPERVISOR_STATE.compare_exchange_weak(
                state,
                HYPERVISOR_STATE_STARTING,
                Ordering::Acquire,
                Ordering::Relaxed,
            ) {
                Ok(_) => break 'anti_race_loop,
                Err(new_state) => state = new_state,
            }
        }

        return Err(HypervisorInitializationError::AlreadyActive);
    }

    let processor_count = BootInterface::get_processor_count();
    PROCESSOR_COUNT.store(processor_count, Ordering::Relaxed);
    log::debug!("Detected {processor_count} processors");

    let size =
        processor_count * mem::size_of::<<Virtualization as VirtualizationOps>::ProcessorState>();

    let frame = BootInterface::allocate_frames(size.div_ceil(4096)).unwrap();
    let ptr = BootInterface::map_frames(frame, size.div_ceil(4096))
        .unwrap()
        .as_ptr()
        .cast::<<Virtualization as VirtualizationOps>::ProcessorState>();

    PROCESSOR_STATE.store(ptr, Ordering::Relaxed);

    BootInterface::execute_on_all_processors(initialize_processor_impl, core::ptr::null_mut());

    Ok(())
}

/// Various errors that can occur while setting up the hypervisor.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum HypervisorInitializationError {
    /// The hypervisor is already active.
    AlreadyActive,
}

impl fmt::Display for HypervisorInitializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyActive => write!(f, "hypervisor is already active"),
        }
    }
}

impl error::Error for HypervisorInitializationError {}

/// Implementation detail of [`initialize_processor()`], which helps pretty print any errors that
/// occur.
fn initialize_processor_impl(_: *mut core::ffi::c_void) {
    match initialize_processor() {
        Ok(()) => {}
        Err(error) => {
            log::error!(
                "Error initializing hypervisor on processor {}: {error}",
                BootInterface::processor_identity()
            );
        }
    }
}

/// Performs processor initialization.
fn initialize_processor() -> Result<(), InitializeProcessorError> {
    let processor_id = BootInterface::processor_identity();

    let supported = Virtualization::is_supported().ok_or(InitializeProcessorError::NotSupported)?;

    let processor_state = Virtualization::initialize_processor(supported)?;

    // SAFETY:
    // `PROCESSOR_STATE` points to an array of `ProcessorState` structs.
    let ptr = unsafe { PROCESSOR_STATE.load(Ordering::Relaxed).add(processor_id) };
    // SAFETY:
    // This processor has exclusive access to this `ProcessorState` right now.
    unsafe { *ptr = processor_state };

    Ok(())
}

/// Various errors that can occur while initializing a processor's hypervisor support.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum InitializeProcessorError {
    /// The processor does not support virtualization.
    NotSupported,
    /// An error occurred while initializing virtualization.
    InitializationError(<Virtualization as VirtualizationOps>::InitializeProcessorError),
}

impl From<<Virtualization as VirtualizationOps>::InitializeProcessorError>
    for InitializeProcessorError
{
    fn from(value: <Virtualization as VirtualizationOps>::InitializeProcessorError) -> Self {
        Self::InitializationError(value)
    }
}

impl fmt::Display for InitializeProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotSupported => writeln!(f, "virtualization is not supported"),
            Self::InitializationError(error) => {
                writeln!(f, "error initializing virtualization: {error}")
            }
        }
    }
}
