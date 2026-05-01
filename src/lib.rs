pub mod app; // Declare the app module

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, IntoActiveModel};
use serde::Deserialize;
use uuid::Uuid;

// Import the entity definitions from the new app module
use crate::app::{ActiveModel, Entity, Model};

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
}

// The shared state
#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

pub fn app_router(conn: DatabaseConnection) -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users", get(get_users))
        .route("/users/{id}", get(get_user))
        .route("/users/{id}", put(update_user))
        .route("/users/{id}", delete(delete_user))
        .with_state(AppState { conn })
}

// --- Handlers ---

// POST /users
async fn create_user(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<(StatusCode, Json<Model>), StatusCode> {
    let new_user = ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(input.name),
    };

    let user = new_user
        .insert(&state.conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(user)))
}

// GET /users
async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<Model>>, StatusCode> {
    let users = Entity::find()
        .all(&state.conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

// GET /users/{id}
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Model>, StatusCode> {
    let user = Entity::find_by_id(id)
        .one(&state.conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user))
}

// PUT /users/{id}
async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<CreateUser>,
) -> Result<Json<Model>, StatusCode> {
    let user = Entity::find_by_id(id)
        .one(&state.conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut user: ActiveModel = user.into_active_model();
    user.name = Set(input.name);

    let updated_user = user
        .update(&state.conn)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(updated_user))
}

// DELETE /users/{id}
async fn delete_user(State(state): State<AppState>, Path(id): Path<Uuid>) -> StatusCode {
    let result = Entity::delete_by_id(id)
        .exec(&state.conn)
        .await
        .unwrap();

    if result.rows_affected == 1 {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
