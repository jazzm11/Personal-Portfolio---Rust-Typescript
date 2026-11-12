use axum::Json;
use portfolio_models::Project;
use portfolio_repository::get_projects as find_projects;

pub async fn get_projects() -> Json<Vec<Project>> {
    let projects = find_projects();

    Json(projects)
}