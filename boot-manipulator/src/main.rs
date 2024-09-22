//! UEFI boot manipulation tool.

#![no_std]
#![no_main]

use uefi::{boot, proto::console::serial::Serial};

pub mod console;
mod spinlock;

#[uefi::entry]
fn entry_point() -> uefi::Status {
    match setup() {
        uefi::Status::SUCCESS => {}
        status_code => {
            uefi::boot::stall(10_000_000);
            return status_code;
        }
    }

    loop {}
}

fn setup() -> uefi::Status {
    let serial_handle = match boot::get_handle_for_protocol::<Serial>() {
        Ok(handle) => handle,
        Err(error) => {
            let _ = uefi::system::with_stdout(|stdout| {
                writeln!(stdout, "failed to acquire serial device: {error}")
            });
            return uefi::Status::LOAD_ERROR;
        }
    };
    let mut serial = match boot::open_protocol_exclusive::<Serial>(serial_handle) {
        Ok(protocol) => protocol,
        Err(error) => {
            let _ = uefi::system::with_stdout(|stdout| {
                writeln!(stdout, "failed to open serial protocol: {error}")
            });
            return uefi::Status::LOAD_ERROR;
        }
    };

    uefi::Status::SUCCESS
}

#[cfg_attr(not(test), panic_handler)]
#[allow(unused)]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if uefi::table::system_table_boot().is_some() {
        uefi::system::with_stdout(|stdout| writeln!(stdout, "{info}"));
    }

    loop {}
}
