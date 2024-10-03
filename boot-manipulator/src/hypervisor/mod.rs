//! Architecture and platform independent implementation of the hypervisor.

use core::{
    error, fmt,
    sync::atomic::{AtomicU8, AtomicUsize, Ordering},
};

use crate::{
    arch::{Architecture, ArchitectureOps, VirtualizationOps},
    platform::{Platform, PlatformOps},
};

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
/// The processor id of the bootstrap processor.
static BOOTSTRAP_PROCESSOR: AtomicUsize = AtomicUsize::new(0);

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

    let processor_count = Platform::get_processor_count();
    PROCESSOR_COUNT.store(processor_count, Ordering::Relaxed);
    log::debug!("Detected {processor_count} processors");

    BOOTSTRAP_PROCESSOR.store(Platform::processor_identity(), Ordering::Relaxed);

    Platform::execute_on_all_processors(initialize_processor, core::ptr::null_mut());

    todo!()
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

/// Performs processor initialization.
fn initialize_processor(_: *mut core::ffi::c_void) {
    let processor_id = Platform::processor_identity();
    let is_bootstrap = processor_id == BOOTSTRAP_PROCESSOR.load(Ordering::Relaxed);

    assert!(<Architecture as ArchitectureOps>::Virtualization::is_supported());
    <Architecture as ArchitectureOps>::Virtualization::initialize_processor().unwrap();
}
