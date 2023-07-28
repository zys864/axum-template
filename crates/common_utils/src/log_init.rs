use std::path::Path;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{
    fmt::{self, time::OffsetTime},
    EnvFilter,
};

pub fn log_init<T: AsRef<Path>>(log_dir: impl Into<Option<T>>) -> Option<WorkerGuard> {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn"));

    let local_time = OffsetTime::new(
        time::UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::macros::format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
        ),
    );

    let console_layer = fmt::Layer::new()
        .with_writer(std::io::stdout)
        .with_timer(local_time.clone())
        .pretty();

    let registry = tracing_subscriber::registry()
        .with(filter)
        .with(console_layer);

    let mut workguard = None;
    if let Some(dir) = log_dir.into() {
        // file appender
        let file_appender = tracing_appender::rolling::hourly(dir.as_ref(), "app.log");
        let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);

        workguard = Some(guard);
        let file_layer = fmt::layer()
            .with_ansi(false)
            .with_timer(local_time)
            .with_writer(non_blocking_appender);
        registry.with(file_layer);
    }

    workguard
}
