use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct RideLocation {
    pub id: i64,
    pub ride_id: Uuid,
    pub user_id: Uuid,
    pub lat: rust_decimal::Decimal,
    pub lng: rust_decimal::Decimal,
    pub heading: Option<rust_decimal::Decimal>,
    pub speed_kmh: Option<rust_decimal::Decimal>,
    pub recorded_at: DateTime<Utc>,
}

pub async fn create(
    pool: &sqlx::PgPool,
    ride_id: Uuid,
    user_id: Uuid,
    lat: f64,
    lng: f64,
    heading: Option<f64>,
    speed_kmh: Option<f64>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO ride_locations (ride_id, user_id, lat, lng, heading, speed_kmh)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(ride_id)
    .bind(user_id)
    .bind(rust_decimal::Decimal::from_f64_retain(lat).unwrap_or_default())
    .bind(rust_decimal::Decimal::from_f64_retain(lng).unwrap_or_default())
    .bind(heading.and_then(rust_decimal::Decimal::from_f64_retain))
    .bind(speed_kmh.and_then(rust_decimal::Decimal::from_f64_retain))
    .execute(pool)
    .await?;
    Ok(())
}
