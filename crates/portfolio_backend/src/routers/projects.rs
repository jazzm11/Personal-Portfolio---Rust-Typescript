use axum::{
    Json,
    extract::{State, Path},
    http::StatusCode,
};

use portfolio_models::Project;
use sqlx::SqlitePool;

pub async fn get_projects(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Project>>, StatusCode> {
    match portfolio_repository::get_projects(&pool).await {
        Ok(projects) => Ok(Json(projects)),
        Err(error) => {
            eprintln!("Failed to retrieve projects: {}", error);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_project(
    State(pool): State<SqlitePool>,
    Json(project): Json<Project>,
) -> Result<StatusCode, StatusCode> {
    match portfolio_repository::create_project(&pool, &project).await {
        Ok(()) => Ok(StatusCode::CREATED),
        Err(error) => {
            eprintln!("Failed to create project: {}", error);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_project(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
    Json(project): Json<Project>,
) -> Result<StatusCode, StatusCode> {
    match portfolio_repository::update_project(&pool, id, &project).await {
        Ok(true) => Ok(StatusCode::OK),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(error) => {
            eprintln!("Failed to update project: {}", error);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_project(
    Path(id): Path<i64>,
    State(pool): State<SqlitePool>,
) -> Result<StatusCode, StatusCode> {
    match portfolio_repository::delete_project(&pool, id).await {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(error) => {
            eprintln!("Failed to delete project: {}", error);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}