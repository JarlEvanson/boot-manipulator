//! Build script for `boot-manipulator`.

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "uefi" {
        println!("cargo::rustc-link-arg=/subsystem:efi_runtime_driver");
    }
}
