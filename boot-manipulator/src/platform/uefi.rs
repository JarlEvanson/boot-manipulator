//! UEFI specific code.

use uefi::Status;

use crate::platform::PlatformOps;

#[uefi::entry]
fn entry() -> Status {
    Status::SUCCESS
}

/// Implementation of [`PlatformOps`] for the UEFI environment.
pub struct Uefi;

impl PlatformOps for Uefi {}
