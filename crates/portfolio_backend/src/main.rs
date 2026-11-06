mod app;
mod routers;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = app::create_app();

    let listener = TcpListener::bind("127.0.0.1:3000")
    .await
    .expect("Failed to bind adress");

    println!("Portfolio API is running at http://127.0.0.1:3000");

    // Start accepting HTTP requests
    axum::serve(listener, app)
        .await
        .expect("Server failed");
}