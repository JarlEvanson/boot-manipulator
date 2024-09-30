//! Abstracts the boot environment into [`BootOps`] used by the hypervisor to initialize.

#[cfg(target_os = "uefi")]
mod uefi;

/// The [`BootOps`] for this boot environment.
#[cfg(target_os = "uefi")]
pub type BootInterface = uefi::Uefi;

/// The [`BootOps`] for this boot environment.
#[cfg(not(target_os = "uefi"))]
pub type BootInterface = DummyBootInterface;

/// Describes the basic set of APIs required for setting up `boot-manipulator`.
pub trait BootOps {}

/// Dummy boot structure to allow for development.
pub struct DummyBootInterface;

impl BootOps for DummyBootInterface {}
