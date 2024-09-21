//! UEFI boot manipulation tool.

#![no_std]
#![no_main]

use uefi::{boot, proto::console::serial::Serial};

pub mod console;

#[uefi::entry]
fn entry_point() -> uefi::Status {
    let serial = boot::get_handle_for_protocol::<Serial>().unwrap();
    let mut serial = boot::open_protocol_exclusive::<Serial>(serial).unwrap();


    loop {}
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
