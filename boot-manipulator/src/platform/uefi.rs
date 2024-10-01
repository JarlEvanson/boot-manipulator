//! UEFI specific code.

use core::{error, fmt};

use uefi::Status;

use crate::{logging, platform::PlatformOps, spinlock::Spinlock, DEFAULT_LOG_LEVEL};

#[uefi::entry]
fn entry() -> Status {
    match logging::initialize_logging(DEFAULT_LOG_LEVEL) {
        Ok(()) => {}
        Err(error) => {
            use core::fmt::Write;

            let _ = uefi::system::with_stdout(|stdout| {
                writeln!(stdout, "[ERROR]: logging failed to be initialized: {error}")
            });

            return Status::LOAD_ERROR;
        }
    }

    match crate::main() {
        Ok(()) => {}
        Err(error) => {
            log::error!("{error}");
            return Status::LOAD_ERROR;
        }
    }

    Status::SUCCESS
}

/// Implementation of [`PlatformOps`] for the UEFI environment.
pub struct Uefi;

impl PlatformOps for Uefi {
    type LoggingInitializationError = LoggingInitializationError;

    fn initialize_logger() -> Result<&'static dyn log::Log, Self::LoggingInitializationError> {
        Ok(&StdoutLogger)
    }
}

/// Zero sized struct to represent logging to UEFI stdout.
pub struct StdoutLogger;

impl log::Log for StdoutLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        use core::fmt::Write;

        /// Lock to prevent racing writes to UEFI stdout.
        static STDOUT_LOCK: Spinlock<()> = Spinlock::new(());

        let lock = STDOUT_LOCK.lock();

        let _ = uefi::system::with_stdout(|stdout| {
            writeln!(stdout, "[{}]: {}", record.level(), record.args())
        });

        drop(lock);
    }

    fn flush(&self) {}
}

/// Various errors that can occur while initializing [`StdoutLogger`].
pub enum LoggingInitializationError {}

impl fmt::Debug for LoggingInitializationError {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        unreachable!()
    }
}

impl fmt::Display for LoggingInitializationError {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        unreachable!()
    }
}

impl error::Error for LoggingInitializationError {}
