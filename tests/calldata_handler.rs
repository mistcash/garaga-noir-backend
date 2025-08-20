use garaga_noir_backend::app;
use reqwest::Client;
use serde_json::json;
use std::net::SocketAddr;
use tokio::task;

#[tokio::test]
async fn test_calldata_handler_success() {
    // Start the server in a background task
    let port = 4000; // Use a test port
    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();
    let app = app();
    task::spawn(async move {
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });
    tokio::time::sleep(std::time::Duration::from_millis(500)).await; // Wait for server

    let client = Client::new();
    let payload = json!({
        "proof_bytes": [],
        "public_inputs_bytes": [],
        "vk_bytes": [],
        "flavor_num": 0
    });
    let res = client
        .post(&format!("http://{}/calldata", addr))
        .json(&payload)
        .send()
        .await
        .unwrap();
    let body: serde_json::Value = res.json().await.unwrap();
    println!("Response body: {:?}", body);
    assert!(body.get("calldata").is_some() || body.get("error").is_some());
    assert!(false);
}
