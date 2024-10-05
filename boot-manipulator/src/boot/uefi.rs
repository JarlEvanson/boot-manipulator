//! UEFI specific code.

use core::{error, fmt};

use uefi::{boot::ScopedProtocol, proto::pi::mp::MpServices, Status};

use crate::{boot::BootOps, logging, spinlock::Spinlock, DEFAULT_LOG_LEVEL};

/// Globally available service to query processor information and control processors.
static mut MP_SERVICES: Option<ScopedProtocol<MpServices>> = None;

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

    let mp_services_ptr = core::ptr::addr_of_mut!(MP_SERVICES);
    let mp_services = 'mp_services: {
        let Ok(handle) = uefi::boot::get_handle_for_protocol::<MpServices>() else {
            log::debug!("failed to find mp services handle");
            break 'mp_services None;
        };

        let params = uefi::boot::OpenProtocolParams {
            handle,
            agent: uefi::boot::image_handle(),
            controller: None,
        };

        let attributes = uefi::boot::OpenProtocolAttributes::GetProtocol;

        // SAFETY:
        // We don't need exclusive access to this protocol.
        unsafe { uefi::boot::open_protocol::<MpServices>(params, attributes).ok() }
    };

    // SAFETY:
    // Currently, initialized with `None` and this is the only processor running.
    unsafe { *mp_services_ptr = mp_services }

    Status::SUCCESS
}

/// Implementation of [`BootOps`] for the UEFI environment.
pub struct Uefi;

impl BootOps for Uefi {
    type LoggingInitializationError = LoggingInitializationError;

    fn initialize_logger() -> Result<&'static dyn log::Log, Self::LoggingInitializationError> {
        Ok(&StdoutLogger)
    }

    fn processor_identity() -> usize {
        let mp_services = core::ptr::addr_of!(MP_SERVICES);

        // SAFETY:
        // `mp_services` is only used in a read-only manner.
        let mp_services = unsafe { &*mp_services };
        let Some(mp_services) = mp_services else {
            return 0;
        };

        mp_services.who_am_i().unwrap()
    }

    fn get_processor_count() -> usize {
        let mp_services = core::ptr::addr_of!(MP_SERVICES);

        // SAFETY:
        // `mp_services` is only used in a read-only manner.
        let mp_services = unsafe { &*mp_services };
        let Some(mp_services) = mp_services else {
            return 1;
        };

        mp_services.get_number_of_processors().unwrap().total
    }

    fn execute_on_all_processors(
        function: fn(*mut core::ffi::c_void),
        argument: *mut core::ffi::c_void,
    ) {
        /// Struct for helping with wrapped calls.
        struct WrapperStruct {
            /// The function to call.
            func: fn(*mut core::ffi::c_void),
            /// The argument to that function.
            arg: *mut core::ffi::c_void,
        }

        /// Wrapper function to create a compatible function ABI.
        extern "efiapi" fn wrapper_function(argument: *mut core::ffi::c_void) {
            // SAFETY:
            // `argument` is read-only, and is used in a blocking context, so the lifetime exceeds
            // that of this function.
            let wrapper = unsafe { &*argument.cast::<WrapperStruct>() };

            (wrapper.func)(wrapper.arg)
        }

        let mp_services = core::ptr::addr_of!(MP_SERVICES);

        // SAFETY:
        // `mp_services` is only used in a read-only manner.
        let mp_services = unsafe { &*mp_services };
        let Some(mp_services) = mp_services else {
            function(argument);
            return;
        };

        let mut wrapper_struct = WrapperStruct {
            func: function,
            arg: argument,
        };

        // SAFETY:
        // The created event is used only inside this function, which does not exit boot services.
        let mut event = unsafe {
            uefi::boot::create_event(
                uefi::boot::EventType::empty(),
                uefi::boot::Tpl::APPLICATION,
                None,
                None,
            )
            .unwrap()
        };

        let result = mp_services
            .startup_all_aps(
                false,
                wrapper_function,
                core::ptr::addr_of_mut!(wrapper_struct).cast::<core::ffi::c_void>(),
                // SAFETY:
                // The cloned event is waited on and then closed.
                unsafe { Some(event.unsafe_clone()) },
                None,
            )
            .map_err(|error| error.status());
        let wait_event = match result {
            Ok(()) => true,
            Err(error) if error == Status::NOT_STARTED => false,
            Err(error) => panic!("{error:?}"),
        };

        function(argument);
        if wait_event {
            uefi::boot::wait_for_event(core::slice::from_mut(&mut event)).unwrap();
        }

        uefi::boot::close_event(event).unwrap();
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
