//! UEFI boot manipulation tool.

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::fmt;

use uefi::{boot, Status};

pub mod hypervisor;
pub mod logging;
pub mod spinlock;

#[uefi::entry]
fn entry_point() -> Status {
    logging::initialize_logging(log::LevelFilter::Trace);

    let (image_base, image_size) = match get_image_info() {
        Ok(image_info) => image_info,
        Err(error) => {
            log::error!("Error when getting image info: {error}");
            return Status::LOAD_ERROR;
        }
    };

    log::debug!("Image Base: {image_base:p} Image Size: 0x{image_size:x}");
    match hypervisor::initialize() {
        Ok(()) => {}
        Err(error) => {
            log::error!("{error}");
            return Status::LOAD_ERROR;
        }
    }

    Status::SUCCESS
}

/// Returns the number of processors detected.
fn get_processor_count() -> usize {
    use uefi::proto::pi::mp::MpServices;

    let Ok(handle) = boot::get_handle_for_protocol::<MpServices>() else {
        return 1;
    };

    let Ok(mp_services) = boot::open_protocol_exclusive::<MpServices>(handle) else {
        return 1;
    };

    let Ok(processor_count) = mp_services.get_number_of_processors() else {
        return 1;
    };

    processor_count.total
}

/// Returns the loaded `boot-manipulator`'s image base and size.
fn get_image_info() -> Result<(*const u8, usize), GetImageInfoError> {
    let loaded_image = boot::open_protocol_exclusive::<uefi::proto::loaded_image::LoadedImage>(
        boot::image_handle(),
    )
    .map_err(GetImageInfoError::LoadedImage)?;

    let (image_base, image_size) = loaded_image.info();

    Ok((image_base.cast::<u8>(), image_size.try_into().unwrap()))
}

/// Various errors that can occur while retrieving information about the loaded `boot-manipulator`.
#[derive(Clone, Debug, PartialEq, Eq)]
enum GetImageInfoError {
    /// Failed to open `LoadedImage` protocol.
    LoadedImage(uefi::Error),
}

impl fmt::Display for GetImageInfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LoadedImage(error) => write!(f, "failed to open LoadedImage protocol: {error}",),
        }
    }
}

/// Handles panics occurring in `boot-manipulator`.
#[cfg_attr(not(test), panic_handler)]
#[cfg_attr(test, allow(unused))]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    log::error!("{info}");

    loop {}
}
