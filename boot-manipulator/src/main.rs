//! UEFI boot manipulation tool.

#![no_std]
#![no_main]

use core::{fmt, ptr};

use arch::virtualization;

mod arch;
pub mod console;
mod logging;
mod spinlock;

static mut EXIT_BOOT_SERVICES_PTR: unsafe extern "efiapi" fn(
    *mut core::ffi::c_void,
    usize,
) -> uefi::Status = placeholder;

#[uefi::entry]
fn entry_point() -> uefi::Status {
    logging::initialize_logging(log::LevelFilter::Trace);

    match setup() {
        Ok(()) => {}
        Err(error) => {
            log::error!("{error}");
            uefi::boot::stall(10_000_000);
            return uefi::Status::LOAD_ERROR;
        }
    }

    log::info!("boot-manipulator successfully loaded");

    uefi::Status::SUCCESS
}

fn setup() -> Result<(), DriverSetupError> {
    setup_boot_services_interception();

    virtualization::allocate_basic_memory();

    Ok(())
}

/// Various errors that can occur while setting up the driver.
pub enum DriverSetupError {}

impl fmt::Display for DriverSetupError {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _ => Ok(()),
        }
    }
}

fn setup_boot_services_interception() {
    let system_table_ptr = uefi::table::system_table_raw()
        .map(|ptr| ptr.as_ptr())
        .unwrap_or(ptr::null_mut());

    let boot_services_table_ptr = unsafe { (*system_table_ptr).boot_services };
    let exit_boot_services_func = unsafe { &mut ((*boot_services_table_ptr).exit_boot_services) };

    unsafe { EXIT_BOOT_SERVICES_PTR = *exit_boot_services_func };
    *exit_boot_services_func = exit_boot_services;
}

unsafe extern "efiapi" fn exit_boot_services(
    image_handle: *mut core::ffi::c_void,
    map_key: usize,
) -> uefi::Status {
    let func = unsafe { EXIT_BOOT_SERVICES_PTR };

    let result = unsafe { (func)(image_handle, map_key) };
    if result != uefi::Status::SUCCESS {
        return result;
    }

    if !virtualization::is_supported() {
        panic!("Virtualization not supported");
    }

    virtualization::enable_support();
    log::info!("VMX successfully entered");

    loop {}
}

unsafe extern "efiapi" fn placeholder(_: *mut core::ffi::c_void, _: usize) -> uefi::Status {
    panic!("exit_boot_services placeholder reached")
}

#[cfg_attr(not(test), panic_handler)]
#[allow(unused)]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    log::error!("{info}");

    loop {}
}
