use portfolio_models::Project;
use sqlx::{Row, SqlitePool};

pub mod database;

// READ/GET Request
pub async fn get_projects(pool: &SqlitePool) -> Result<Vec<Project>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT title, description, technologies, github_url, demo_url FROM projects"
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
