//! UEFI boot manipulation tool.

#![no_std]
#![no_main]

use core::{
    fmt::Write,
    ptr,
};

use arch::virtualization;
use uefi::{boot, proto::console::serial::Serial};

mod arch;
pub mod console;
mod spinlock;

static EXIT_SERVICES: spinlock::Spinlock<
    unsafe extern "efiapi" fn(*mut core::ffi::c_void, usize) -> uefi::Status,
> = spinlock::Spinlock::new(placeholder);

#[uefi::entry]
fn entry_point() -> uefi::Status {
    match setup() {
        uefi::Status::SUCCESS => {}
        status_code => {
            uefi::boot::stall(10_000_000);
            return status_code;
        }
    }

    let system_table_ptr = uefi::table::system_table_raw()
        .map(|ptr| ptr.as_ptr())
        .unwrap_or(ptr::null_mut());

    let boot_services_table_ptr = unsafe { (*system_table_ptr).boot_services };
    let exit_boot_services_func_ptr =
        unsafe { ptr::addr_of_mut!((*boot_services_table_ptr).exit_boot_services) };

    *EXIT_SERVICES.lock() = unsafe { *exit_boot_services_func_ptr };
    unsafe { *exit_boot_services_func_ptr = exit_boot_services };

    virtualization::allocate_basic_memory();

    let _ = unsafe { boot::exit_boot_services(boot::MemoryType::LOADER_DATA) };

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

    let _ = serial.write_str("Testing");

    uefi::Status::SUCCESS
}

struct Debugcon;

impl Write for Debugcon {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            unsafe { core::arch::asm!("out dx, al", in("dx") 0xe9, in("al") byte) }
        }

        Ok(())
    }
}

unsafe extern "efiapi" fn exit_boot_services(
    image_handle: *mut core::ffi::c_void,
    map_key: usize,
) -> uefi::Status {
    let func = *EXIT_SERVICES.lock();

    let result = unsafe { (func)(image_handle, map_key) };
    if result != uefi::Status::SUCCESS {
        let _ = writeln!(Debugcon, "Exit failed: {result}");
        return result;
    }

    if !virtualization::is_supported() {
        panic!("Virtualization not supported");
    }

    virtualization::enable_support(&mut Debugcon);
    let _ = writeln!(Debugcon, "Virtualization succeeded");

    loop {}
}

unsafe extern "efiapi" fn placeholder(_: *mut core::ffi::c_void, _: usize) -> uefi::Status {
    panic!("exit_boot_services placeholder reached")
}

#[cfg_attr(not(test), panic_handler)]
#[allow(unused)]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if uefi::table::system_table_boot().is_some() {
        uefi::system::with_stdout(|stdout| writeln!(stdout, "{info}"));
    }

    writeln!(Debugcon, "{info}");

    loop {}
}
