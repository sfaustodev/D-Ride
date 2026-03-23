use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;

use crate::auth::middleware::AuthUser;
use crate::error::AppError;
use crate::models::rating::{self, CreateRatingRequest, RatingResponse};
use crate::models::ride;
use crate::AppState;

pub async fn rate_ride(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(ride_id): Path<Uuid>,
    Json(body): Json<CreateRatingRequest>,
) -> Result<(StatusCode, Json<RatingResponse>), AppError> {
    if body.score < 1 || body.score > 5 {
        return Err(AppError::Validation("Score must be between 1 and 5".into()));
    }

    let db_ride = ride::find_by_id(&state.pool, ride_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Ride not found".into()))?;

    if db_ride.status != "completed" {
        return Err(AppError::Validation("Can only rate completed rides".into()));
    }

    // Determine who is being rated
    let (rater_id, rated_id) = if db_ride.passenger_id == auth.claims.sub {
        // Passenger rates driver
        let driver_id = db_ride.driver_id
            .ok_or_else(|| AppError::Validation("No driver assigned to this ride".into()))?;
        (auth.claims.sub, driver_id)
    } else if db_ride.driver_id == Some(auth.claims.sub) {
        // Driver rates passenger
        (auth.claims.sub, db_ride.passenger_id)
    } else {
        return Err(AppError::Unauthorized);
    };

    let db_rating = rating::create(
        &state.pool,
        ride_id,
        rater_id,
        rated_id,
        body.score,
        body.comment.as_deref(),
    )
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
            AppError::Validation("You already rated this ride".into())
        }
        other => AppError::Sqlx(other),
    })?;

    // Update cached rating on the rated user
    rating::update_user_rating(&state.pool, rated_id).await?;

    Ok((StatusCode::CREATED, Json(RatingResponse::from(db_rating))))
}
