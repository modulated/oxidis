// Spawn background instance of application on random port and return address
fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("localhost:0")
        .expect("Failed to bind random port.");
    let port = listener.local_addr().unwrap().port();
    let server = oxidis::run(listener).expect("Failed to bind address.");
    let _ = tokio::spawn(server);
    format!("http://localhost:{}", port)
}

#[actix_rt::test]
async fn health_check_test() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribe_returns_200_for_valid_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=Cuck%20Lord&email=fuck%40off.com";

    let response = client
        .post(format!("{}/subscriptions", app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn subscribe_returns_400_when_missing_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=Alpha", "missing email"),
        ("email=fuck%40off.com", "missing name"),
        ("", "missing name and email")
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(400, response.status().as_u16(), "The API did not fail with 400 Bad Request when payload was {}.", error_message);
    }


}