use axum::extract::ws::{Message, WebSocket};
use axum::extract::{Query, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::auth::jwt::verify_token;
use crate::models::ride_location;
use crate::AppState;

use super::messages::{DriverLocationEvent, LocationUpdate, WsMessage};

#[derive(Debug, Deserialize)]
pub struct WsQuery {
    pub token: String,
}

/// WebSocket upgrade handler: wss://api.dride.app/v1/ws?token=JWT
pub async fn ws_upgrade(
    State(state): State<AppState>,
    Query(query): Query<WsQuery>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    // Validate JWT before upgrading
    let claims = match verify_token(&query.token, &state.config.jwt_secret) {
        Ok(c) => c,
        Err(_) => {
            return axum::http::Response::builder()
                .status(401)
                .body(axum::body::Body::from("Unauthorized"))
                .unwrap()
                .into_response();
        }
    };

    ws.on_upgrade(move |socket| handle_socket(socket, claims.sub, state))
        .into_response()
}

async fn handle_socket(socket: WebSocket, user_id: Uuid, state: AppState) {
    let (mut ws_sink, mut ws_stream) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    // Register in hub
    state.hub.register(user_id, tx).await;

    // Spawn task to forward hub messages to WebSocket
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sink.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Process incoming messages
    while let Some(Ok(msg)) = ws_stream.next().await {
        match msg {
            Message::Text(text) => {
                handle_client_message(&state, &user_id, &text).await;
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    // Cleanup
    state.hub.unregister(&user_id).await;
    send_task.abort();
}

async fn handle_client_message(state: &AppState, user_id: &Uuid, text: &str) {
    let ws_msg: WsMessage = match serde_json::from_str(text) {
        Ok(m) => m,
        Err(_) => return,
    };

    match ws_msg.msg_type.as_str() {
        "location_update" => {
            let update: LocationUpdate = match serde_json::from_value(ws_msg.payload) {
                Ok(u) => u,
                Err(_) => return,
            };

            // Update hub location for geo-broadcast
            state.hub.update_location(user_id, update.lat, update.lng).await;

            // Find active ride for this driver
            if let Ok(Some(ride)) = find_active_ride_for_driver(&state.pool, user_id).await {
                // Store location in DB
                let _ = ride_location::create(
                    &state.pool,
                    ride.id,
                    *user_id,
                    update.lat,
                    update.lng,
                    update.heading,
                    update.speed_kmh,
                )
                .await;

                // Broadcast to passenger
                let event = DriverLocationEvent {
                    ride_id: ride.id,
                    lat: update.lat,
                    lng: update.lng,
                    heading: update.heading,
                    eta_min: None,
                };
                let msg = WsMessage::new("driver_location", &event);
                state.hub.send_to_user(&ride.passenger_id, &msg).await;
            }
        }
        "ping" => {}
        _ => {
            tracing::warn!("Unknown WS message type: {}", ws_msg.msg_type);
        }
    }
}

async fn find_active_ride_for_driver(
    pool: &sqlx::PgPool,
    driver_id: &Uuid,
) -> Result<Option<crate::models::ride::Ride>, sqlx::Error> {
    sqlx::query_as::<_, crate::models::ride::Ride>(
        "SELECT * FROM rides WHERE driver_id = $1 AND status = 'active' LIMIT 1",
    )
    .bind(driver_id)
    .fetch_optional(pool)
    .await
}
