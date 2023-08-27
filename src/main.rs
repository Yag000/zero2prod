use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configurations::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
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
