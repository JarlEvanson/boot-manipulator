//! Definitions and interfaces to interact with virtualization on `x86` and `x86_64` processors.

use core::{error, fmt};

use x86::instructions::cpuid::{cpuid, has_cpuid};

use crate::boot::{BootInterface, BootOps};

pub mod vmx;

/// Initializes support for virtualization on this processor.
pub fn initialize_processor() -> Result<ProcessorState, InitializeProcessorError> {
    let processor_state = match supported_technology().unwrap() {
        Technology::Vmx(()) => {
            let frame = BootInterface::allocate_frames(1).unwrap();
            let ptr = BootInterface::map_frames(frame, 1).unwrap();

            // SAFETY:
            // - VMX is supported by this processor.
            // - `ptr` points to a memory region under the control of this processor which is 4096
            //      bytes.
            // `frame` is the physical base address of the frame to which `vmxon_ptr`'s page is
            //      mapped.
            let processor_state =
                unsafe { vmx::initialize_processor(ptr.as_ptr().cast::<vmx::Vmxon>(), frame)? };

            ProcessorState {
                internal: Technology::Vmx(processor_state),
            }
        }
    };

    Ok(processor_state)
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

/// Processor specific state associated with virtualization.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ProcessorState {
    /// Internal state related to virtualization support on a processor.
    internal: Technology<vmx::ProcessorState>,
}

/// Returns the [`Technology`] supported by this processor.
pub fn supported_technology() -> Option<Technology> {
    if !has_cpuid() {
        return None;
    }

    // SAFETY:
    // This processor supports `cpuid`.
    let processor_manufacturer = unsafe { cpuid(0, 0) };
    let processor_manufacturer = [
        processor_manufacturer.ebx.to_ne_bytes(),
        processor_manufacturer.edx.to_ne_bytes(),
        processor_manufacturer.ecx.to_ne_bytes(),
    ];
    let processor_manufacturer: &[u8] = processor_manufacturer.as_flattened();

    let technology = match processor_manufacturer {
        b"GenuineIntel" if vmx::is_supported() => Technology::Vmx(()),
        _ => return None,
    };

    Some(technology)
}

/// The virtualization technologies supported on `x86` and `x86_64` processors.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Technology<T = ()> {
    /// Virtual Machine Extensions.
    Vmx(T),
}
