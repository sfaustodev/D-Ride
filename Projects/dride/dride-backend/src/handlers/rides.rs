use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth::middleware::AuthUser;
use crate::error::AppError;
use crate::models::ride::{self, CancelRequest, CreateRideRequest, DepositConfirmRequest, EstimateRequest, RideResponse, RideStatus};
use crate::models::ride_event;
use crate::services::pricing;
use crate::services::sol_rate;
use crate::ws::messages::*;
use crate::AppState;

// ── POST /rides/estimate ────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct EstimateResponse {
    pub distance_km: f64,
    pub duration_min: i32,
    pub fare_lamports: i64,
    pub fare_brl: f64,
    pub fare_sol: f64,
    pub protocol_fee_bps: i32,
    pub sol_brl_rate: f64,
}

pub async fn estimate(
    State(state): State<AppState>,
    _auth: AuthUser,
    Json(body): Json<EstimateRequest>,
) -> Result<Json<EstimateResponse>, AppError> {
    let distance_km = pricing::haversine_km(body.pickup_lat, body.pickup_lng, body.dropoff_lat, body.dropoff_lng);
    let duration_min = pricing::estimate_duration_min(distance_km);
    let sol_brl_rate = sol_rate::get_sol_brl_rate_safe(&state.redis, &state.config.coingecko_api_url).await;

    let fare = pricing::calculate_fare(distance_km, duration_min, sol_brl_rate);

    Ok(Json(EstimateResponse {
        distance_km: (fare.distance_km * 100.0).round() / 100.0,
        duration_min: fare.duration_min,
        fare_lamports: fare.fare_lamports,
        fare_brl: (fare.fare_brl * 100.0).round() / 100.0,
        fare_sol: (fare.fare_sol * 1_000_000.0).round() / 1_000_000.0,
        protocol_fee_bps: fare.protocol_fee_bps,
        sol_brl_rate,
    }))
}

// ── POST /rides ─────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct CreateRideResponse {
    pub ride: RideResponse,
    pub escrow_pubkey: String,
}

pub async fn create_ride(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(body): Json<CreateRideRequest>,
) -> Result<(StatusCode, Json<CreateRideResponse>), AppError> {
    let distance_km = pricing::haversine_km(body.pickup_lat, body.pickup_lng, body.dropoff_lat, body.dropoff_lng);
    let duration_min = pricing::estimate_duration_min(distance_km);
    let sol_brl_rate = sol_rate::get_sol_brl_rate_safe(&state.redis, &state.config.coingecko_api_url).await;
    let fare = pricing::calculate_fare(distance_km, duration_min, sol_brl_rate);

    // Derive escrow PDA address (placeholder — real PDA derivation needs program ID + ride UUID)
    let ride_uuid = Uuid::new_v4();
    let escrow_pubkey = format!("escrow_{}", ride_uuid.to_string().replace('-', ""));

    let db_ride = ride::create(
        &state.pool,
        auth.claims.sub,
        &body,
        fare.distance_km,
        fare.duration_min,
        fare.fare_lamports,
        fare.fare_brl,
        &escrow_pubkey,
    )
    .await
    .map_err(AppError::Sqlx)?;

    Ok((
        StatusCode::CREATED,
        Json(CreateRideResponse {
            escrow_pubkey: escrow_pubkey.clone(),
            ride: RideResponse::from(db_ride),
        }),
    ))
}

// ── POST /rides/:id/deposit-confirm ─────────────────────────────────

pub async fn deposit_confirm(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ride_id): Path<Uuid>,
    Json(body): Json<DepositConfirmRequest>,
) -> Result<Json<RideResponse>, AppError> {
    let db_ride = ride::find_by_id(&state.pool, ride_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Ride not found".into()))?;

    if db_ride.passenger_id != auth.claims.sub {
        return Err(AppError::Unauthorized);
    }

    let status = RideStatus::from_str(&db_ride.status)
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Invalid ride status: {}", db_ride.status)))?;
    status.validate_transition(&RideStatus::DepositPending)?;

    // TODO: verify tx_signature on-chain via Solana RPC

    let updated = ride::set_deposit_confirmed(&state.pool, ride_id, &body.tx_signature).await?;
    Ok(Json(RideResponse::from(updated)))
}

// ── GET /rides/available ────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AvailableQuery {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Serialize)]
pub struct AvailableRide {
    pub id: Uuid,
    pub pickup_address: String,
    pub dropoff_address: String,
    pub distance_km: f64,
    pub fare_brl: f64,
    pub fare_sol: f64,
    pub pickup_distance_km: f64,
}

pub async fn available(
    State(state): State<AppState>,
    _auth: AuthUser,
    Query(q): Query<AvailableQuery>,
) -> Result<Json<Vec<AvailableRide>>, AppError> {
    let radius_km = 5.0;
    let rides = ride::find_available(&state.pool, q.lat, q.lng, radius_km).await?;

    let sol_brl_rate = sol_rate::get_sol_brl_rate_safe(&state.redis, &state.config.coingecko_api_url).await;

    let results: Vec<AvailableRide> = rides
        .into_iter()
        .map(|r| {
            use rust_decimal::prelude::ToPrimitive;
            let pickup_lat = r.pickup_lat.to_f64().unwrap_or(0.0);
            let pickup_lng = r.pickup_lng.to_f64().unwrap_or(0.0);
            let distance_km = r.distance_km.to_f64().unwrap_or(0.0);
            let fare_brl = r.fare_brl.to_f64().unwrap_or(0.0);
            let fare_sol = fare_brl / sol_brl_rate;
            let pickup_distance = pricing::haversine_km(q.lat, q.lng, pickup_lat, pickup_lng);

            AvailableRide {
                id: r.id,
                pickup_address: r.pickup_address,
                dropoff_address: r.dropoff_address,
                distance_km: (distance_km * 100.0).round() / 100.0,
                fare_brl: (fare_brl * 100.0).round() / 100.0,
                fare_sol: (fare_sol * 1_000_000.0).round() / 1_000_000.0,
                pickup_distance_km: (pickup_distance * 100.0).round() / 100.0,
            }
        })
        .collect();

    Ok(Json(results))
}

// ── POST /rides/:id/accept ──────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct AcceptResponse {
    pub ride: RideResponse,
    pub passenger: PassengerInfo,
}

#[derive(Debug, Serialize)]
pub struct PassengerInfo {
    pub name: String,
    pub phone: String,
    pub rating_avg: f64,
}

pub async fn accept(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ride_id): Path<Uuid>,
) -> Result<Json<AcceptResponse>, AppError> {
    let db_ride = ride::find_by_id(&state.pool, ride_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Ride not found".into()))?;

    let status = RideStatus::from_str(&db_ride.status)
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Invalid ride status")))?;
    status.validate_transition(&RideStatus::Accepted)?;

    let updated = ride::set_driver(&state.pool, ride_id, auth.claims.sub).await?;

    let passenger = crate::models::user::find_by_id(&state.pool, db_ride.passenger_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Passenger not found".into()))?;

    // Log event
    let _ = ride_event::create(&state.pool, ride_id, "accepted", Some(auth.claims.sub), None).await;

    // Notify passenger via WebSocket
    let driver = crate::models::user::find_by_id(&state.pool, auth.claims.sub).await?.unwrap();
    let event = RideAcceptedEvent {
        ride_id,
        driver_name: driver.name,
        driver_rating: driver.rating_avg,
    };
    state.hub.send_to_user(&db_ride.passenger_id, &WsMessage::new("ride_accepted", &event)).await;

    Ok(Json(AcceptResponse {
        ride: RideResponse::from(updated),
        passenger: PassengerInfo {
            name: passenger.name,
            phone: passenger.phone,
            rating_avg: passenger.rating_avg,
        },
    }))
}

// ── POST /rides/:id/start ───────────────────────────────────────────

pub async fn start(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ride_id): Path<Uuid>,
) -> Result<Json<RideResponse>, AppError> {
    let db_ride = ride::find_by_id(&state.pool, ride_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Ride not found".into()))?;

    if db_ride.driver_id != Some(auth.claims.sub) {
        return Err(AppError::Unauthorized);
    }

    let status = RideStatus::from_str(&db_ride.status)
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Invalid ride status")))?;

    // accepted → active (skip deposit_pending for now, will be handled in flow)
    if status != RideStatus::Accepted {
        return Err(AppError::InvalidRideTransition {
            current: db_ride.status,
            action: "start".into(),
        });
    }

    let updated = ride::start(&state.pool, ride_id).await?;

    // Log event + notify passenger
    let _ = ride_event::create(&state.pool, ride_id, "started", Some(auth.claims.sub), None).await;
    let event = RideStartedEvent {
        ride_id,
        started_at: updated.started_at.map(|t| t.to_rfc3339()).unwrap_or_default(),
    };
    state.hub.send_to_user(&db_ride.passenger_id, &WsMessage::new("ride_started", &event)).await;

    Ok(Json(RideResponse::from(updated)))
}

// ── POST /rides/:id/complete ────────────────────────────────────────

pub async fn complete(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ride_id): Path<Uuid>,
) -> Result<Json<RideResponse>, AppError> {
    let db_ride = ride::find_by_id(&state.pool, ride_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Ride not found".into()))?;

    if db_ride.driver_id != Some(auth.claims.sub) {
        return Err(AppError::Unauthorized);
    }

    let status = RideStatus::from_str(&db_ride.status)
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Invalid ride status")))?;
    status.validate_transition(&RideStatus::Completing)?;

    // Mark as completing, then trigger escrow release
    ride::update_status(&state.pool, ride_id, "completing").await?;

    // TODO: trigger on-chain escrow release via Solana RPC
    let release_tx_sig = "placeholder_release_tx";

    let updated = ride::complete(&state.pool, ride_id, release_tx_sig).await?;

    // Log event + notify both parties
    let _ = ride_event::create(&state.pool, ride_id, "completed", Some(auth.claims.sub), None).await;
    let event = RideCompletedEvent {
        ride_id,
        release_tx_sig: Some(release_tx_sig.to_string()),
    };
    let msg = WsMessage::new("ride_completed", &event);
    state.hub.send_to_user(&db_ride.passenger_id, &msg).await;
    if let Some(driver_id) = db_ride.driver_id {
        state.hub.send_to_user(&driver_id, &msg).await;
    }

    Ok(Json(RideResponse::from(updated)))
}

// ── POST /rides/:id/cancel ──────────────────────────────────────────

pub async fn cancel_ride(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ride_id): Path<Uuid>,
    Json(_body): Json<CancelRequest>,
) -> Result<Json<RideResponse>, AppError> {
    let db_ride = ride::find_by_id(&state.pool, ride_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Ride not found".into()))?;

    // Either passenger or driver can cancel
    let is_passenger = db_ride.passenger_id == auth.claims.sub;
    let is_driver = db_ride.driver_id == Some(auth.claims.sub);
    if !is_passenger && !is_driver {
        return Err(AppError::Unauthorized);
    }

    let status = RideStatus::from_str(&db_ride.status)
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Invalid ride status")))?;
    status.validate_transition(&RideStatus::Cancelled)?;

    let cancelled_by = if is_passenger { "passenger" } else { "driver" };

    // TODO: trigger on-chain escrow refund if deposit exists

    let updated = ride::cancel(&state.pool, ride_id, cancelled_by).await?;

    // Log event + notify both parties
    let _ = ride_event::create(&state.pool, ride_id, "cancelled", Some(auth.claims.sub), None).await;
    let event = RideCancelledEvent {
        ride_id,
        cancelled_by: cancelled_by.to_string(),
    };
    let msg = WsMessage::new("ride_cancelled", &event);
    state.hub.send_to_user(&db_ride.passenger_id, &msg).await;
    if let Some(driver_id) = db_ride.driver_id {
        state.hub.send_to_user(&driver_id, &msg).await;
    }

    Ok(Json(RideResponse::from(updated)))
}

// ── GET /rides/:id ──────────────────────────────────────────────────

pub async fn get_ride(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ride_id): Path<Uuid>,
) -> Result<Json<RideResponse>, AppError> {
    let db_ride = ride::find_by_id(&state.pool, ride_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Ride not found".into()))?;

    // Only participant can view
    if db_ride.passenger_id != auth.claims.sub && db_ride.driver_id != Some(auth.claims.sub) {
        return Err(AppError::Unauthorized);
    }

    Ok(Json(RideResponse::from(db_ride)))
}

// ── GET /rides/history ──────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct HistoryQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub role: Option<String>,
}

pub async fn history(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(q): Query<HistoryQuery>,
) -> Result<Json<Vec<RideResponse>>, AppError> {
    let page = q.page.unwrap_or(1).max(1);
    let per_page = q.per_page.unwrap_or(20).min(50);
    let role = q.role.as_deref().unwrap_or("passenger");

    let rides = ride::history(&state.pool, auth.claims.sub, role, page, per_page).await?;
    let responses: Vec<RideResponse> = rides.into_iter().map(RideResponse::from).collect();
    Ok(Json(responses))
}
