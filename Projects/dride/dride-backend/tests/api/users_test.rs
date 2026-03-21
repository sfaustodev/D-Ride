use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::{json, Value};
use tower::ServiceExt;

use crate::helpers::{create_test_app_with_state, create_test_user_token};

#[tokio::test]
async fn test_get_me_unauthorized() {
    let (app, _) = create_test_app_with_state().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/users/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_me_success() {
    let (app, state) = create_test_app_with_state().await;
    let token = create_test_user_token(&state).await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/users/me")
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert!(json["id"].is_string());
    assert!(json["phone"].is_string());
}

#[tokio::test]
async fn test_update_me() {
    let (app, state) = create_test_app_with_state().await;
    let token = create_test_user_token(&state).await;

    let response = app
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri("/v1/users/me")
                .header("Authorization", format!("Bearer {token}"))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({ "name": "João Silva", "role": "driver" }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["name"], "João Silva");
    assert_eq!(json["role"], "driver");
}

#[tokio::test]
async fn test_delete_me() {
    let (app, state) = create_test_app_with_state().await;
    let token = create_test_user_token(&state).await;

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/v1/users/me")
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
