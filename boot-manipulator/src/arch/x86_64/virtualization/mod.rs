//! Definitions and interfaces to interact with virtualization.

use x86::instructions::cpuid::{cpuid, has_cpuid};

use crate::arch::VirtualizationOps;

mod vmx;

/// Multiplexer over the various virtualization implementations offered on the `x86_64`
/// architecture.
pub struct Multiplexer;

impl VirtualizationOps for Multiplexer {
    fn is_supported() -> bool {
        supported_technology().is_some()
    }
}

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
