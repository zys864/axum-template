use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn tracing_init() -> WorkerGuard {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn"));

    // file appender
    let file_appender = tracing_appender::rolling::hourly("logs", "app.log");
    let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_thread_names(true)
        .with_writer(non_blocking_appender);
    let console_layer = fmt::Layer::new()
        .with_writer(std::io::stdout)
        .with_thread_names(true)
        .pretty();
    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    guard
}
