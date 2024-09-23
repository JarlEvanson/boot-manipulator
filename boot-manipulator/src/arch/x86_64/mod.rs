//! Definitions of `x86_64` architecture specific mechanisms.

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
    "jne 5f",
    "call {setup_virtualization}",
    "5:",
    "mov rax, qword ptr [rsp + 32]",
    "add rsp, 40",
    "ret",
    intercepted_func = sym crate::EXIT_BOOT_SERVICES_PTR,
    setup_virtualization = sym crate::setup_virtualization,
);
