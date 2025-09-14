use axum::{
    Router,
    routing::{get, post},
};

mod database;
mod handlers;
mod models;
mod slug;

#[tokio::main]
async fn main() {
    // Initialize the sqlite database pool
    let pool = database::create_pool().await;

    // Set up the router app with routes
    let app = Router::new()
        .route("/shorten", post(handlers::create_slug))
        .route("/:slug", get(handlers::redirect_slug))
        .route("/stats/:slug", get(handlers::get_slug_stats))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
