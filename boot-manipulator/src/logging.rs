//! Logging implementation for `boot-manipulator`.

use log::{LevelFilter, Log};

use crate::{
    platform::{Platform, PlatformOps},
    spinlock::Spinlock,
};

/// The currently active [`Log`] implementation.
static ACTIVE_LOGGER: Spinlock<&dyn Log> = Spinlock::new(&Placeholder);

/// Initializes the logging system for `boot-manipulator`.
///
/// # Errors
///
/// If [`Platform::initialize_logger()`] fails, this function returns that error.
///
/// # Panics
///
/// If [`initialize_logging()`] has already been called, this function will panic.
pub fn initialize_logging(
    filter: LevelFilter,
) -> Result<(), <Platform as PlatformOps>::LoggingInitializationError> {
    *ACTIVE_LOGGER.lock() = Platform::initialize_logger()?;

    log::set_logger(&Logger).expect("initialize_logging was called twice");
    log::set_max_level(filter);

    Ok(())
}

/// Zero sizedstruct to implement [`Log`] switching capabilities.
struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        ACTIVE_LOGGER.lock().enabled(metadata)
    }

    fn log(&self, record: &log::Record) {
        ACTIVE_LOGGER.lock().log(record)
    }

    fn flush(&self) {
        ACTIVE_LOGGER.lock().flush()
    }
}

/// Zeroed sized placeholder for logging startup.
struct Placeholder;

impl Log for Placeholder {
    fn enabled(&self, _: &log::Metadata) -> bool {
        false
    }

    fn log(&self, _: &log::Record) {}

    fn flush(&self) {}
}
