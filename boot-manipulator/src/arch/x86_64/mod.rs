//! Definitions of `x86_64` architecture specific mechanisms.

use core::mem::MaybeUninit;

pub mod logging;
mod registers;
mod serial;
pub mod virtualization;

extern "efiapi" {
    #[link_name = "exit_boot_services_handler"]
    pub fn exit_boot_services_handler(
        image_handle: *mut core::ffi::c_void,
        map_key: usize,
    ) -> uefi::Status;
}

core::arch::global_asm!(
    ".global exit_boot_services_handler",
    "exit_boot_services_handler:",
    "sub rsp, 40",
    "call qword ptr [rip + {intercepted_func}]",
    "mov qword ptr [rsp + 32], rax",
    "cmp rax, 0",
    "je 5f",
    "4:",
    "mov rax, qword ptr [rsp + 32]", // exit failed or in virtual machine.
    "add rsp, 40",
    "ret",
    "5:", // Exit succeeded, start initializing VMCS
    "mov [{uefi_registers}], rax",
    "mov [{uefi_registers} + 8], rbx",
    "mov [{uefi_registers} + 16], rcx",
    "mov [{uefi_registers} + 24], rdx",
    "mov [{uefi_registers} + 32], rdi",
    "mov [{uefi_registers} + 40], rsi",
    "mov [{uefi_registers} + 48], rsp",
    "mov [{uefi_registers} + 56], rbp",
    "mov [{uefi_registers} + 64], r8",
    "mov [{uefi_registers} + 72], r9",
    "mov [{uefi_registers} + 80], r10",
    "mov [{uefi_registers} + 88], r11",
    "mov [{uefi_registers} + 96], r12",
    "mov [{uefi_registers} + 104], r13",
    "mov [{uefi_registers} + 112], r14",
    "mov [{uefi_registers} + 120], r15",
    "lea rax, [rip]",
    "mov [{uefi_registers} + 128], rax",
    "mov rax, cr0",
    "mov [{uefi_registers} + 136], rax",
    "mov rax, cr2",
    "mov [{uefi_registers} + 144], rax",
    "mov rax, cr3",
    "mov [{uefi_registers} + 152], rax",
    "mov rax, cr4",
    "mov [{uefi_registers} + 160], rax",
    "mov ax, cs",
    "mov [{uefi_registers} + 168], ax",
    "mov ax, ss",
    "mov [{uefi_registers} + 170], ax",
    "mov ax, ds",
    "mov [{uefi_registers} + 172], ax",
    "mov ax, es",
    "mov [{uefi_registers} + 174], ax",
    "mov ax, fs",
    "mov [{uefi_registers} + 176], ax",
    "mov ax, gs",
    "mov [{uefi_registers} + 178], ax",
    "sldt [{uefi_registers} + 180]",
    "str [{uefi_registers} + 182]",
    "pushfq",
    "pop rax",
    "mov [{uefi_registers} + 184], rax",
    "call {setup_virtualization}",
    intercepted_func = sym crate::EXIT_BOOT_SERVICES_PTR,
    setup_virtualization = sym crate::setup_virtualization,
    uefi_registers = sym REGISTERS
);

pub static mut REGISTERS: MaybeUninit<UefiRegisters> = MaybeUninit::zeroed();

#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, Default, PartialEq, Eq)]
pub struct UefiRegisters {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rdi: u64,
    rsi: u64,
    rsp: u64,
    rbp: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    rip: u64,
    cr0: u64,
    cr2: u64,
    cr3: u64,
    cr4: u64,
    cs: u16,
    ss: u16,
    ds: u16,
    es: u16,
    fs: u16,
    gs: u16,
    ldtr: u16,
    tr: u16,
    rflags: u64,
}
