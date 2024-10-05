//! Definitions and interfaces to interact with virtualization on `x86` and `x86_64` processors.

use x86::instructions::cpuid::{cpuid, has_cpuid};

pub mod vmx;

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
        b"GenuineIntel" if vmx::is_supported() => Technology::Vmx,
        _ => return None,
    };

    Some(technology)
}

/// The virtualization technologies supported on `x86` and `x86_64` processors.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Technology {
    /// Virtual Machine Extensions.
    Vmx,
}
