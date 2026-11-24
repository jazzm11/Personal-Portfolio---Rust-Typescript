// Imports axum functionality like GET & .
use axum::{
    routing::{get, put}, // add post if needed
    Router,
};

// Imports the database pool.
use sqlx::SqlitePool;

// Imports the functions from the router modules.
use crate::routers::{
    health::health_check,
    projects::{get_projects, create_project, update_project, delete_project},
};

// Creates the application routes and registers which functions should handle request.
pub fn create_app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/projects", get(get_projects).post(create_project)) // CREATE & READ
        .route("/api/projects/{id}", put(update_project).delete(delete_project)) // UPDATE & DELETE
        .with_state(pool)
}