//! Hypervisor and proxy server used to control and explore various architecture and device's
//! interactions.

#![no_std]
#![no_main]

pub mod platform;

/// Handles panics that occur.
///
/// Currently executes a spin loop.
#[cfg_attr(not(test), panic_handler)]
#[cfg_attr(test, allow(unused))]
pub fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop()
    }
}
