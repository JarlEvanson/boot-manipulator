//! UEFI boot manipulation tool.

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use uefi::Status;

#[uefi::entry]
fn entry_point() -> Status {
    Status::SUCCESS
}

#[cfg_attr(not(test), panic_handler)]
#[cfg_attr(test, allow(unused))]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
