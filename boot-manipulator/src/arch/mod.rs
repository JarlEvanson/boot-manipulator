//! Abstracts platform specific code.

#[cfg(target_arch = "x86")]
mod x86;

#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86_common;

/// The active [`Architecture`].
#[cfg(target_arch = "x86")]
pub type Architecture = x86::X86;

/// The active [`Architecture`].
#[cfg(target_arch = "x86_64")]
pub type Architecture = x86_64::X86_64;

/// The active [`Architecture`].
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub type Architecture = DummyArch;

/// Describes the basic set of architecture APIs required for setting up `boot-manipulator`.
pub trait ArchitectureOps {
    /// Architectural virtualization APIs.
    type Virtualization: VirtualizationOps;
}

/// Describes the basic set of virtualization APIs required for setting up `boot-manipulator`'s
/// hypervisor.
pub trait VirtualizationOps {}

/// Dummy architecture to allow for easier development.
pub struct DummyArch;

impl ArchitectureOps for DummyArch {
    type Virtualization = DummyVirtualization;
}

/// Dummy virtualization implementation to allow for easier development.
pub struct DummyVirtualization;

impl VirtualizationOps for DummyVirtualization {}
