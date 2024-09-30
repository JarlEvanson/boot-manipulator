//! Abstracts platform specific code.

#[cfg(target_os = "uefi")]
mod uefi;

/// The active [`Platform`].
#[cfg(target_os = "uefi")]
pub type Platform = uefi::Uefi;

/// The active [`Platform`].
#[cfg(not(target_os = "uefi"))]
pub type Platform = DummyPlatform;

/// Describes the basic set of platform APIs required for setting up `boot-manipulator`.
pub trait PlatformOps {}

/// Dummy platform to allow for development.
pub struct DummyPlatform;

impl PlatformOps for DummyPlatform {}
