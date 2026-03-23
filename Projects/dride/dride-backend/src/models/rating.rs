use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Rating {
    pub id: Uuid,
    pub ride_id: Uuid,
    pub rater_id: Uuid,
    pub rated_id: Uuid,
    pub score: i16,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct RatingResponse {
    pub id: Uuid,
    pub score: i16,
    pub comment: Option<String>,
}

impl From<Rating> for RatingResponse {
    fn from(r: Rating) -> Self {
        Self {
            id: r.id,
            score: r.score,
            comment: r.comment,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateRatingRequest {
    pub score: i16,
    pub comment: Option<String>,
}

pub async fn create(
    pool: &sqlx::PgPool,
    ride_id: Uuid,
    rater_id: Uuid,
    rated_id: Uuid,
    score: i16,
    comment: Option<&str>,
) -> Result<Rating, sqlx::Error> {
    sqlx::query_as::<_, Rating>(
        r#"
        INSERT INTO ratings (ride_id, rater_id, rated_id, score, comment)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
    )
    .bind(ride_id)
    .bind(rater_id)
    .bind(rated_id)
    .bind(score)
    .bind(comment)
    .fetch_one(pool)
    .await
}

/// Recalculate and update user's cached rating
pub async fn update_user_rating(pool: &sqlx::PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE users SET
            rating_avg = COALESCE((SELECT AVG(score)::float8 FROM ratings WHERE rated_id = $1), 5.0),
            rating_count = (SELECT COUNT(*) FROM ratings WHERE rated_id = $1),
            updated_at = now()
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}
