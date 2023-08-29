use sqlx::PgPool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use zero2prod::{configurations::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Logger setup
    LogTracer::init().expect("Failed to set logger");
    // Info or above will be logged if the RUST_LOG environment variable is not set
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let foirmatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(foirmatting_layer);
    set_global_default(subscriber).expect("Failed to set subscriber");
    // We want to panic if we cannot read the configuration
    let configuration = get_configuration().expect("Failed to read configurations");
    let connection = PgPool::connect(&configuration.database.get_connnection_string())
        .await
        .expect("Failed to connect to Postgresf");
    // Bind the TCP listener socket address with the configuration port
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    run(listener, connection)?.await
}
