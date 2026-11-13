
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

pub async fn create_database_pool() -> Result<SqlitePool, sqlx::Error> {
    // Connect to SQLite. Create portfolio.db if it doesn't exist.
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:portfolio.db?mode=rwc")
        .await?;

    // Create the projects table if it doesn't already exist.
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            technologies TEXT NOT NULL,
            github_url TEXT,
            demo_url TEXT
        )
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}
