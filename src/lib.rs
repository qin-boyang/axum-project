use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
}

pub fn app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users", get(get_users))
        .route("/users/{id}", get(get_user))
        .route("/users/{id}", put(update_user))
        .route("/users/{id}", delete(delete_user))
        .with_state(pool)
}

// POST /users
async fn create_user(
    State(pool): State<SqlitePool>,
    Json(input): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let user = User {
        id: Uuid::new_v4(),
        name: input.name,
    };

    sqlx::query("INSERT INTO users (id, name) VALUES (?, ?)")
        .bind(user.id)
        .bind(&user.name)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(user)))
}

// GET /users
async fn get_users(State(pool): State<SqlitePool>) -> Result<Json<Vec<User>>, StatusCode> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

// GET /users/{id}
async fn get_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok((StatusCode::OK, Json(user)))
}

// PUT /users/{id}
async fn update_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<Uuid>,
    Json(input): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    sqlx::query("UPDATE users SET name = ? WHERE id = ?")
        .bind(&input.name)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = User {
        id,
        name: input.name,
    };

    Ok((StatusCode::OK, Json(user)))
}

// DELETE /users/{id}
async fn delete_user(State(pool): State<SqlitePool>, Path(id): Path<Uuid>) -> StatusCode {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map(|result| {
            if result.rows_affected() > 0 {
                StatusCode::NO_CONTENT
            } else {
                StatusCode::NOT_FOUND
            }
        })
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}
