use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::error::AppError;

// ── Ride Status Machine ─────────────────────────────────────────────
// requested → accepted → deposit_pending → active → completing → completed
//                ↘ cancelled                     ↘ disputed
// requested → expired (no driver within 5 min)

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RideStatus {
    Requested,
    Accepted,
    DepositPending,
    Active,
    Completing,
    Completed,
    Cancelled,
    Expired,
    Disputed,
}

impl RideStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::Accepted => "accepted",
            Self::DepositPending => "deposit_pending",
            Self::Active => "active",
            Self::Completing => "completing",
            Self::Completed => "completed",
            Self::Cancelled => "cancelled",
            Self::Expired => "expired",
            Self::Disputed => "disputed",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "requested" => Some(Self::Requested),
            "accepted" => Some(Self::Accepted),
            "deposit_pending" => Some(Self::DepositPending),
            "active" => Some(Self::Active),
            "completing" => Some(Self::Completing),
            "completed" => Some(Self::Completed),
            "cancelled" => Some(Self::Cancelled),
            "expired" => Some(Self::Expired),
            "disputed" => Some(Self::Disputed),
            _ => None,
        }
    }

    pub fn can_transition_to(&self, target: &RideStatus) -> bool {
        matches!(
            (self, target),
            (Self::Requested, Self::Accepted)
                | (Self::Requested, Self::Cancelled)
                | (Self::Requested, Self::Expired)
                | (Self::Accepted, Self::DepositPending)
                | (Self::Accepted, Self::Cancelled)
                | (Self::DepositPending, Self::Requested) // deposit confirmed → visible to drivers
                | (Self::DepositPending, Self::Cancelled)
                | (Self::Active, Self::Completing)
                | (Self::Active, Self::Cancelled)
                | (Self::Active, Self::Disputed)
                | (Self::Completing, Self::Completed)
        )
    }

    pub fn validate_transition(&self, target: &RideStatus) -> Result<(), AppError> {
        if self.can_transition_to(target) {
            Ok(())
        } else {
            Err(AppError::InvalidRideTransition {
                current: self.as_str().to_string(),
                action: target.as_str().to_string(),
            })
        }
    }
}

// ── Database Model ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Ride {
    pub id: Uuid,
    pub passenger_id: Uuid,
    pub driver_id: Option<Uuid>,
    pub status: String,
    pub pickup_lat: rust_decimal::Decimal,
    pub pickup_lng: rust_decimal::Decimal,
    pub pickup_address: String,
    pub dropoff_lat: rust_decimal::Decimal,
    pub dropoff_lng: rust_decimal::Decimal,
    pub dropoff_address: String,
    pub distance_km: rust_decimal::Decimal,
    pub duration_min: i32,
    pub fare_lamports: i64,
    pub fare_brl: rust_decimal::Decimal,
    pub protocol_fee_bps: i32,
    pub escrow_pubkey: Option<String>,
    pub escrow_tx_sig: Option<String>,
    pub release_tx_sig: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub cancelled_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct RideResponse {
    pub id: Uuid,
    pub passenger_id: Uuid,
    pub driver_id: Option<Uuid>,
    pub status: String,
    pub pickup_lat: f64,
    pub pickup_lng: f64,
    pub pickup_address: String,
    pub dropoff_lat: f64,
    pub dropoff_lng: f64,
    pub dropoff_address: String,
    pub distance_km: f64,
    pub duration_min: i32,
    pub fare_lamports: i64,
    pub fare_brl: f64,
    pub protocol_fee_bps: i32,
    pub escrow_pubkey: Option<String>,
    pub escrow_tx_sig: Option<String>,
    pub release_tx_sig: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub cancelled_by: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<Ride> for RideResponse {
    fn from(r: Ride) -> Self {
        use rust_decimal::prelude::ToPrimitive;
        Self {
            id: r.id,
            passenger_id: r.passenger_id,
            driver_id: r.driver_id,
            status: r.status,
            pickup_lat: r.pickup_lat.to_f64().unwrap_or(0.0),
            pickup_lng: r.pickup_lng.to_f64().unwrap_or(0.0),
            pickup_address: r.pickup_address,
            dropoff_lat: r.dropoff_lat.to_f64().unwrap_or(0.0),
            dropoff_lng: r.dropoff_lng.to_f64().unwrap_or(0.0),
            dropoff_address: r.dropoff_address,
            distance_km: r.distance_km.to_f64().unwrap_or(0.0),
            duration_min: r.duration_min,
            fare_lamports: r.fare_lamports,
            fare_brl: r.fare_brl.to_f64().unwrap_or(0.0),
            protocol_fee_bps: r.protocol_fee_bps,
            escrow_pubkey: r.escrow_pubkey,
            escrow_tx_sig: r.escrow_tx_sig,
            release_tx_sig: r.release_tx_sig,
            started_at: r.started_at,
            completed_at: r.completed_at,
            cancelled_at: r.cancelled_at,
            cancelled_by: r.cancelled_by,
            created_at: r.created_at,
        }
    }
}

// ── Request types ───────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreateRideRequest {
    pub pickup_lat: f64,
    pub pickup_lng: f64,
    pub pickup_address: String,
    pub dropoff_lat: f64,
    pub dropoff_lng: f64,
    pub dropoff_address: String,
}

#[derive(Debug, Deserialize)]
pub struct EstimateRequest {
    pub pickup_lat: f64,
    pub pickup_lng: f64,
    pub dropoff_lat: f64,
    pub dropoff_lng: f64,
}

#[derive(Debug, Deserialize)]
pub struct DepositConfirmRequest {
    pub tx_signature: String,
}

#[derive(Debug, Deserialize)]
pub struct CancelRequest {
    pub reason: Option<String>,
}

// ── Queries ─────────────────────────────────────────────────────────

pub async fn find_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Option<Ride>, sqlx::Error> {
    sqlx::query_as::<_, Ride>("SELECT * FROM rides WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn create(
    pool: &sqlx::PgPool,
    passenger_id: Uuid,
    req: &CreateRideRequest,
    distance_km: f64,
    duration_min: i32,
    fare_lamports: i64,
    fare_brl: f64,
    escrow_pubkey: &str,
) -> Result<Ride, sqlx::Error> {
    sqlx::query_as::<_, Ride>(
        r#"
        INSERT INTO rides (
            passenger_id, pickup_lat, pickup_lng, pickup_address,
            dropoff_lat, dropoff_lng, dropoff_address,
            distance_km, duration_min, fare_lamports, fare_brl,
            escrow_pubkey
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING *
        "#,
    )
    .bind(passenger_id)
    .bind(rust_decimal::Decimal::from_f64_retain(req.pickup_lat).unwrap_or_default())
    .bind(rust_decimal::Decimal::from_f64_retain(req.pickup_lng).unwrap_or_default())
    .bind(&req.pickup_address)
    .bind(rust_decimal::Decimal::from_f64_retain(req.dropoff_lat).unwrap_or_default())
    .bind(rust_decimal::Decimal::from_f64_retain(req.dropoff_lng).unwrap_or_default())
    .bind(&req.dropoff_address)
    .bind(rust_decimal::Decimal::from_f64_retain(distance_km).unwrap_or_default())
    .bind(duration_min)
    .bind(fare_lamports)
    .bind(rust_decimal::Decimal::from_f64_retain(fare_brl).unwrap_or_default())
    .bind(escrow_pubkey)
    .fetch_one(pool)
    .await
}

pub async fn update_status(
    pool: &sqlx::PgPool,
    id: Uuid,
    status: &str,
) -> Result<Ride, sqlx::Error> {
    sqlx::query_as::<_, Ride>(
        "UPDATE rides SET status = $2, updated_at = now() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .bind(status)
    .fetch_one(pool)
    .await
}

pub async fn set_driver(
    pool: &sqlx::PgPool,
    id: Uuid,
    driver_id: Uuid,
) -> Result<Ride, sqlx::Error> {
    sqlx::query_as::<_, Ride>(
        r#"
        UPDATE rides SET driver_id = $2, status = 'accepted', updated_at = now()
        WHERE id = $1 RETURNING *
        "#,
    )
    .bind(id)
    .bind(driver_id)
    .fetch_one(pool)
    .await
}

pub async fn set_deposit_confirmed(
    pool: &sqlx::PgPool,
    id: Uuid,
    tx_sig: &str,
) -> Result<Ride, sqlx::Error> {
    sqlx::query_as::<_, Ride>(
        r#"
        UPDATE rides SET escrow_tx_sig = $2, status = 'requested', updated_at = now()
        WHERE id = $1 RETURNING *
        "#,
    )
    .bind(id)
    .bind(tx_sig)
    .fetch_one(pool)
    .await
}

pub async fn start(pool: &sqlx::PgPool, id: Uuid) -> Result<Ride, sqlx::Error> {
    sqlx::query_as::<_, Ride>(
        r#"
        UPDATE rides SET status = 'active', started_at = now(), updated_at = now()
        WHERE id = $1 RETURNING *
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn complete(
    pool: &sqlx::PgPool,
    id: Uuid,
    release_tx_sig: &str,
) -> Result<Ride, sqlx::Error> {
    sqlx::query_as::<_, Ride>(
        r#"
        UPDATE rides SET status = 'completed', release_tx_sig = $2,
            completed_at = now(), updated_at = now()
        WHERE id = $1 RETURNING *
        "#,
    )
    .bind(id)
    .bind(release_tx_sig)
    .fetch_one(pool)
    .await
}

pub async fn cancel(
    pool: &sqlx::PgPool,
    id: Uuid,
    cancelled_by: &str,
) -> Result<Ride, sqlx::Error> {
    sqlx::query_as::<_, Ride>(
        r#"
        UPDATE rides SET status = 'cancelled', cancelled_by = $2,
            cancelled_at = now(), updated_at = now()
        WHERE id = $1 RETURNING *
        "#,
    )
    .bind(id)
    .bind(cancelled_by)
    .fetch_one(pool)
    .await
}

/// Find available rides within radius_km of driver's location
pub async fn find_available(
    pool: &sqlx::PgPool,
    driver_lat: f64,
    driver_lng: f64,
    radius_km: f64,
) -> Result<Vec<Ride>, sqlx::Error> {
    sqlx::query_as::<_, Ride>(
        r#"
        SELECT * FROM rides
        WHERE status = 'requested'
          AND escrow_tx_sig IS NOT NULL
          AND earth_distance(
                ll_to_earth($1::float8, $2::float8),
                ll_to_earth(pickup_lat::float8, pickup_lng::float8)
              ) / 1000.0 <= $3
        ORDER BY created_at ASC
        "#,
    )
    .bind(driver_lat)
    .bind(driver_lng)
    .bind(radius_km)
    .fetch_all(pool)
    .await
}

pub async fn history(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    role: &str,
    page: i64,
    per_page: i64,
) -> Result<Vec<Ride>, sqlx::Error> {
    let offset = (page - 1) * per_page;
    if role == "driver" {
        sqlx::query_as::<_, Ride>(
            "SELECT * FROM rides WHERE driver_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_as::<_, Ride>(
            "SELECT * FROM rides WHERE passenger_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool)
        .await
    }
}
