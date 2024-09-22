//! Build script ensuring `boot-manipulator` is built as an UEFI runtime driver.

fn main() {
    println!("cargo::rustc-link-arg=/subsystem:efi_runtime_driver");
}
