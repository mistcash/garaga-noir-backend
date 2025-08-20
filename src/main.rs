use axum::{routing::post, Json, Router};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
mod garaga;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CalldataRequest {
    proof_bytes: Vec<u8>,
    public_inputs_bytes: Vec<u8>,
    vk_bytes: Vec<u8>,
    flavor_num: usize,
}

pub async fn calldata_handler(Json(payload): Json<CalldataRequest>) -> Json<serde_json::Value> {
    let result = crate::garaga::get_zk_honk_calldata(
        &payload.proof_bytes,
        &payload.public_inputs_bytes,
        &payload.vk_bytes,
        payload.flavor_num,
    );

    match result {
        Ok(calldata) => Json(serde_json::json!({ "calldata": calldata })),
        Err(e) => Json(serde_json::json!({ "error": e })),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing

    info!(
        "Starting Cairo Compilation API v{}",
        env!("CARGO_PKG_VERSION")
    );

    // Build the application with middleware
    let app = Router::new()
        .route("/calldata", post(calldata_handler))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        );

    // Start the server
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()?;

    let addr = format!("0.0.0.0:{}", port);
    println!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
