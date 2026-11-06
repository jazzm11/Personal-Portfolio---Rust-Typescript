use axum::{
    routing::get,
    Router,
};

use crate::routers::health::health_check;

pub fn create_app() -> Router {
    Router::new().route("/api/health", get(health_check))
}