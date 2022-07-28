use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to get configuration");
    let address = format!("localhost:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind to address");
    run(listener)?.await
}
