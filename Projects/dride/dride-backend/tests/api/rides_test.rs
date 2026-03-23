use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::{json, Value};
use tower::ServiceExt;

use crate::helpers::{create_test_app_with_state, create_test_driver, create_test_user_token};

#[tokio::test]
async fn test_estimate() {
    let (app, state) = create_test_app_with_state().await;
    let token = create_test_user_token(&state).await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/rides/estimate")
                .header("Authorization", format!("Bearer {token}"))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "pickup_lat": -16.4489,
                        "pickup_lng": -39.0648,
                        "dropoff_lat": -16.4370,
                        "dropoff_lng": -39.0580
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["distance_km"].as_f64().unwrap() > 0.0);
    assert!(json["duration_min"].as_i64().unwrap() > 0);
    assert!(json["fare_lamports"].as_i64().unwrap() > 0);
    assert!(json["fare_brl"].as_f64().unwrap() > 0.0);
    assert_eq!(json["protocol_fee_bps"], 1000);
}

#[tokio::test]
async fn test_create_ride() {
    let (app, state) = create_test_app_with_state().await;
    let token = create_test_user_token(&state).await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/rides")
                .header("Authorization", format!("Bearer {token}"))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "pickup_lat": -16.4489,
                        "pickup_lng": -39.0648,
                        "pickup_address": "Av. do Descobrimento, 123",
                        "dropoff_lat": -16.4370,
                        "dropoff_lng": -39.0580,
                        "dropoff_address": "Praia de Taperapuã"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["ride"]["id"].is_string());
    assert_eq!(json["ride"]["status"], "requested");
    assert!(json["ride"]["fare_lamports"].as_i64().unwrap() > 0);
    assert!(json["escrow_pubkey"].is_string());
}

#[tokio::test]
async fn test_full_ride_lifecycle() {
    let (app, state) = create_test_app_with_state().await;
    let passenger_token = create_test_user_token(&state).await;
    let (driver_token, _driver_id) = create_test_driver(&state).await;

    // 1. Create ride
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/rides")
                .header("Authorization", format!("Bearer {passenger_token}"))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "pickup_lat": -16.4489,
                        "pickup_lng": -39.0648,
                        "pickup_address": "Av. do Descobrimento, 123",
                        "dropoff_lat": -16.4370,
                        "dropoff_lng": -39.0580,
                        "dropoff_address": "Praia de Taperapuã"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let create_json: Value = serde_json::from_slice(&body).unwrap();
    let ride_id = create_json["ride"]["id"].as_str().unwrap();

    // 2. Accept ride (driver)
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&format!("/v1/rides/{ride_id}/accept"))
                .header("Authorization", format!("Bearer {driver_token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let accept_json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(accept_json["ride"]["status"], "accepted");
    assert!(accept_json["passenger"]["name"].is_string());

    // 3. Start ride (driver)
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&format!("/v1/rides/{ride_id}/start"))
                .header("Authorization", format!("Bearer {driver_token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let start_json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(start_json["status"], "active");
    assert!(start_json["started_at"].is_string());

    // 4. Complete ride (driver)
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&format!("/v1/rides/{ride_id}/complete"))
                .header("Authorization", format!("Bearer {driver_token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let complete_json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(complete_json["status"], "completed");
    assert!(complete_json["completed_at"].is_string());

    // 5. Rate ride (passenger rates driver)
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&format!("/v1/rides/{ride_id}/rate"))
                .header("Authorization", format!("Bearer {passenger_token}"))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({ "score": 5, "comment": "Ótimo motorista!" }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let rate_json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(rate_json["score"], 5);
}

#[tokio::test]
async fn test_cancel_ride_before_accept() {
    let (app, state) = create_test_app_with_state().await;
    let passenger_token = create_test_user_token(&state).await;

    // Create ride
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/rides")
                .header("Authorization", format!("Bearer {passenger_token}"))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "pickup_lat": -16.4489,
                        "pickup_lng": -39.0648,
                        "pickup_address": "Rua A",
                        "dropoff_lat": -16.4370,
                        "dropoff_lng": -39.0580,
                        "dropoff_address": "Rua B"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    let ride_id = json["ride"]["id"].as_str().unwrap();

    // Cancel
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&format!("/v1/rides/{ride_id}/cancel"))
                .header("Authorization", format!("Bearer {passenger_token}"))
                .header("Content-Type", "application/json")
                .body(Body::from(json!({ "reason": "changed_plans" }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "cancelled");
    assert_eq!(json["cancelled_by"], "passenger");
}

#[tokio::test]
async fn test_ride_history() {
    let (app, state) = create_test_app_with_state().await;
    let token = create_test_user_token(&state).await;

    // Create a ride first
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/rides")
                .header("Authorization", format!("Bearer {token}"))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "pickup_lat": -16.4489,
                        "pickup_lng": -39.0648,
                        "pickup_address": "Rua X",
                        "dropoff_lat": -16.4370,
                        "dropoff_lng": -39.0580,
                        "dropoff_address": "Rua Y"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Get history
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/rides/history?role=passenger")
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert!(json.as_array().unwrap().len() >= 1);
}

#[tokio::test]
async fn test_cannot_complete_non_active_ride() {
    let (app, state) = create_test_app_with_state().await;
    let (driver_token, _) = create_test_driver(&state).await;
    let passenger_token = create_test_user_token(&state).await;

    // Create ride
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/rides")
                .header("Authorization", format!("Bearer {passenger_token}"))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "pickup_lat": -16.45,
                        "pickup_lng": -39.06,
                        "pickup_address": "A",
                        "dropoff_lat": -16.44,
                        "dropoff_lng": -39.05,
                        "dropoff_address": "B"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    let ride_id = json["ride"]["id"].as_str().unwrap();

    // Accept
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&format!("/v1/rides/{ride_id}/accept"))
                .header("Authorization", format!("Bearer {driver_token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Try to complete without starting → should fail
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&format!("/v1/rides/{ride_id}/complete"))
                .header("Authorization", format!("Bearer {driver_token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_duplicate_rating_rejected() {
    let (app, state) = create_test_app_with_state().await;
    let passenger_token = create_test_user_token(&state).await;
    let (driver_token, _) = create_test_driver(&state).await;

    // Create → accept → start → complete
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/rides")
                .header("Authorization", format!("Bearer {passenger_token}"))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "pickup_lat": -16.45,
                        "pickup_lng": -39.06,
                        "pickup_address": "A",
                        "dropoff_lat": -16.44,
                        "dropoff_lng": -39.05,
                        "dropoff_address": "B"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    let ride_id = json["ride"]["id"].as_str().unwrap();

    // Accept
    let _ = app.clone().oneshot(
        Request::builder()
            .method("POST")
            .uri(&format!("/v1/rides/{ride_id}/accept"))
            .header("Authorization", format!("Bearer {driver_token}"))
            .body(Body::empty())
            .unwrap(),
    ).await.unwrap();

    // Start
    let _ = app.clone().oneshot(
        Request::builder()
            .method("POST")
            .uri(&format!("/v1/rides/{ride_id}/start"))
            .header("Authorization", format!("Bearer {driver_token}"))
            .body(Body::empty())
            .unwrap(),
    ).await.unwrap();

    // Complete
    let _ = app.clone().oneshot(
        Request::builder()
            .method("POST")
            .uri(&format!("/v1/rides/{ride_id}/complete"))
            .header("Authorization", format!("Bearer {driver_token}"))
            .body(Body::empty())
            .unwrap(),
    ).await.unwrap();

    // First rating
    let response = app.clone().oneshot(
        Request::builder()
            .method("POST")
            .uri(&format!("/v1/rides/{ride_id}/rate"))
            .header("Authorization", format!("Bearer {passenger_token}"))
            .header("Content-Type", "application/json")
            .body(Body::from(json!({ "score": 4 }).to_string()))
            .unwrap(),
    ).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // Duplicate rating → should fail
    let response = app.oneshot(
        Request::builder()
            .method("POST")
            .uri(&format!("/v1/rides/{ride_id}/rate"))
            .header("Authorization", format!("Bearer {passenger_token}"))
            .header("Content-Type", "application/json")
            .body(Body::from(json!({ "score": 3 }).to_string()))
            .unwrap(),
    ).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
