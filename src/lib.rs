use axum::{routing::post, Json, Router};
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
mod garaga;

#[derive(Deserialize)]
pub struct CalldataRequest {
    pub proof_bytes: Vec<u8>,
    pub public_inputs_bytes: Vec<u8>,
    pub vk_bytes: Vec<u8>,
    pub flavor_num: usize,
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

pub fn app() -> Router {
    Router::new()
        .route("/calldata", post(calldata_handler))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        )
}
