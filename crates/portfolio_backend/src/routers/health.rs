use axum::Json; // Returns data as a JSON format.
use serde::Serialize; // Used to convert RUST structs into formats like JSON.

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub message: &'static str,
}

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        message: "Portfolio API is running",
    })
}