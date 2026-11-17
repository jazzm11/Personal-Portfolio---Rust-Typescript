use portfolio_models::Project;
use sqlx::{Row, SqlitePool};

pub mod database;

// READ/GET Request
pub async fn get_projects(pool: &SqlitePool) -> Result<Vec<Project>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT id, title, description, technologies, github_url, demo_url FROM projects"
    )
    .fetch_all(pool)
    .await?;

    let mut projects = Vec::new();

    for row in rows {
        let technologies_json: String = row.try_get("technologies")?;

        let technologies: Vec<String> =
            serde_json::from_str(&technologies_json)
                .map_err(|error| sqlx::Error::Decode(Box::new(error)))?;

        projects.push(Project {
            id: Some(row.try_get("id")?),
            title: row.try_get("title")?,
            description: row.try_get("description")?,
            technologies,
            github_url: row.try_get("github_url")?,
            demo_url: row.try_get("demo_url")?,
        });
    }

    Ok(projects)
}

// CREATE/POST Request
pub async fn create_project(
    pool: &SqlitePool,
    project: &Project,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Convert the technologies Vec into JSON text for SQLite.
    let technologies = serde_json::to_string(&project.technologies)?;

    sqlx::query(
        r#"
        INSERT INTO projects (title, description, technologies, github_url, demo_url)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(&project.title)
    .bind(&project.description)
    .bind(&technologies)
    .bind(project.github_url.as_deref())
    .bind(project.demo_url.as_deref())
    .execute(pool)
    .await?;

    Ok(())
}

// UPDATE/PUT Request
pub async fn update_project(
    pool: &SqlitePool,
    id: i64,
    project: &Project,
) -> Result<bool, sqlx::Error> {
    let technologies = serde_json::to_string(&project.technologies)
        .map_err(|error| sqlx::Error::Encode(Box::new(error)))?;

    let result = sqlx::query(
        r#"
        UPDATE projects
        SET title = ?, description = ?, technologies = ?, github_url = ?, demo_url = ?
        WHERE id = ?
        "#,
    )
    .bind(&project.title)
    .bind(&project.description)
    .bind(&technologies)
    .bind(project.github_url.as_deref())
    .bind(project.demo_url.as_deref())
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

// DELETE REQUEST 
pub async fn delete_project(
    pool: &SqlitePool,
    id: i64,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM projects WHERE id = ?"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}