use axum::Json;
use serde::Serialize;

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