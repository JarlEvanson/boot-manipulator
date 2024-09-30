//! Definitions and interfaces dealing with hypervisor construction, control, and cleanup.

use core::{
    error, fmt,
    sync::atomic::{AtomicU8, AtomicUsize, Ordering},
};

use crate::get_processor_count;

/// The hypervisor is unitialized.
const HYPERVISOR_STATE_UNINITIALIZED: u8 = 0;
/// The hypervisor is starting.
const HYPERVISOR_STATE_STARTING: u8 = 1;
/// The hypervisor is active.
const HYPERVISOR_STATE_ACTIVE: u8 = 2;
/// The hypervisor is stopping.
const HYPERVISOR_STATE_STOPPING: u8 = 3;

/// The current state of the hypervisor.
static HYPERVISOR_STATE: AtomicU8 = AtomicU8::new(HYPERVISOR_STATE_UNINITIALIZED);
/// The number of processors this hypervisor controls.
static PROCESSOR_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Initializes the hypervisor.
///
/// # Errors
/// - Returns [`HypervisorInitializationError::AlreadyActive`] when the hypervisor is already in
///     use.
pub fn initialize() -> Result<(), HypervisorInitializationError> {
    'anti_reentrancy_loop: {
        let mut state = HYPERVISOR_STATE.load(Ordering::Relaxed);
        while state == HYPERVISOR_STATE_UNINITIALIZED {
            match HYPERVISOR_STATE.compare_exchange_weak(
                state,
                HYPERVISOR_STATE_STARTING,
                Ordering::Acquire,
                Ordering::Relaxed,
            ) {
                Ok(_) => break 'anti_reentrancy_loop,
                Err(new_state) => state = new_state,
            }
        }

        return Err(HypervisorInitializationError::AlreadyActive);
    }

    let processor_count = get_processor_count();
    PROCESSOR_COUNT.store(processor_count, Ordering::Relaxed);
    log::debug!("Detected {processor_count} processors");

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
