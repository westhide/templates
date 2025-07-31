pub use tracing::*;

pub fn init_tracing_subscriber_log() {
    use tracing_subscriber::{EnvFilter, fmt::format::FmtSpan};
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .init()
}

#[cfg(feature = "tracing-browser")]
pub fn init_tracing_browser_subscriber_log() {
    tracing_browser_subscriber::configure_as_global_default();
}
