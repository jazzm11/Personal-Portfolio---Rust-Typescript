use axum::{
    routing::{get, put}, // add post if needed
    Router,
};
use sqlx::SqlitePool;

use crate::routers::{
    health::health_check,
    projects::{get_projects, create_project, update_project, delete_project},
};

pub fn create_app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/projects", get(get_projects).post(create_project))
        .route("/api/projects/{id}", put(update_project).delete(delete_project))
        .with_state(pool)
}