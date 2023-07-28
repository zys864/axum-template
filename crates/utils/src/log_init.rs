use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{self, time::OffsetTime},
    prelude::__tracing_subscriber_SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

pub fn log_init(log_dir: Option<impl AsRef<str>>) -> Option<WorkerGuard> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn"));
    let registry = tracing_subscriber::registry()
        .with(filter);
    let local_time = OffsetTime::new(
        time::UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::macros::format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
        ),
    );
    let mut workguard = None;
    if let Some(dir) = log_dir {
        // file appender
        let file_appender = tracing_appender::rolling::hourly(dir.as_ref(), "app.log");
        let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);

        workguard = Some(guard);
        let file_layer = fmt::layer()
            .with_ansi(false)
            .with_timer(local_time.clone())
            .with_writer(non_blocking_appender);
        registry
            .with(file_layer)
            .init();
    }


    let console_layer = fmt::Layer::new()
        .with_writer(std::io::stdout)
        .with_timer(local_time)
        .pretty();

    registry.with(console_layer)
        .init();

    workguard
}