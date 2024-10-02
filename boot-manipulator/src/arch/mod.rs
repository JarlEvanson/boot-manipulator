//! Abstracts platform specific code.

#[cfg(target_arch = "x86_64")]
mod x86_64;

/// The active [`Architecture`].
#[cfg(target_arch = "x86_64")]
pub type Architecture = x86_64::X86_64;

/// The active [`Architecture`].
#[cfg(not(target_arch = "x86_64"))]
pub type Architecture = DummyArch;

/// Describes the basic set of architecture APIs required for setting up `boot-manipulator`.
pub trait ArchitectureOps {
    /// Architectural virtualization APIs.
    type Virtualization: VirtualizationOps;
}

/// Describes the basic set of virtualization APIs required for setting up `boot-manipulator`.
pub trait VirtualizationOps {
    /// Returns `true` if virtualization is supported on this processor; otherwise returns `false`.
    fn is_supported() -> bool;
}

/// Dummy architecture to allow for easier development.
pub struct DummyArch;

impl ArchitectureOps for DummyArch {
    type Virtualization = DummyVirtualization;
}

/// Dummy virtualization implementation to allow for easier development.
pub struct DummyVirtualization;

impl VirtualizationOps for DummyVirtualization {
    fn is_supported() -> bool {
        unimplemented!()
    }
}
