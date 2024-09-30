//! UEFI boot manipulation tool.

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use uefi::Status;

pub mod logging;
pub mod spinlock;

#[uefi::entry]
fn entry_point() -> Status {
    logging::initialize_logging(log::LevelFilter::Trace);

    Status::SUCCESS
}

/// Handles panics occurring in `boot-manipulator`.
#[cfg_attr(not(test), panic_handler)]
#[cfg_attr(test, allow(unused))]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    log::error!("{info}");

    loop {}
}
