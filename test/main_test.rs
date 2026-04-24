use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;
use axum_project::{app, User};
use serde_json::json;
use uuid::Uuid;

#[tokio::test]
async fn test_crud_mock() {
    let app = app();

    // 1. Create a user
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("content-type", "application/json")
                .body(Body::from(json!({ "name": "Alice" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let user: User = serde_json::from_slice(&body).unwrap();
    assert_eq!(user.name, "Alice");

    let user_id = user.id;

    // 2. Get the user
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/users/{}", user_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let fetched_user: User = serde_json::from_slice(&body).unwrap();
    assert_eq!(fetched_user.id, user_id);
    assert_eq!(fetched_user.name, "mock user");

    // 3. Update the user
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(format!("/users/{}", user_id))
                .header("content-type", "application/json")
                .body(Body::from(json!({ "name": "Bob" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let updated_user: User = serde_json::from_slice(&body).unwrap();
    assert_eq!(updated_user.name, "Bob");

    // 4. Delete the user
    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/users/{}", Uuid::new_v4()))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
