use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Wallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub pubkey: String,
    pub encrypted_sk: String,
    pub key_version: i32,
    pub balance_cached: i64,
    pub created_at: DateTime<Utc>,
}

pub async fn create(
    pool: &sqlx::PgPool,
    user_id: Uuid,
    pubkey: &str,
    encrypted_sk: &str,
) -> Result<Wallet, sqlx::Error> {
    sqlx::query_as::<_, Wallet>(
        "INSERT INTO wallets (user_id, pubkey, encrypted_sk) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(user_id)
    .bind(pubkey)
    .bind(encrypted_sk)
    .fetch_one(pool)
    .await
}
