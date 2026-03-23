use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::{json, Value};
use tower::ServiceExt;

use crate::helpers::create_test_app;

#[tokio::test]
async fn test_request_otp_success() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/otp/request")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({ "phone": "+5573999001234" }).to_string(),
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
    assert_eq!(json["message"], "OTP sent");
    assert_eq!(json["expires_in"], 300);
}

#[tokio::test]
async fn test_request_otp_invalid_phone() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/otp/request")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({ "phone": "invalid" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_verify_otp_creates_user() {
    let (app, state) = create_test_app_with_state().await;

    // First, request OTP to store it in Redis
    let phone = format!("+55739990{:05}", rand::random::<u32>() % 100_000);
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/otp/request")
                .header("Content-Type", "application/json")
                .body(Body::from(json!({ "phone": phone }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Get the OTP from Redis
    let mut redis_conn = state
        .redis
        .get_multiplexed_async_connection()
        .await
        .unwrap();
    let code: String = redis::AsyncCommands::get(&mut redis_conn, format!("otp:{phone}"))
        .await
        .unwrap();

    // Verify OTP
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/auth/otp/verify")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({ "phone": phone, "code": code }).to_string(),
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
    assert!(json["token"].is_string());
    assert_eq!(json["is_new_user"], true);
    assert_eq!(json["user"]["phone"], phone);
}

use crate::helpers::create_test_app_with_state;
