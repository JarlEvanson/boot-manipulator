//! Logging solution.

use core::{
    fmt::Write,
    sync::atomic::{AtomicBool, Ordering},
};

use log::{LevelFilter, Log};

/// Whether UEFI boot services is still active.
static IN_BOOT_SERVICES: AtomicBool = AtomicBool::new(true);

/// Initialize the logging system for `boot-manipulator`.
///
/// # Panics
/// If [`initialize_logging`] has already been called, this function panics.
pub fn initialize_logging(filter: LevelFilter) {
    log::set_logger(&Logger).expect("initialize_logging has already been called");
    log::set_max_level(filter);
}

/// Zeroed sized marker for logger.
struct Logger;

impl Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if IN_BOOT_SERVICES.load(Ordering::Relaxed) {
            let _ = uefi::system::with_stdout(|stdout| {
                writeln!(
                    stdout,
                    "[Hypervisor {:>5}]: {}",
                    record.level(),
                    record.args()
                )
            });
        }
    }

    fn flush(&self) {}
}
