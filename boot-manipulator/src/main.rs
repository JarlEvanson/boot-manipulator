//! Hypervisor and proxy server used to control and explore various architecture and device's
//! interactions.

#![no_std]
#![no_main]

pub mod boot;
pub mod logging;
pub mod spinlock;

/// The default logging level.
pub const DEFAULT_LOG_LEVEL: log::LevelFilter = log::LevelFilter::Trace;

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
