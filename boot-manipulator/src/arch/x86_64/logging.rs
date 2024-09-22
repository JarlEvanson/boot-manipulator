//! Architecture specific logging mechanisms.

use core::fmt::Write;

use crate::{
    arch::x86_64::serial::{
        DmaMode, DmaTriggerLevel, FifoControl, InterruptEnable, LineControl, SerialPort,
    },
    spinlock::Spinlock,
};

pub fn init_transition_logger(logger: &mut TransitionLogger) {
    let mut serial_port = logger.serial_port.lock();

    serial_port.set_interrupt_enable(InterruptEnable::new());
    serial_port.set_line_control(LineControl::new().set_dlab(true));
    serial_port.set_divisor(1);
    serial_port.set_line_control(LineControl::new());
    serial_port.set_fifo_control(
        FifoControl::new()
            .enable_fifo(true)
            .reset_receive_fifo(true)
            .reset_transmit_fifo(true)
            .dma_mode(DmaMode::MultiByte)
            .trigger_level(DmaTriggerLevel::Bytes14),
    );
}

pub struct TransitionLogger {
    serial_port: Spinlock<SerialPort>,
}

impl TransitionLogger {
    pub const fn new() -> Self {
        Self {
            serial_port: unsafe { Spinlock::new(SerialPort::new(0x3f8)) },
        }
    }
}

impl log::Log for TransitionLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let _ = writeln!(
            self.serial_port.lock(),
            "[{}]: {}",
            record.level(),
            record.args()
        );
    }

    fn flush(&self) {}
}
