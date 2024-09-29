//! Build script for `boot-manipulator`.

fn main() {
    println!("cargo::rustc-link-arg=/subsystem:efi_runtime_driver");
}
