use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub phone: String,
    pub name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub role: String,
    pub is_driver_active: bool,
    pub driver_vehicle: Option<JsonValue>,
    pub wallet_pubkey: String,
    pub rating_avg: f64,
    pub rating_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub role: String,
    pub is_driver_active: bool,
    pub driver_vehicle: Option<JsonValue>,
    pub wallet_pubkey: String,
    pub rating_avg: f64,
    pub rating_count: i32,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            name: u.name,
            phone: u.phone,
            email: u.email,
            avatar_url: u.avatar_url,
            role: u.role,
            is_driver_active: u.is_driver_active,
            driver_vehicle: u.driver_vehicle,
            wallet_pubkey: u.wallet_pubkey,
            rating_avg: u.rating_avg,
            rating_count: u.rating_count,
        }
    }
}

pub async fn find_by_id(pool: &sqlx::PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_phone(pool: &sqlx::PgPool, phone: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE phone = $1")
        .bind(phone)
        .fetch_optional(pool)
        .await
}

pub async fn create(
    pool: &sqlx::PgPool,
    phone: &str,
    wallet_pubkey: &str,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (phone, wallet_pubkey) VALUES ($1, $2) RETURNING *",
    )
    .bind(phone)
    .bind(wallet_pubkey)
    .fetch_one(pool)
    .await
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    pub driver_vehicle: Option<JsonValue>,
}

pub async fn update(
    pool: &sqlx::PgPool,
    id: Uuid,
    req: &UpdateUserRequest,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        UPDATE users SET
            name = COALESCE($2, name),
            email = COALESCE($3, email),
            role = COALESCE($4, role),
            driver_vehicle = COALESCE($5, driver_vehicle),
            updated_at = now()
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(&req.name)
    .bind(&req.email)
    .bind(&req.role)
    .bind(&req.driver_vehicle)
    .fetch_one(pool)
    .await
}

pub async fn soft_delete(pool: &sqlx::PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE users SET
            phone = CONCAT('del_', LEFT(id::text, 14)),
            name = 'Deleted User',
            email = NULL,
            avatar_url = NULL,
            driver_vehicle = NULL,
            updated_at = now()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}
