use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("localhost:0").expect("Failed to bind to address");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://localhost:{port}")
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to send request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
