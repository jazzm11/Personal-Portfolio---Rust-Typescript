use axum::{
    routing::get,
    Router,
};

use crate::routers::{
    health::health_check,
    projects::get_projects,
};

pub fn create_app() -> Router {
    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/projects", get(get_projects))
}