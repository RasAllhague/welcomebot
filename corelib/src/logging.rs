use fastrace::collector::{Config, ConsoleReporter};
use logforth::{
    append::{
        RollingFile,
        rolling_file::{self, RollingFileWriter, Rotation},
    },
    diagnostic,
    layout::JsonLayout,
    non_blocking::WorkerGuard,
};

/// Sets up observability for the bot.
///
/// This function configures logging and tracing for the bot, including rolling file logs
/// and console reporters.
///
/// # Returns
/// A `WorkerGuard` that ensures logs are flushed when the application exits.
pub fn setup_observability(service_name: &str) -> WorkerGuard {
    let rolling_writer = RollingFileWriter::builder()
        .rotation(Rotation::Daily)
        .filename_prefix(format!("{service_name}_app_log"))
        .build("logs")
        .unwrap();

    let (non_blocking, guard) = rolling_file::non_blocking(rolling_writer).finish();

    logforth::builder()
        .dispatch(|d| {
            d.filter(log::LevelFilter::Trace)
                .append(logforth::append::FastraceEvent::default())
        })
        .dispatch(|d| {
            d.diagnostic(diagnostic::FastraceDiagnostic::default())
                .append(logforth::append::Stderr::default())
        })
        .dispatch(|d| {
            d.filter(log::LevelFilter::Trace)
                .append(RollingFile::new(non_blocking).with_layout(JsonLayout::default()))
        })
        .apply();

    fastrace::set_reporter(ConsoleReporter, Config::default());

    guard
}
