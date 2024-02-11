//! Android logger that logs to logcat.
//!
//! ## Example
//!
//! ```
//! #[macro_use] extern crate log;
//! extern crate android_logger;
//!
//! fn main() {
//!     android_logger::init("MyApp").unwrap();
//!
//!     trace!("Initialized Rust");
//!     debug!("Address is {:p}", main as *const ());
//!     info!("Did you know? {} = {}", "1 + 1", 2);
//!     warn!("Don't log sensitive information!");
//!     error!("Nothing more to say");
//! }
//! ```
//!
//! ```{.bash}
//! $ logcat
//! 12-25 12:00:00.000  1234  1234 V MyApp: Initialized Rust
//! 12-25 12:00:00.000  1234  1234 D MyApp: Address is 0xdeadbeef
//! 12-25 12:00:00.000  1234  1234 I MyApp: Did you know? 1 + 1 = 2
//! 12-25 12:00:00.000  1234  1234 W MyApp: Don't log sensitive information!
//! 12-25 12:00:00.000  1234  1234 E MyApp: Nothing more to say

// TODO: Add support for auto-generated tags
use log::{Log, Level, Metadata, Record, SetLoggerError};
use android::log::{log, LogPriority};

/// `AndroidLogger` is the implementation of the logger.
///
/// It should not be used from Rust libraries which should only use the facade.
pub struct AndroidLogger {
    tag: String,
    format: Box<dyn Fn(&Record) -> String + Sync + Send>,
}

/// `LogBuilder` acts as builder for initializing the `AndroidLogger`. It can be
/// used to customize the log format.
///
/// ## Example
///
/// ```
/// #[macro_use] extern crate log;
/// extern crate android_logger;
///
/// use log::{Record, LevelFilter};
/// use android_logger::LogBuilder;
///
/// fn main() {
///     let format = |record: &Record| {
///         format!("{} - {}", record.target(), record.args())
///     };
///
///     let mut builder = LogBuilder::new("tag");
///     builder.format(format);
///
///     builder.init().unwrap();
///
/// 	warn!("Warning message");
///     error!("Error message");
/// }
/// ```
pub struct LogBuilder {
    tag: String,
    format: Box<dyn Fn(&Record) -> String + Sync + Send>,
}

/// Initializes the global logger with an `AndroidLogger`
///
/// This should be called early in the execution of a Rust program and the
/// global logger may only be initialized once. Future attempts will return an
/// error.
pub fn init<S: Into<String>>(tag: S) -> Result<(), SetLoggerError> {
    AndroidLogger::new(tag).init()
}

impl AndroidLogger {
    /// Initializes the logger with defaults
    pub fn new<S: Into<String>>(tag: S) -> AndroidLogger {
        LogBuilder::new(tag).build()
    }

    fn max_level(&self) -> log::LevelFilter {
        log::LevelFilter::max()
    }

    /// Initializes the global logger with `self`
    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_max_level(self.max_level());
        log::set_boxed_logger(Box::new(self))
    }
}

impl Log for AndroidLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !Log::enabled(self, record.metadata()) {
            return;
        }

        let format = String::from((self.format)(record));

        let prio = match record.level() {
            Level::Error => LogPriority::Error,
            Level::Warn => LogPriority::Warn,
            Level::Info => LogPriority::Info,
            Level::Debug => LogPriority::Debug,
            Level::Trace => LogPriority::Verbose,
        };
        log(prio, &self.tag, &format);
    }

    fn flush(&self) {
        todo!()
    }
}

impl LogBuilder {
    /// Initializes the builder with defaults
    pub fn new<S: Into<String>>(tag: S) -> LogBuilder {
        LogBuilder {
            tag: tag.into(),
            format: Box::new(|record: &Record| {
                format!("{:?}: {}", record.module_path(), record.args())
            }),
        }
    }

    /// Sets the format function for formatting the log output
    pub fn format<F: 'static>(&mut self, format: F) -> &mut Self
        where F: Fn(&Record) -> String + Sync + Send
    {
        self.format = Box::new(format);
        self
    }

    /// Builds an `AndroidLogger`
    pub fn build(self) -> AndroidLogger {
        AndroidLogger {
            tag: self.tag,
            format: self.format,
        }
    }
}