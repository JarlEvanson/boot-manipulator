//! UEFI specific code.

use uefi::Status;

use crate::boot::BootOps;

#[uefi::entry]
fn entry() -> Status {
    Status::SUCCESS
}

/// Implementation of [`BootOps`] for the UEFI environment.
pub struct Uefi;

impl BootOps for Uefi {}
