//! Hypervisor and proxy server used to control and explore various architecture and device's
//! interactions.

#![no_std]
#![no_main]

use core::{error, fmt};

pub mod arch;
pub mod boot;
pub mod logging;
pub mod spinlock;

/// The default logging level.
pub const DEFAULT_LOG_LEVEL: log::LevelFilter = log::LevelFilter::Trace;

/// Main initialization function for `boot-manipulator`.
///
/// # Errors
pub fn main() -> Result<(), InitializationError> {
    Ok(())
}

/// Various errors that can occur while initializing `boot-manipulator`.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum InitializationError {}

impl fmt::Display for InitializationError {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        unreachable!()
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
