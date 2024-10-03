//! Definitions and interfaces to interact with virtualization.

//! Definitions and interfaces to interact with virtualization.

use core::{error, fmt};

use x86::instructions::cpuid::{cpuid, has_cpuid};

use crate::{
    arch::VirtualizationOps,
    platform::{Platform, PlatformOps},
};

mod vmx;

/// Multiplexer over the various virtualization implementations offered on the `x86_64`
/// architecture.
pub struct Multiplexer;

impl VirtualizationOps for Multiplexer {
    type InitializeProcessorError = InitializeProcessorError;

    fn is_supported() -> bool {
        supported_technology().is_some()
    }

    fn initialize_processor() -> Result<(), Self::InitializeProcessorError> {
        match supported_technology().unwrap() {
            Technology::Vmx => {
                let frame = Platform::allocate_frames(1).unwrap();
                let ptr = Platform::map_frames(frame, 1).unwrap();

                // SAFETY:
                // - `ptr` points to a region of memory under control of this processor that is
                //      4096 bytes.
                // - `frame` is the physical base address of the frame to which `ptr`'s page is
                //      mapped.
                unsafe { vmx::initialize_processor(ptr.cast::<vmx::Vmxon>(), frame)? }
            }
        }

        Ok(())
    }
}

/// Various errors that can occur while initializing virtualization on a processor.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum InitializeProcessorError {
    /// Various errors that can occur while initializing VMX support on a processor.
    Vmx(vmx::InitializeProcessorError),
}

impl From<vmx::InitializeProcessorError> for InitializeProcessorError {
    fn from(value: vmx::InitializeProcessorError) -> Self {
        Self::Vmx(value)
    }
}

impl fmt::Display for InitializeProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Vmx(error) => write!(f, "error initializing vmx: {error}"),
        }
    }
}

impl error::Error for InitializeProcessorError {}

/// Returns the virtualization [`Technology`] supported by this processor.
fn supported_technology() -> Option<Technology> {
    if !has_cpuid() {
        return None;
    }

    // SAFETY:
    // The processor supports `cpuid`.
    let processor_manufacturer = unsafe { cpuid(0, 0) };
    let processor_manufacturer = [
        processor_manufacturer.ebx.to_ne_bytes(),
        processor_manufacturer.edx.to_ne_bytes(),
        processor_manufacturer.ecx.to_ne_bytes(),
    ];
    let processor_manufacturer_str: &[u8] = processor_manufacturer.as_flattened();

    let technology = match processor_manufacturer_str {
        b"GenuineIntel" if vmx::is_supported() => Technology::Vmx,
        _ => return None,
    };

    Some(technology)
}

/// The virtualization technologies supported on `x86_64` processors.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Technology {
    /// Virtual Machine Extensions.
    Vmx,
}
