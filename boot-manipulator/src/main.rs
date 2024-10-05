//! Hypervisor and proxy server used to control and explore various architecture and device's
//! interactions.

#![no_std]
#![no_main]

use core::{error, fmt};

use hypervisor::HypervisorInitializationError;

pub mod arch;
pub mod hypervisor;
pub mod logging;
pub mod platform;
pub mod polyfill;
pub mod spinlock;

/// The default logging level.
pub const DEFAULT_LOG_LEVEL: log::LevelFilter = log::LevelFilter::Trace;

/// Main initialization function for `boot-manipulator`.
///
/// # Errors
///
/// Returns [`InitializationError::HypervisorError`] when an error occurs while setting up the
/// hypervisor.
pub fn main() -> Result<(), InitializationError> {
    hypervisor::initialize()?;

    Ok(())
}

/// Various errors that can occur while initializing `boot-manipulator`.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum InitializationError {
    /// An error occured while initializing the hypervisor.
    HypervisorError(HypervisorInitializationError),
}

impl From<HypervisorInitializationError> for InitializationError {
    fn from(value: HypervisorInitializationError) -> Self {
        Self::HypervisorError(value)
    }
}

impl fmt::Display for InitializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HypervisorError(error) => {
                write!(f, "error while initializing hypervisor: {error}")
            }
        }
    }
}

impl error::Error for InitializationError {}

/// Handles panics that occur.
///
/// Currently executes a spin loop.
#[cfg_attr(not(test), panic_handler)]
#[cfg_attr(test, allow(unused))]
pub fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    log::error!("{info}");

    loop {
        core::hint::spin_loop()
    }
}
