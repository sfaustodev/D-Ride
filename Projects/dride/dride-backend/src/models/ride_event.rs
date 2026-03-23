use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct RideEvent {
    pub id: Uuid,
    pub ride_id: Uuid,
    pub event_type: String,
    pub actor_id: Option<Uuid>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

pub async fn create(
    pool: &sqlx::PgPool,
    ride_id: Uuid,
    event_type: &str,
    actor_id: Option<Uuid>,
    metadata: Option<serde_json::Value>,
) -> Result<RideEvent, sqlx::Error> {
    sqlx::query_as::<_, RideEvent>(
        r#"
        INSERT INTO ride_events (ride_id, event_type, actor_id, metadata)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(ride_id)
    .bind(event_type)
    .bind(actor_id)
    .bind(metadata)
    .fetch_one(pool)
    .await
}
