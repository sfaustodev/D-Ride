use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Envelope for all WebSocket messages
#[derive(Debug, Serialize, Deserialize)]
pub struct WsMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub payload: serde_json::Value,
}

// ── Client → Server ─────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct LocationUpdate {
    pub lat: f64,
    pub lng: f64,
    pub heading: Option<f64>,
    pub speed_kmh: Option<f64>,
}

// ── Server → Client ─────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct RideRequestedEvent {
    pub ride_id: Uuid,
    pub pickup_address: String,
    pub dropoff_address: String,
    pub distance_km: f64,
    pub fare_brl: f64,
    pub pickup_distance_km: f64,
}

#[derive(Debug, Serialize)]
pub struct RideAcceptedEvent {
    pub ride_id: Uuid,
    pub driver_name: String,
    pub driver_rating: f64,
}

#[derive(Debug, Serialize)]
pub struct RideCancelledEvent {
    pub ride_id: Uuid,
    pub cancelled_by: String,
}

#[derive(Debug, Serialize)]
pub struct RideStartedEvent {
    pub ride_id: Uuid,
    pub started_at: String,
}

#[derive(Debug, Serialize)]
pub struct RideCompletedEvent {
    pub ride_id: Uuid,
    pub release_tx_sig: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DriverLocationEvent {
    pub ride_id: Uuid,
    pub lat: f64,
    pub lng: f64,
    pub heading: Option<f64>,
    pub eta_min: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct DepositConfirmedEvent {
    pub ride_id: Uuid,
    pub escrow_pubkey: String,
}

#[derive(Debug, Serialize)]
pub struct EscrowReleasedEvent {
    pub ride_id: Uuid,
    pub tx_sig: String,
    pub driver_amount: u64,
}

impl WsMessage {
    pub fn new<T: Serialize>(msg_type: &str, payload: &T) -> Self {
        Self {
            msg_type: msg_type.to_string(),
            payload: serde_json::to_value(payload).unwrap_or_default(),
        }
    }

    pub fn to_text(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}
