use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to get configuration");
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to db");
    let address = format!("localhost:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind to address");
    run(listener, db_pool)?.await
}
