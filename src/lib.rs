use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateUser {
    pub name: String,
}

pub fn app() -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users", get(get_users))
        .route("/users/{id}", get(get_user))
        .route("/users/{id}", put(update_user))
        .route("/users/{id}", delete(delete_user))
}

// POST /users
async fn create_user(Json(input): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: Uuid::new_v4(),
        name: input.name,
    };
    (StatusCode::CREATED, Json(user))
}

// GET /users
async fn get_users() -> Json<Vec<User>> {
    // Return an empty list as we are not storing anything
    Json(vec![])
}

// GET /users/{id}
async fn get_user(Path(id): Path<Uuid>) -> (StatusCode, Json<User>) {
    // Return a mock user, ignoring the id for now
    let user = User {
        id,
        name: "mock user".to_string(),
    };
    (StatusCode::OK, Json(user))
}

// PUT /users/{id}
async fn update_user(
    Path(id): Path<Uuid>,
    Json(input): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id,
        name: input.name,
    };
    (StatusCode::OK, Json(user))
}

// DELETE /users/{id}
async fn delete_user(Path(_id): Path<Uuid>) -> StatusCode {
    StatusCode::NO_CONTENT
}
