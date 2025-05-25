use tracing::level_filters::LevelFilter;
use tracing_subscriber::{prelude::*, EnvFilter};

pub fn init_standard_logger() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();
}
