//! Definitions of `x86_64` virtualization mechanisms.

use core::{
    arch::asm,
    ptr,
    sync::atomic::{AtomicPtr, Ordering},
};

use uefi::boot;

use crate::arch::x86_64::registers::control::{Cr0, Cr0Display, Cr4, Cr4Display};

const CR4_VMXE_BIT: u8 = 5;
const CR4_VMXE: u64 = 1 << CR4_VMXE_BIT;

const FEATURE_CONTROL_MSR: u32 = 0x3a;
const FEATURE_CONTROL_MSR_LOCKED: u64 = 1;
const FEATURE_CONTROL_MSR_VMX_OUTSIDE_SMX: u64 = 1 << 2;

const VMX_CR0_FIXED0: u32 = 0x486;
const VMX_CR0_FIXED1: u32 = 0x487;

const VMX_CR4_FIXED0: u32 = 0x488;
const VMX_CR4_FIXED1: u32 = 0x489;

const VMX_REVISION: u32 = 0x480;

static VMXON_REGION: AtomicPtr<u8> = AtomicPtr::new(ptr::null_mut());
static VMCS_REGION: AtomicPtr<u8> = AtomicPtr::new(ptr::null_mut());

pub fn is_supported() -> bool {
    let ecx = unsafe { core::arch::x86_64::__cpuid(1).ecx };
    (ecx as u64 & CR4_VMXE) == CR4_VMXE
}

pub fn allocate_basic_memory() {
    let vmxon_ptr = boot::allocate_pages(
        boot::AllocateType::AnyPages,
        boot::MemoryType::LOADER_DATA,
        1,
    )
    .unwrap();

    VMXON_REGION.store(vmxon_ptr.as_ptr(), Ordering::Relaxed);

    let vmcs_ptr = boot::allocate_pages(
        boot::AllocateType::AnyPages,
        boot::MemoryType::LOADER_DATA,
        1,
    )
    .unwrap();

    VMCS_REGION.store(vmcs_ptr.as_ptr(), Ordering::Relaxed);
}

pub fn enable_support() {
    assert!(is_supported());

    let feature_control = readmsr(FEATURE_CONTROL_MSR);
    let required_bits = FEATURE_CONTROL_MSR_LOCKED | FEATURE_CONTROL_MSR_VMX_OUTSIDE_SMX;
    log::trace!("VMX Feature Control: {feature_control:016X}");
    log::trace!("VMX Feature Control Required: {required_bits:016X}");

    assert!(
        (feature_control & FEATURE_CONTROL_MSR_LOCKED) != FEATURE_CONTROL_MSR_LOCKED
            || ((feature_control & required_bits) == required_bits)
    );

    if (feature_control & required_bits) != required_bits {
        writemsr(FEATURE_CONTROL_MSR, feature_control | required_bits);
        log::trace!("Enabled feature control bits");
    }

    unsafe {
        asm!(
            "mov {0}, cr4",
            "or {0}, 0x2000",
            "mov cr4, {0}",
            out(reg) _,
            options(nomem, nostack)
        );
    }
    log::trace!("Enabled CR4 VMX bit");

    log::trace!(
        "CR0 VMX Fixed 0: {}\nCR0 VMX Fixed 1: {}\nCR0: {}",
        Cr0Display(readmsr(VMX_CR0_FIXED0)),
        Cr0Display(!readmsr(VMX_CR0_FIXED1)),
        Cr0::get()
    );

    log::trace!(
        "CR4 VMX Fixed 0: {}\nCR4 VMX Fixed 1: {}\nCR4: {}",
        Cr4Display(readmsr(VMX_CR4_FIXED0)),
        Cr4Display(!readmsr(VMX_CR4_FIXED1)),
        Cr4::get(),
    );

    let vmx_revision = readmsr(VMX_REVISION) as u32;
    log::trace!("VMX basic: {:016X}", readmsr(VMX_REVISION));

    let vmxon_ptr = VMXON_REGION.load(Ordering::Relaxed);
    assert!(!vmxon_ptr.is_null());
    log::trace!("VMXON ptr: {vmxon_ptr:p}");
    unsafe { core::ptr::write_bytes::<u8>(vmxon_ptr, 0, 4096) }
    unsafe { vmxon_ptr.cast::<u32>().write(vmx_revision) }

    let success: u8;
    unsafe {
        asm!(
            "vmxon [{}]",
            "seta {}",
            in(reg) &vmxon_ptr,
            lateout(reg_byte) success,
        )
    }
    assert_eq!(success, 1);
}

pub fn setup_virtual_machine_state() {
    let vmcs_ptr = VMCS_REGION.load(Ordering::Relaxed);

    unsafe { core::ptr::write_bytes::<u8>(vmcs_ptr, 0, 4096) }
    unsafe { vmcs_ptr.cast::<u32>().write(readmsr(VMX_REVISION) as u32) }
    log::trace!("VMCS ptr: {vmcs_ptr:p}");

    let valid_vmcs_ptr: u8;
    let other_error: u8;
    unsafe {
        asm!(
            "vmptrld [{}]",
            "setnc {}",
            "setnz {}",
            in(reg) &vmcs_ptr,
            lateout(reg_byte) valid_vmcs_ptr,
            lateout(reg_byte) other_error,
        )
    }

    assert!(valid_vmcs_ptr == 1);
    assert!(other_error == 1);
}

fn readmsr(msr: u32) -> u64 {
    let rax: u64;
    let rdx: u64;
    unsafe { asm!("rdmsr", in("ecx") msr, lateout("eax") rax, lateout("edx") rdx) }

    (rax as u64) | ((rdx as u64) << 32)
}

fn writemsr(msr: u32, value: u64) {
    unsafe {
        asm!("wrmsr", in("ecx") msr, in("eax") value as u32, in("edx") ((value >> 32) as u32))
    }
}
