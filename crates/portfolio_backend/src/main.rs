mod app;
mod routers;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Initialize the SQLite database
    let pool = portfolio_repository::database::create_database_pool()
        .await
        .expect("Failed to connect to SQLite database");

    println!("Connected to SQLite database!");

    let app = app::create_app();

    let listener = TcpListener::bind("127.0.0.1:3000")
    .await
    .expect("Failed to bind adress");

    println!("Portfolio API is running at http://127.0.0.1:3000");

    // Start accepting HTTP requests
    axum::serve(listener, app)
        .await
        .expect("Server failed");

    // Keep the pool alive while the server runs
    // drop(pool); OPTIONAL
}