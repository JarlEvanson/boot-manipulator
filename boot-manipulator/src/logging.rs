//! Logging for `boot-manipulator`.

use core::{
    fmt::Write,
    sync::atomic::{AtomicU8, Ordering},
};

use crate::{
    arch::logging::{init_transition_logger, TransitionLogger},
    spinlock::Spinlock,
};

const BOOT_SERVICES: u8 = 0;
const INITIALIZING: u8 = 1;
const RUNNING: u8 = 2;

static PROGRAM_STATE: AtomicU8 = AtomicU8::new(BOOT_SERVICES);

static TRANSITION_LOGGER: Spinlock<TransitionLogger> = Spinlock::new(TransitionLogger::new());

pub fn initialize_logging(level_filter: log::LevelFilter) {
    log::set_logger(&Logger).expect("initialize_logging shouldn't be called twice");
    log::set_max_level(level_filter);
}

pub fn transition_boot_services() {
    PROGRAM_STATE.store(INITIALIZING, Ordering::Relaxed);
    init_transition_logger(&mut TRANSITION_LOGGER.lock());
}

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        match PROGRAM_STATE.load(Ordering::Relaxed) {
            BOOT_SERVICES => uefi::system::with_stdout(|stdout| {
                let _ = writeln!(stdout, "[{}]: {}", record.level(), record.args());
            }),
            INITIALIZING => TRANSITION_LOGGER.lock().log(record),
            state => unreachable!("Unreachable program state: {state}"),
        };
    }

    fn flush(&self) {}
}
