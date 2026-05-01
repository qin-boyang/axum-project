use axum_project::{app, app_router}; // Use the router function, not the module
use sea_orm::{ConnectionTrait, Database, Schema};

#[tokio::main]
async fn main() {
    // Connect to the in-memory SQLite database
    let conn = Database::connect("sqlite::memory:").await.unwrap();

    // Set up the schema
    let backend = conn.get_database_backend();
    let schema = Schema::new(backend);
    let create_table_stmt = schema.create_table_from_entity(app::Entity); // Now references the module correctly
    conn.execute(backend.build(&create_table_stmt)).await.unwrap();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app_router(conn)).await.unwrap();
}
