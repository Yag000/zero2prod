use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_log::LogTracer;
use zero2prod::{configurations::get_configuration, startup::run, telemetry::get_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Logger setup
    LogTracer::init().expect("Failed to set logger");

    // Info or above will be logged if the RUST_LOG environment variable is not set
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    set_global_default(subscriber).expect("Failed to set subscriber");

    // We want to panic if we cannot read the configuration
    let configuration = get_configuration().expect("Failed to read configurations");
    let connection = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(
            configuration
                .database
                .get_connnection_string()
                .expose_secret(),
        )
        .expect("Failed to connect to Postgres");

    // Bind the TCP listener socket address with the configuration port
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    run(listener, connection)?.await
}
