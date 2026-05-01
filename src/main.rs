use axum_project::app;
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() {
    // Create an embedded SQLite database
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:database.db?mode=rwc")
        .await
        .unwrap();

    // Create the users table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .unwrap();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app(pool)).await.unwrap();
}
