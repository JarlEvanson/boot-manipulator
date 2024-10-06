//! Definitions and interfaces to interact with VMX.

use core::{error, fmt};

use x86::{
    instructions::cpuid::{cpuid, has_cpuid},
    registers::model_specific::{read_msr, write_msr},
};

/// Bit indicating that VMX entensions are supported on this processor.
const VMXE_SUPPORTED_BIT: u32 = 1 << 5;

/// The address of the feature control model specific register.
const FEATURE_CONTROL_ADDRESS: u32 = 0x3a;
/// The bit of the feature control model specific register that prevents any changes.
const FEATURE_CONTROL_LOCKED: u64 = 1;
/// The bit of the feature control model specific register that enables VMX operation outside of
/// SMX.
const FEATURE_CONTROL_VMX_OUTSIDE_SMX: u64 = 1 << 2;

/// The address of the VMX CR0 fixed 0s value.
const VMX_CR0_FIXED0_ADDRESS: u32 = 0x486;
/// The address of the VMX CR0 fixed 1s value.
const VMX_CR0_FIXED1_ADDRESS: u32 = 0x487;

/// The address of the VMX CR4 fixed 0s value.
const VMX_CR4_FIXED0_ADDRESS: u32 = 0x488;
/// The address of the VMX CR4 fixed 1s value.
const VMX_CR4_FIXED1_ADDRESS: u32 = 0x489;

/// The address of the VMX basic model specific register, which indicates various capabilities of
/// the VMX extensions on the queried processor.
const VMX_BASIC_ADDRESS: u32 = 0x480;

/// Returns `true` if virtualization is supported; otherwise returns `false`.
pub fn is_supported() -> bool {
    if !has_cpuid() {
        return false;
    }

    // SAFETY:
    // This processor supports `cpuid`.
    let ecx = unsafe { cpuid(1, 0) }.ecx;
    ecx & VMXE_SUPPORTED_BIT == VMXE_SUPPORTED_BIT
}

/// Initializes support for VMX on this processor.
///
/// # Safety
///
/// - This processor must support VMX.
/// - `vmxon_ptr` must point to a memory region under the control of this processor and must be at
///     least 4096 bytes.
/// - `vmxon_phys_addr` must be the physical base address of the frame to which `vmxon_ptr`'s page
///     is mapped.
pub unsafe fn initialize_processor(
    vmxon_ptr: *mut Vmxon,
    vmxon_phys_addr: u64,
) -> Result<ProcessorState, InitializeProcessorError> {
    debug_assert!(is_supported());

    // SAFETY:
    // VMX is supported.
    let feature_control = unsafe { read_msr(FEATURE_CONTROL_ADDRESS) };
    let required_bits = FEATURE_CONTROL_LOCKED | FEATURE_CONTROL_VMX_OUTSIDE_SMX;

    if (feature_control & FEATURE_CONTROL_LOCKED) == FEATURE_CONTROL_LOCKED
        && (feature_control & required_bits) != required_bits
    {
        return Err(InitializeProcessorError::FeatureDisabled);
    }

    if (feature_control & required_bits) != required_bits {
        // SAFETY:
        // - VMX is supported.
        // - FEATURE_CONTROL is not locked.
        unsafe { write_msr(FEATURE_CONTROL_ADDRESS, feature_control | required_bits) }
    }

    // Set the VMX bit in CR4.
    //
    // SAFETY:
    // VMX is supported.
    unsafe {
        core::arch::asm!(
            "mov {0}, cr4",
            "or {0}, 0x2000",
            "mov cr4, {0}",
            out(reg) _,
            options(nomem, nostack)
        )
    }

    // SAFETY:
    // VMX is supported.
    let cr0_fixed_0 = unsafe { read_msr(VMX_CR0_FIXED0_ADDRESS) };
    // SAFETY:
    // VMX is supported.
    let cr0_fixed_1 = unsafe { read_msr(VMX_CR0_FIXED1_ADDRESS) };

    //  Sets the CR0 bits required to be one and clears the bits required to be zero.
    //
    // SAFETY:
    // Manipulating CR0 should be fine.
    unsafe {
        core::arch::asm!(
            "mov {0}, cr0",
            "and {0}, {1}",
            "or {0}, {2}",
            "mov cr0, {0}",
            out(reg) _,
            in(reg) cr0_fixed_1 as usize,
            in(reg) cr0_fixed_0 as usize,
            options(nomem, nostack)
        )
    }

    // SAFETY:
    // VMX is supported.
    let cr4_fixed_0 = unsafe { read_msr(VMX_CR4_FIXED0_ADDRESS) };
    // SAFETY:
    // VMX is supported.
    let cr4_fixed_1 = unsafe { read_msr(VMX_CR4_FIXED1_ADDRESS) };

    //  Sets the CR4 bits required to be one and clears the bits required to be zero.
    //
    // SAFETY:
    // Manipulating CR4 should be fine.
    unsafe {
        core::arch::asm!(
            "mov {0}, cr4",
            "and {0}, {1}",
            "or {0}, {2}",
            "mov cr4, {0}",
            out(reg) _,
            in(reg) cr4_fixed_1 as usize,
            in(reg) cr4_fixed_0 as usize,
            options(nomem, nostack)
        )
    }

    // SAFETY:
    // VMX is supported.
    let vmx_basic = unsafe { read_msr(VMX_BASIC_ADDRESS) };
    let vmx_revision = vmx_basic as u32;

    // Initialize VMXON region.
    // SAFETY:
    // The memory that `vmx_on_region` was allocated correctly.
    unsafe { vmxon_ptr.write(Vmxon { vmx_revision }) }

    let success: u8;
    // SAFETY:
    // VMX is supported.
    unsafe {
        core::arch::asm!(
            "vmxon [{vmxon_ptr}]",
            "setnc {}",
            out(reg_byte) success,
            vmxon_ptr = in(reg) &vmxon_phys_addr,
        )
    }

    assert_eq!(success, 1);

    let processor_state = ProcessorState {
        vmxon_ptr,
        vmxon_phys_addr,
    };

    Ok(processor_state)
}

/// Various errors that can occur while initializing support for VMX.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum InitializeProcessorError {
    /// VMX operation was disabled.
    FeatureDisabled,
}

impl fmt::Display for InitializeProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FeatureDisabled => write!(f, "vmx operation was disabled"),
        }
    }
}

impl error::Error for InitializeProcessorError {}

/// Processor specific state related to VMX support.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ProcessorState {
    /// Pointer to the [`Vmxon`] region associated with this processor.
    vmxon_ptr: *mut Vmxon,
    /// Physical address of the [`Vmxon`] region associated with this processor.
    vmxon_phys_addr: u64,
}

/// Representation of the known elements of the VMXON region.
#[repr(C)]
pub struct Vmxon {
    /// The revision identifier used by the processor.
    vmx_revision: u32,
}
